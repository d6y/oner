use csv;
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Dataset<N, X> {
    pub attribute_names: Vec<N>,
    pub examples: Vec<X>,
}

pub type AttributeName = String;

#[derive(Debug)]
pub struct Example {
    pub attribute_values: Vec<Value>,
    pub class: Value,
}

pub type Value = String;

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

    let dataset = Dataset {
        attribute_names,
        examples: examples?,
    };

    Ok(dataset)
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
            attribute_names: self.attribute_names.clone(),
            examples: left_examples,
        };

        let right = Dataset {
            attribute_names: self.attribute_names,
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
            attribute_names: vec!["a", "b", "c", "d"],
            examples: vec![1, 2, 3, 4],
        };
        let mut rng: StdRng = SeedableRng::seed_from_u64(3u64);
        let (left, right) = ds.split(&mut rng, 2.0 / 3.0);
        assert_eq!(left.examples.len(), 3);
        assert_eq!(right.examples.len(), 1);
    }
}
