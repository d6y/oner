use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Complete data set to learn from (in CSV format, with header row)
    #[structopt(short, long, name = "filename")]
    pub data: PathBuf,

    /// Random seed
    #[structopt(short, long, default_value = "1")]
    pub seed: u64,

    /// Fraction of the data to use for training (vs. testing)
    #[structopt(short, long, default_value = "0.6666666666666666")]
    pub training_fraction: f64,

    /// Number of times to repeat an experiment to report average accuracy
    #[structopt(short, long, default_value = "25")]
    pub repeats: i64,
}
