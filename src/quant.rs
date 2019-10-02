use std::collections::HashMap;
use ord_subset::OrdSubsetSliceExt;

pub fn is_numeric() -> bool {
    true
}

// fn quantize<'v, C : std::fmt::Display + Eq>(column: &'v [(&str, &str)], small: usize) -> HashMap<&'v str, String> {
fn quantize<'v>(column: &'v [(&str, &str)], small: usize) -> HashMap<&'v str, String> {
    // 1. Get the attribute values in sorted order:
    let mut sorted: Vec<(f32, &str)> = Vec::new();
    for (v, c) in column {
        if let Ok(n) = v.parse::<f32>() {
            sorted.push( (n, c) );
        } else {
            // TODO: handle unparsable input
            unimplemented!("not yet handling non-numeric input e.g., missing values");
        }
    }
    sorted.ord_subset_sort_by_key(|pair| pair.0);

    // 2. Create a split each time the classification changes
    // providing there are at least `small` of some class in the split.
    // (This does not apply to the last split)

    // Index into `sorted` where the classification changes to a different value:
    let mut split_index = Vec::new();
    for (prev_index, ( (_cur_value, cur_class), (_prev_val, prev_class) ) ) in sorted.iter().skip(1).zip(sorted.iter()).enumerate() {
        if cur_class != prev_class {
            split_index.push(prev_index + 1);
        }
    }

    // 3. Remove splits that are too small:
    let mut split_trimmed = trim_splits(split_index, small, &sorted);
    dbg!(&split_trimmed);



    // Merge any neighbouring splits with the same classification:

    // Generate a re-mapping table from each value we've seen to the new value:

   HashMap::new()
}

fn trim_splits(splits: Vec<usize>, small: usize, data: &Vec<(f32, &str)>) -> Vec<usize> {
    let mut keeps = Vec::new();
    trim_splits0(splits.as_slice(), small, data, keeps, 0)
}

fn trim_splits0(splits: &[usize], small: usize, data: &Vec<(f32, &str)>, mut keep: Vec<usize>, start_index: usize) -> Vec<usize> {

    if let Some(head) = splits.first() {
        let size = head - start_index;
        let tail = &splits[1..];
        if size < small || no_dominant_class(start_index, *head, data) {
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

fn no_dominant_class(start: usize, until: usize, data: &Vec<(f32, &str)>) -> bool {
    false // TODO: implement
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

        let i1 = ">= 64 and < 71"; // TODO: Could be just "< 71"?
        let i2 = ">= 71 and < 85";
        let i3 = ">= 85";

        let expected: HashMap<&str, String> = [
            ("64", i1),
            ("65", i1),
            ("68", i1),
            ("69", i1),
            ("70", i1),
            ("71", i2),
            ("72", i2),
            ("75", i2),
            ("75", i2),
            ("80", i2),
            ("81", i2),
            ("83", i2),
            ("85", i3),
        ].iter().map(|(v,s)| (*v, s.to_string())).collect();
        assert_eq!(expected, quantize(&inputs, 3));
    }
}
