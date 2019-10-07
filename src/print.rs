use super::oner::Rule;

pub fn as_if_then(rule: &Rule, attribute_names: &[String], class_name: &str) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    let attr_name = &attribute_names[rule.attribute_index];

    for case in &rule.cases {
        rows.push(format!(
            "IF {}={} THEN {}={}",
            attr_name, case.attribute_value, class_name, case.predicted_class
        ));
    }

    rows.join("\n")
}

// An alternative output format
#[allow(dead_code)]
pub fn as_matcher(rule: &Rule, attribute_names: &[String]) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    rows.push(format!("match {}", attribute_names[rule.attribute_index]));

    for case in &rule.cases {
        rows.push(format!(
            "  {} => {}",
            case.attribute_value, case.predicted_class
        ));
    }

    rows.join("\n")
}
