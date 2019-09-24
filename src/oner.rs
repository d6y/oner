use super::dataset::*;

#[derive(Debug)]
pub struct Rule {
    attribute_index: usize,
    attribute_value: Value,
    predicted_class: Value,
}

pub fn find(_dataset: &Dataset<AttributeName, Example>) -> Rule {
    Rule {
        attribute_index: 0,
        attribute_value: String::from("foo"),
        predicted_class: String::from("baz"),
    }        
}