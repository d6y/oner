use super::dataset::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    attribute_index: usize,
    // attribute_name: Value,
    attribute_value: Value,
    predicted_class: Value,
    // training_set_accuracy: f64,
}

fn distinct_column_values(
    dataset: &Dataset<AttributeName, Example>,
    attribute_index: usize,
) -> Vec<&Value> {
    dataset
        .examples
        .iter()
        .map(|row| &row.attribute_values[attribute_index])
        .unique()
        .collect()
}

fn most_frequent_class<'d>(
    dataset: &'d Dataset<AttributeName, Example>,
    attribute_index: usize,
    value: &str,
) -> Option<&'d Value> {
    let classes = dataset
        .examples
        .iter()
        .filter(|row| row.attribute_values[attribute_index] == value)
        .map(|row| &row.class);

    let mut counts = HashMap::new();
    for class in classes {
        *counts.entry(class).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(class, _)| class)
}

#[cfg(test)]
mod test_freq_class {
    use super::most_frequent_class;
    use super::Dataset;
    use super::Example;
    #[test]
    fn find_most_frequent_class() {
        let dataset = Dataset {
            input_attribute_names: vec![String::from("x")],
            output_attribute_name: String::from("y"),
            examples: vec![
                Example {
                    attribute_values: vec![String::from("yes")],
                    class: String::from("lo"),
                },
                Example {
                    attribute_values: vec![String::from("noo")],
                    class: String::from("lo"),
                },
                Example {
                    attribute_values: vec![String::from("yes")],
                    class: String::from("hi"),
                },
                Example {
                    attribute_values: vec![String::from("yes")],
                    class: String::from("hi"),
                },
            ],
        };
        assert_eq!(
            most_frequent_class(&dataset, 0, "yes"),
            Some(&String::from("hi"))
        );
    }
}

pub fn discover(dataset: &Dataset<AttributeName, Example>) -> Rule {
    // let hypotheses = Vec::new();

    for (a_index, a_name) in dataset.input_attribute_names.iter().enumerate() {
        for v in distinct_column_values(dataset, a_index) {
            let class = most_frequent_class(dataset, a_index, &v);
            println!("{} IF {} = {} THEN {:?}", a_index, a_name, &v, &class);
        }
    }

    Rule {
        attribute_index: 0,
        // attribute_name: String::from("a1"),
        attribute_value: String::from("v1"),
        predicted_class: String::from("vc"),
        // training_set_accuracy: 0.0,
    }
}
