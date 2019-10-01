use super::oner::Rule;

pub fn as_matcher(rule: &Rule, attribute_names: &[String]) -> String {
    let mut rows = Vec::with_capacity(1 + rule.cases.len());

    rows.push(format!("match {}", attribute_names[rule.attribute_index]));

    for case in &rule.cases {
        rows.push(format!("  {} => {}", case.attribute_value, case.predicted_class));
    }

    rows.join("\n")
}