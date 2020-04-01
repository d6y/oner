use anyhow::Result;
use csv::ReaderBuilder;
use ndarray::{s, Array1, Array2, Axis};
use ndarray_csv::Array2Reader;
use rand::Rng;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Dataset {
    pub attribute_names: Vec<String>,
    pub attributes: Array2<String>,
    pub classes: Array1<String>,
}

/// Read a CSV file (with headers and final column as classification) as a `Dataset`.
///
/// The CSV file must:
/// - start with a header row; and
/// - have the class as the last attribute.
pub fn load(path: &PathBuf) -> Result<Dataset> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // The header contains attribute names:
    let headers = rdr.headers()?;
    let num_attributes = headers.len() - 1;
    let attribute_names: Vec<String> =
        headers.iter().map(|name| name.to_owned()).take(num_attributes).collect();

    // Split the CSV data into attributes and class arrays:
    let csv: Array2<String> = rdr.deserialize_array2_dynamic()?;

    // The class is the last column:
    let classes = csv.slice(s![.., -1]).map(|v| v.to_owned());

    // The other columns are attributes:
    let attributes = csv.slice(s![.., 0..-1]).map(|v| v.to_owned());

    // TODO: maybe something like...
    // let (attrbiutes, classes) = csv.split_at(Axis(1), -1);

    Ok(Dataset { attribute_names, attributes, classes })
}

impl Dataset {
    pub fn new(num_rows: usize, attribute_names: &[String]) -> Dataset {
        Dataset {
            attribute_names: attribute_names.to_vec(),
            attributes: Array2::default((num_rows, attribute_names.len())),
            classes: Array1::default(num_rows),
        }
    }

    pub fn split<R: Rng + ?Sized>(&self, rng: &mut R, left_fraction: f64) -> (Dataset, Dataset) {
        // Left and Right refer to the first and second datasets in the return tuple

        let row_axis = Axis(0);
        let num_examples = self.attributes.len_of(row_axis);

        let left_count: usize = (left_fraction * num_examples as f64).round() as usize;
        let right_count: usize = num_examples - left_count;

        // We'll populate these structures...
        let mut left = Dataset::new(left_count, &self.attribute_names);
        let mut right = Dataset::new(right_count, &self.attribute_names);

        // ...from a shuffled set of indexes:
        let random_indicies = rand::seq::index::sample(rng, num_examples, num_examples);

        for (left_index, index) in random_indicies.iter().enumerate() {
            if left_index < left_count {
                left.attributes.row_mut(left_index).assign(&self.attributes.row(index));
                left.classes[left_index] = self.classes[index].clone();
            } else {
                let right_index = left_index - left_count;
                right.attributes.row_mut(right_index).assign(&self.attributes.slice(s![index, ..]));
                right.classes[right_index] = self.classes[index].clone();
            }
        }

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
        let ds = Dataset::new(100, &vec!["a".to_string(), "b".to_string()]);
        let mut rng: StdRng = SeedableRng::seed_from_u64(1u64);
        let (left, right) = ds.split(&mut rng, 2.0 / 3.0);
        assert_eq!(left.attributes.len(), 2 * 67);
        assert_eq!(right.attributes.len(), 2 * (100 - 67));
    }
}
