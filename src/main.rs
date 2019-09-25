mod config;
mod dataset;
mod oner;
use config::Config;
use oner::Interpreter;
use rand::rngs::StdRng;
use rand::SeedableRng;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("{:?}", &config);

    let mut rng: StdRng = SeedableRng::seed_from_u64(config.seed);

    match dataset::load(&config.data) {
        Ok(dataset) => {
            println!("Full dataset size: {}", dataset.examples.len());
            let (training, testing) = dataset.split(&mut rng, config.training_fraction);
            println!(
                "Training set size: {}, test set size: {}",
                training.examples.len(),
                testing.examples.len()
            );

            if let Some(rule) = oner::discover(&training) {
                println!("{:#?}", &rule.print(&training.input_attribute_names));
                println!("Test set: {:?}", &rule.apply(&testing));
            } else {
                println!("No rule (no data?)");
            };
        }
        Err(msg) => println!("Error reading data: {}", msg),
    };
}
