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

    /// Use all the data for training and testing (overrides -t)
    #[structopt(short = "w", long)]
    pub use_whole_dataset: bool,

    /// Number of times to repeat an experiment to report average accuracy
    #[structopt(short, long, default_value = "25")]
    pub repeats: usize,

    /// An attribute must have more than than this number of distinct values for a column to be detected as numeric (and so quantized)
    #[structopt(long, default_value = "6")]
    pub distinct_above: usize,

    /// When quantizing, an interval must have a dominant class must occure more than this many times.
    #[structopt(long, default_value = "3")]
    pub small: usize,

    /// When quantizing, a value to treat as a missing value (in addition to blank attribute values)
    #[structopt(short, long, default_value = "?")]
    pub missing: String,
}
