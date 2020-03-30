use super::dataset::Dataset;
use ndarray::Axis;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

pub fn auto_quantize(dataset: Dataset, min_distinct_values: usize, missing_value: &str) -> Dataset {
    let all_either_numeric_or_missing = |xs: &[String]| -> bool {
        xs.iter().all(|x| x == &"" || x == &missing_value || f32::from_str(x).is_ok())
    };

    let is_numeric = |values: &[String]| -> bool {
        all_either_numeric_or_missing(values) && count_distinct(values) >= min_distinct_values
    };

    let col_axis = Axis(1);

    for (idx, col) in dataset.attributes.axis_iter(col_axis).enumerate() {
        if is_numeric(&col.to_vec()) {
            println!("Found a numeric column! {} {}", idx, &dataset.attribute_names[idx]);
        }
    }

    dataset
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
