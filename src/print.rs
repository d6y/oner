use super::dataset::Dataset;
use oner_induction::Rule;

pub fn as_if_then(
    rule: &Rule<String, String>,
    attribute_index: usize,
    dataset: &Dataset,
) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    let attr_name = &dataset.attribute_names[attribute_index];

    for case in &rule.cases {
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
