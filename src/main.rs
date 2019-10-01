mod config;
mod dataset;
mod oner;
mod print;
use config::Config;
use rand::rngs::StdRng;
use rand::SeedableRng;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("{:?}", &config);

    let mut rng: StdRng = SeedableRng::seed_from_u64(config.seed);

    match dataset::load(&config.data) {
        Ok(dataset) => {
            let (training, testing) = if config.use_whole_dataset {
                (dataset.clone(), dataset)
            } else {
                dataset.split(&mut rng, config.training_fraction)
            };

            println!(
                "Training set size: {}, test set size: {}",
                training.examples.len(),
                testing.examples.len()
            );

            if let Some(rule) = oner::discover(&training) {
                println!(
                    "{}",
                    print::as_matcher(&rule, &training.input_attribute_names)
                );
                println!("Test set: {:?}", oner::evaluate(&rule, &testing));
            } else {
                println!("No rule discovered (no data?)");
            };
        }
        Err(msg) => println!("Error reading data: {}", msg),
    };
}
