use super::dataset::*;
use itertools::Itertools;

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

/*fn most_frequent_class(dataset: &Dataset<AttributeName, Example>, attribute_index: usize, value: &Value) -> &Value {
    let classes = dataset.examples.iter().filter(|row| &row.attribute_values[attribute_index] == value).map(|row| row.class);
    // find most frequent value
}*/

pub fn discover(dataset: &Dataset<AttributeName, Example>) -> Rule {
    // let hypotheses = Vec::new();

    for (a_index, a_name) in dataset.input_attribute_names.iter().enumerate() {
        for v in distinct_column_values(dataset, a_index) {
            // let class = dataset.max_class(a_index, v);
            println!("{} {} {}", a_index, a_name, &v);
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
