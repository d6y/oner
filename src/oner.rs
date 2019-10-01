use super::dataset::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    pub attribute_index: usize,
    pub cases: Vec<Case>,
}

#[derive(Debug)]
pub struct Case {
    pub attribute_value: Value,
    pub predicted_class: Value,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Accuracy(f64);

pub fn evaluate(rule: &Rule, dataset: &Dataset<AttributeName, Example>) -> Accuracy {
    let right_wrong: Vec<Option<bool>> = dataset
        .examples
        .iter()
        .map(|example| {
            rule.apply(example)
                .map(|prediction| prediction == example.class)
        })
        .collect();

    // let num_unclassified = right_wrong.into_iter().filter(|o| o.is_none()).count();

    let num_correct = right_wrong.into_iter().filter(|&o| o == Some(true)).count();

    let num_examples = dataset.examples.len();
    if num_examples == 0 {
        Accuracy(0.0)
    } else {
        Accuracy(num_correct as f64 / num_examples as f64)
    }
}

pub trait Interpreter {
    fn apply(&self, example: &Example) -> Option<Value>;
}

impl Interpreter for Rule {
    fn apply(&self, example: &Example) -> Option<Value> {
        let example_value = &example.attribute_values[self.attribute_index];
        let matching_case = self
            .cases
            .iter()
            .find(|&case| &case.attribute_value == example_value);
        matching_case.map(|case| case.predicted_class.to_owned())
    }
}

/// Here we go!
pub fn discover(dataset: &Dataset<AttributeName, Example>) -> Option<Rule> {
    let mut rules = generate_hypotheses(dataset);

    let scores: Vec<Accuracy> = rules.iter().map(|rule| evaluate(rule, dataset)).collect();

    let maybe_best_index = index_of_largest_value(&scores);

    maybe_best_index.map(|i| rules.remove(i))
}

fn index_of_largest_value<V: PartialOrd>(vs: &[V]) -> Option<usize> {
    if let Some(first_value) = vs.first() {
        let init = (0, first_value);
        let best =
            vs.iter().enumerate().fold(
                init,
                |(best_i, best_v), (i, v)| if v > best_v { (i, v) } else { (best_i, best_v) },
            );
        Some(best.0)
    } else {
        None
    }
}

fn generate_hypotheses(dataset: &Dataset<AttributeName, Example>) -> Vec<Rule> {
    let mut hs = Vec::new();

    // For each attribute...
    for (a_index, _a_name) in dataset.input_attribute_names.iter().enumerate() {
        let hypothesis = generate_rule_for_attribute(dataset, a_index);
        hs.push(hypothesis);
    }

    hs
}

fn generate_rule_for_attribute(
    dataset: &Dataset<AttributeName, Example>,
    attribute_index: usize,
) -> Rule {
    let mut cases = Vec::new();
    for v in distinct_column_values(dataset, attribute_index) {
        // Find the most frequent class for that attribute with that value...
        if let Some(class) = most_frequent_class(dataset, attribute_index, &v) {
            cases.push(Case {
                attribute_value: v.to_owned(),
                predicted_class: class.to_owned(),
            });
        }
    }

    Rule {
        attribute_index,
        cases,
    }
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
