use crate::config::Config;
use csv;
use std::error::Error;

#[derive(Debug)]
pub struct Dataset {
    pub attributes: Vec<AttributeName>,
    pub examples: Vec<Example>,
}

pub type AttributeName = String;

#[derive(Debug)]
pub struct Example {
    pub attributes: Vec<Value>,
    pub class: Value,
}

pub type Value = String;

pub fn load(config: &Config) -> Result<Dataset, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(&config.data)?;

    // The header oontains attribute names:
    let headers = rdr.headers()?;
    let attributes: Vec<AttributeName> = headers.iter().map(|name| name.to_owned()).collect();

    // Traverse the rows, converting them into Example records:
    let to_example = |row: csv::StringRecord| {
        let elements: Vec<&str> = row.iter().collect();
        match elements[..].split_last() {
            Some((&last, init)) => Ok(Example {
                attributes: init.iter().map(|&a| a.to_owned()).collect(),
                class: last.to_owned(),
            }),
            _ => csv_failure(format!("Invalid row: {:?}", &elements)),
        }
    };

    let examples: Result<Vec<Example>, _> = rdr
        .records()
        .map(|result| result.and_then(to_example))
        .collect();

    let dataset = Dataset {
        attributes,
        examples: examples?,
    };

    Ok(dataset)
}

fn csv_failure<T>(msg: String) -> Result<T, csv::Error> {
    use std::io::{Error, ErrorKind};
    let cause = Error::new(ErrorKind::Other, msg);
    Err(csv::Error::from(cause))
}
