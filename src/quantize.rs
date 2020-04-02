use super::dataset::Dataset;
use ndarray::{Array1, Axis};
use oner_quantize::{find_intervals, quantize, Interval};
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

pub fn auto_quantize(
    dataset: Dataset,
    min_distinct_values: usize,
    small: usize,
    missing_value: &str,
) -> Dataset {
    let row_axis = Axis(0);
    let column_axis = Axis(1);

    let num_examples = dataset.attributes.len_of(row_axis);
    let mut quantized_dataset = Dataset::new(num_examples, &dataset.attribute_names);
    quantized_dataset.classes = dataset.classes.clone();

    let all_either_numeric_or_missing = |xs: &[String]| -> bool {
        xs.iter().all(|x| x.is_empty() || *x == missing_value || f32::from_str(x).is_ok())
    };

    let is_numeric = |values: &[String]| -> bool {
        all_either_numeric_or_missing(values) && count_distinct(values) >= min_distinct_values
    };

    for (idx, array1) in dataset.attributes.axis_iter(column_axis).enumerate() {
        let col = array1.to_vec();

        if !is_numeric(&col) {
            // No change to the attribute
            quantized_dataset.attributes.column_mut(idx).assign(&dataset.attributes.column(idx));
        } else {
            // 1. Select numeric (non-blank, non-missing) values for quantization:
            let mut selected_classes = Vec::new();
            let mut selected_attribute_values = Vec::new();
            for (value, class) in col.iter().zip(dataset.classes.iter()) {
                if let Ok(numeric_value) = f32::from_str(value) {
                    selected_attribute_values.push(numeric_value);
                    selected_classes.push(class);
                }
            }

            // 2. Detect intervals from the selected rows:
            let intervals = find_intervals(&selected_attribute_values, &selected_classes, small);

            // 3. Convert attribute values into discrete string value:
            let convert = |orig_str: &String| -> String {
                // NB: efficiency - this is the 3rd time we're parsing this string
                let maybe_interval =
                    f32::from_str(&orig_str).ok().and_then(|number| quantize(&intervals, number));
                maybe_interval.map(|interval| show(interval)).unwrap_or_else(|| orig_str.clone())
            };

            let quantized_attribute_values: Vec<String> = col.iter().map(convert).collect();

            // 4. Populate the dataset column:
            quantized_dataset
                .attributes
                .column_mut(idx)
                .assign(&Array1::from(quantized_attribute_values));
        }
    }

    quantized_dataset
}

pub fn show<A: std::fmt::Display, C>(interval: &Interval<A, C>) -> String {
    match interval {
        Interval::Lower { below, .. } => format!("< {}", below),
        Interval::Range { from, below, .. } => format!(">= {} and < {}", from, below),
        Interval::Upper { from, .. } => format!(">= {}", from),
        Interval::Infinite { .. } => String::from("any value"),
    }
}

fn count_distinct<T>(xs: &[T]) -> usize
where
    T: Eq + Hash,
{
    let set: HashSet<&T> = xs.iter().collect();
    set.len()
}

#[cfg(test)]
mod test_iters {
    use super::count_distinct;
    #[test]
    fn test_count_distinct() {
        assert_eq!(0, count_distinct::<u8>(&[]));
        assert_eq!(1, count_distinct(&[0]));
        assert_eq!(1, count_distinct(&[0, 0]));
        assert_eq!(2, count_distinct(&[0, 1, 0]));
    }
}
