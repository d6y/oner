use super::dataset::Dataset;
use oner_induction::{Case, Rule};

pub fn as_if_then(
    rule: &Rule<String, String>,
    attribute_index: usize,
    dataset: &Dataset,
) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    let attr_name = &dataset.attribute_names[attribute_index];

    // The output is more readable if we sort by the attribute value of each case
    // TODO: implement clone on Case
    let mut sorted_cases = Vec::new();
    for case in &rule.cases {
        sorted_cases.push(Case {
            attribute_value: case.attribute_value.to_owned(),
            predicted_class: case.predicted_class.to_owned(),
        });
    }
    sorted_cases.sort_by_key(|c| c.attribute_value.to_owned());

    for case in sorted_cases {
        rows.push(format!(
            "IF {} IS {} THEN {}",
            attr_name, case.attribute_value, case.predicted_class
        ));
    }

    rows.join("\n")
}

// An alternative output format
#[allow(dead_code)]
pub fn as_matcher(
    rule: &Rule<String, String>,
    attribute_index: usize,
    dataset: &Dataset,
) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    rows.push(format!("match {}", dataset.attribute_names[attribute_index]));

    for case in &rule.cases {
        rows.push(format!("  {} => {}", case.attribute_value, case.predicted_class));
    }

    rows.join("\n")
}

pub fn print_predictions(missing: &str, predictions: Vec<(String, Option<String>)>) -> String {
    let mut rows = Vec::with_capacity(1 + predictions.len());

    rows.push(String::from("Actual,Prediction"));

    for (actual, predicted) in predictions.iter() {
        let pred_text = match predicted {
            Some(value) => value,
            None => missing,
        };
        let row = format!("{},{}", actual, pred_text);
        rows.push(row);
    }

    rows.join("\n")
}
