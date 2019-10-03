use ord_subset::OrdSubsetIterExt;
use ord_subset::OrdSubsetSliceExt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

pub fn is_numeric() -> bool {
    true
}

///
fn quantize<'v>(column: &'v [(&str, &str)], small: usize) -> HashMap<&'v str, String> {
    // 1. Get the attribute values in sorted order:
    let mut sorted: Vec<(f32, &str)> = Vec::new();
    for (v, c) in column {
        if let Ok(n) = v.parse::<f32>() {
            sorted.push((n, c));
        } else {
            unimplemented!(
                "Cannot yet quantize non-numeric values: https://github.com/d6y/oner/issues/1"
            );
        }
    }
    sorted.ord_subset_sort_by_key(|pair| pair.0);

    // 2. Create a split each time the classification changes

    let mut split_index = Vec::new(); // Index into `sorted` where the classification changes to a different value.
    for (prev_index, ((_cur_value, cur_class), (_prev_val, prev_class))) in
        sorted.iter().skip(1).zip(sorted.iter()).enumerate()
    {
        if cur_class != prev_class {
            split_index.push(prev_index + 1);
        }
    }

    // 3. Remove splits that are too small:
    let split_trimmed = trim_splits(split_index, small, &sorted);
    dbg!(&split_trimmed);

    // 4. Generate distinct intervals from the spits:
    let intervals: Vec<Interval<f32, &str>> = Interval::from_splits(split_trimmed, &sorted);
    dbg!(&intervals);

    let merged_intervals = Interval::merge_neighbours_with_same_class(&intervals);
    dbg!(&merged_intervals);

    // Generate a re-mapping table from each value we've seen to the new value:

    HashMap::new()
}

fn trim_splits(splits: Vec<usize>, small: usize, data: &[(f32, &str)]) -> Vec<usize> {
    // Tail-recursive safe walk of the splits:
    trim_splits0(splits.as_slice(), small, data, Vec::new(), 0)
}

fn trim_splits0(
    splits: &[usize],
    small: usize,
    data: &[(f32, &str)],
    mut keep: Vec<usize>,
    start_index: usize,
) -> Vec<usize> {
    if let Some(head) = splits.first() {
        let tail = &splits[1..];
        if no_dominant_class(start_index, *head, small, data) {
            // Drop this split:
            trim_splits0(tail, small, data, keep, start_index)
        } else {
            // Keep the split, and carry on from this point (`head`):
            keep.push(*head);
            trim_splits0(tail, small, data, keep, *head)
        }
    } else {
        // No more elements to process
        keep
    }
}

fn no_dominant_class(start: usize, until: usize, small: usize, data: &[(f32, &str)]) -> bool {
    let classes: Vec<&str> = data[start..until].iter().map(|pair| pair.1).collect();
    let counts = frequency_count(&classes);
    counts.values().all(|&count| count <= small)
}

fn frequency_count<T>(ts: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for t in ts {
        let count = counts.entry(t).or_insert(0);
        *count += 1;
    }
    counts
}

#[derive(Debug, Clone, Copy)]
enum Interval<T, C> {
    Lower { below: T, class: C },          // e.g., < 100
    Range { from: T, below: T, class: C }, // e.g., >= 100 and < 200
    Upper { from: T, class: C },           // e.g., >= 200
    Infinite { class: C },
}

impl<T: Copy + Debug + Display, C: Copy + Debug> Interval<T, C> {
    fn show(&self) -> String {
        match self {
            Interval::Lower { below, .. } => format!("< {}", below),
            Interval::Range { from, below, .. } => format!(">= {} and < {}", from, below),
            Interval::Upper { from, .. } => format!(">= {}", from),
            Interval::Infinite { .. } => String::from("any value"),
        }
    }

    fn class(&self) -> &C {
        match self {
            Interval::Lower { class, .. } => class,
            Interval::Range { class, .. } => class,
            Interval::Upper { class, .. } => class,
            Interval::Infinite { class } => class,
        }
    }

    fn merge(&self, later: &Self) -> Self {
        match (self, later) {
            (Interval::Lower { .. }, Interval::Range { below, class, .. }) => Interval::Lower {
                below: *below,
                class: *class,
            },
            (Interval::Lower { .. }, Interval::Upper { class, .. }) => {
                Interval::Infinite { class: *class }
            }
            (Interval::Range { from, .. }, Interval::Range { below, class, .. }) => {
                Interval::Range {
                    from: *from,
                    below: *below,
                    class: *class,
                }
            }
            (Interval::Range { from, .. }, Interval::Upper { class, .. }) => Interval::Upper {
                from: *from,
                class: *class,
            },
            _ => panic!("Merging {:?} with {:?} is not supported", self, later),
        }
    }
}

impl<T, C> Interval<T, C>
where
    T: Copy + Debug + Display,
    C: Copy + Debug + Eq + Hash,
{
    // `splits` is a list of indices where we want to break the values into intervals.
    // The values are the (value, class) pairs in `data`, and the `splits` contents are indicies are into `data`.
    // The first split is "anything below this value", and the last is "anything of this value and above".
    // Anything else is a range interval.
    // If there are no splits, then there's a single interval covering all values.
    fn from_splits(splits: Vec<usize>, data: &[(T, C)]) -> Vec<Interval<T, C>> {
        // What do do about ties for most frequent class? https://github.com/d6y/oner/issues/3#issuecomment-537864969
        let most_frequent_class = |start: usize, until: usize| {
            let classes: Vec<C> = data[start..until].iter().map(|pair| pair.1).collect();
            let largest: Option<&C> = frequency_count(&classes)
                .into_iter()
                .ord_subset_max_by_key(|pair| pair.1)
                .map(|pair| pair.0);

            *largest.unwrap_or_else(|| panic!("Found no classes for a split during quantization. Range is {} until {} in splits {:?} for data {:?}", start, until, &splits, data))
        };

        let lower = |index: usize| Interval::Lower {
            below: data[index].0,
            class: most_frequent_class(0, index),
        };

        let upper = |index: usize| Interval::Upper {
            from: data[index].0,
            class: most_frequent_class(index, data.len()),
        };

        let range = |index_start: usize, index_end: usize| Interval::Range {
            from: data[index_start].0,
            below: data[index_end].0,
            class: most_frequent_class(index_start, index_end),
        };

        let infinite = || Interval::Infinite {
            class: most_frequent_class(0, data.len()),
        };

        match splits.len() {
            0 => vec![infinite()],
            1 => vec![lower(splits[0]), upper(splits[0])],
            n => {
                let mut intervals = Vec::with_capacity(n + 1);
                intervals.push(lower(splits[0]));
                for (&curr_i, &prev_i) in splits.iter().skip(1).take(n - 1).zip(splits.iter()) {
                    intervals.push(range(prev_i, curr_i));
                }
                intervals.push(upper(splits[n - 1]));
                intervals
            }
        }
    }

    fn merge_neighbours_with_same_class(intervals: &[Interval<T, C>]) -> Vec<Interval<T, C>> {
        let mut merged: Vec<Interval<T, C>> = Vec::new();

        if let Some(head) = intervals.first() {
            let mut last_class = head.class();
            merged.push(*head);

            let tail = &intervals[1..];
            for interval in tail {
                let class = interval.class();
                if class == last_class {
                    let updated = merged.pop().map(|last| last.merge(interval));
                    updated.into_iter().for_each(|i| merged.push(i));
                } else {
                    last_class = class;
                    merged.push(*interval);
                }
            }
        }

        merged
    }
}

#[cfg(test)]
mod test_quantize {
    use super::quantize;
    use std::collections::HashMap;
    #[test]
    fn test_golf_example() {
        // This example (inputs, and boundary points) comes from:
        // Nevill-Manning, Holmes & Witten (1995)  _The Development of Holte's 1R Classifier_, p. 2

        let inputs = [
            ("64", "P"),
            ("65", "D"),
            ("68", "P"),
            ("69", "P"),
            ("70", "P"),
            ("71", "D"),
            ("72", "P"),
            ("72", "D"),
            ("75", "P"),
            ("75", "P"),
            ("80", "D"),
            ("81", "P"),
            ("83", "P"),
            ("85", "D"),
        ];

        let i1 = "< 71";
        let i2 = ">= 85";

        let expected: HashMap<&str, String> = [
            ("64", i1),
            ("65", i1),
            ("68", i1),
            ("69", i1),
            ("70", i1),
            ("71", i1),
            ("72", i1),
            ("75", i1),
            ("75", i1),
            ("80", i1),
            ("81", i1),
            ("83", i1),
            ("85", i2),
        ]
        .iter()
        .map(|(v, s)| (*v, s.to_string()))
        .collect();
        assert_eq!(expected, quantize(&inputs, 3));
    }
}
