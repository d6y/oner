mod config;
mod dataset;
mod oner;
mod print;
use config::Config;
use dataset::{AttributeName, Dataset, Example};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashSet;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("{:?}", &config);
    assert!(
        config.use_whole_dataset || config.repeats >= 1,
        "Repeat an experiment at least once, or use the whole dataset"
    );

    let mut rng: StdRng = SeedableRng::seed_from_u64(config.seed);

    match dataset::load(&config.data) {
        Ok(dataset) => {
            if config.use_whole_dataset {
                run_once(&mut rng, &dataset); // Using all the data means no-need to sample
            } else {
                run_many(&mut rng, &dataset, config.training_fraction, config.repeats);
            }
        }
        Err(msg) => println!("Error reading data: {}", msg),
    };
}

fn run_once<R: Rng + ?Sized>(rng: &mut R, dataset: &Dataset<AttributeName, Example>) {
    let (training, _) = dataset.split(rng, 1.0);
    if let Some(rule) = oner::discover(&training) {
        println!(
            "{}",
            print::as_matcher(&rule, &dataset.input_attribute_names)
        );
        println!("Training set: {:?}", oner::evaluate(&rule, &training));
    } else {
        println!("No rule discovered (no data?)");
    }
}

fn run_many<R: Rng + ?Sized>(
    rng: &mut R,
    dataset: &Dataset<AttributeName, Example>,
    training_fraction: f64,
    repeats: usize,
) {
    let mut accuracy = Vec::with_capacity(repeats);
    let mut rules = HashSet::new();

    for r in 1..repeats {
        let (training, testing) = dataset.split(rng, training_fraction);
        if let Some(rule) = oner::discover(&training) {
            let performance = oner::evaluate(&rule, &testing);
            println!("{}: {:?}", r, performance);
            accuracy.push(performance);
            rules.insert(rule);
        }
    }

    println!(
        "Mean test set accuracy: {}",
        accuracy.iter().map(|a| a.0).sum::<f64>() / accuracy.len() as f64
    );

    println!("Rules found:");
    for (i, r) in rules.iter().enumerate() {
        println!(
            "{}: {}",
            i,
            print::as_matcher(r, &dataset.input_attribute_names)
        );
    }
}
