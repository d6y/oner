//! `Dataset` is a container for examples along with the names of instances.
//!
use csv;
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Dataset<N, X> {
    pub input_attribute_names: Vec<N>,
    pub output_attribute_name: N,
    pub examples: Vec<X>,
}

pub type AttributeName = String;

#[derive(Debug, Clone)]
pub struct Example {
    pub attribute_values: Vec<Value>,
    pub class: Value,
}

pub type Value = String;

/// Read a CSV file (with headers and final column as classification) as a `Dataset`.
/// 
/// The CSV file must:
/// - start with a header row; and 
/// - have the class as the last attribute.
pub fn load(path: &PathBuf) -> Result<Dataset<AttributeName, Example>, csv::Error> {
    let mut rdr = csv::Reader::from_path(path)?;

    // The header contains attribute names:
    let headers = rdr.headers()?;
    let attribute_names: Vec<AttributeName> = headers.iter().map(|name| name.to_owned()).collect();

    // How to turn a row of String values into an `Example`
    let to_example = |row: csv::StringRecord| {
        let elements: Vec<&str> = row.iter().collect();
        match elements[..].split_last() {
            Some((&last, init)) => Ok(Example {
                attribute_values: init.iter().map(|&a| a.to_owned()).collect(),
                class: last.to_owned(),
            }),
            _ => csv_failure(format!(
                "Rows should contain at least two values. Found: {:?}",
                &elements
            )),
        }
    };

    // Traverse the rows, converting them into `Example` records:
    let examples: Result<Vec<Example>, _> = rdr
        .records()
        .map(|result| result.and_then(to_example))
        .collect();

    match attribute_names[..].split_last() {
        None => csv_failure(format!(
            "Too few attribute names (header row problem): {:?}",
            &attribute_names
        )),
        Some((class, inputs)) => Ok(Dataset {
            input_attribute_names: inputs.to_vec(),
            output_attribute_name: class.to_owned(),
            examples: examples?,
        }),
    }
}

fn csv_failure<T>(msg: String) -> Result<T, csv::Error> {
    use std::io::{Error, ErrorKind};
    let cause = Error::new(ErrorKind::Other, msg);
    Err(csv::Error::from(cause))
}

impl<N, X> Dataset<N, X> {
    pub fn split<R>(mut self, rng: &mut R, left_fraction: f64) -> (Self, Self)
    where
        R: Rng + ?Sized,
        N: Clone,
    {
        let left_count: usize = (left_fraction * self.examples.len() as f64).round() as usize;

        let mut left_examples = Vec::with_capacity(left_count);
        let mut right_examples = Vec::with_capacity(self.examples.len() - left_count);

        self.examples.shuffle(rng);
        for (i, example) in self.examples.into_iter().enumerate() {
            if i < left_count {
                left_examples.push(example);
            } else {
                right_examples.push(example);
            }
        }

        let left = Dataset {
            input_attribute_names: self.input_attribute_names.clone(),
            output_attribute_name: self.output_attribute_name.clone(),
            examples: left_examples,
        };

        let right = Dataset {
            input_attribute_names: self.input_attribute_names,
            output_attribute_name: self.output_attribute_name,
            examples: right_examples,
        };

        (left, right)
    }
}

#[cfg(test)]
mod test_split {
    use super::Dataset;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    #[test]
    fn test_can_split() {
        let ds = Dataset {
            input_attribute_names: vec!["a"],
            output_attribute_name: "z",
            examples: vec![1, 2, 3, 4],
        };
        let mut rng: StdRng = SeedableRng::seed_from_u64(3u64);
        let (left, right) = ds.split(&mut rng, 2.0 / 3.0);
        assert_eq!(left.examples.len(), 3);
        assert_eq!(right.examples.len(), 1);
    }
}
