use csv;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Dataset {
    pub attribute_names: Vec<AttributeName>,
    pub examples: Vec<Example>,
}

pub type AttributeName = String;

#[derive(Debug)]
pub struct Example {
    pub attribute_values: Vec<Value>,
    pub class: Value,
}

pub type Value = String;

pub fn load(path: &PathBuf) -> Result<Dataset, csv::Error> {
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
