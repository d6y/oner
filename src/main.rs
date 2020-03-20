mod config;
mod dataset;
mod print;
use anyhow::Result;
use config::Config;
use dataset::Dataset;
use ndarray::s;
use oner_induction::{discover, evaluate};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use structopt::StructOpt;

fn main() -> Result<()> {
    let config = Config::from_args();
    println!("{:?}", &config);

    assert!(
        config.use_whole_dataset
            || (config.training_fraction > 0.0 && config.training_fraction <= 1.0),
        "Training fraction should be between 0 and 1 unless using the whole dataset"
    );

    assert!(
        config.use_whole_dataset || config.repeats >= 1,
        "Repeat an experiment at least once, or use the whole dataset"
    );

    let mut rng: StdRng = SeedableRng::seed_from_u64(config.seed);

    let dataset = dataset::load(&config.data)?;

    if config.use_whole_dataset {
        run_once(&dataset); // Using all the data means no-need to sample
    } else {
        run_many(&mut rng, &dataset, config.training_fraction, config.repeats);
    }
    Ok(())
}

fn run_once(dataset: &Dataset) {
    if let Some((attribute_index, rule)) =
        discover(&dataset.attributes.view(), &dataset.classes.view())
    {
        println!(
            "// Training set accuracy: {:.3}\n{}",
            &rule.accuracy.0,
            print::as_if_then(&rule, attribute_index, &dataset)
        );
    } else {
        println!("No rule discovered (no data?)");
    }
}

fn run_many<R: Rng + ?Sized>(
    rng: &mut R,
    dataset: &Dataset,
    training_fraction: f64,
    repeats: usize,
) {
    let mut accuracy = Vec::with_capacity(repeats);
    let mut rules = Vec::with_capacity(repeats);
    let mut rule_attribute_indicies = Vec::with_capacity(repeats);

    for _r in 1..repeats {
        let (training, testing) = dataset.split(rng, training_fraction);
        if let Some((attribute_index, rule)) =
            discover(&training.attributes.view(), &training.classes.view())
        {
            let test_set_accuracy = evaluate(
                &rule.cases,
                &testing.attributes.slice(s![.., attribute_index]),
                &testing.classes.view(),
            );
            accuracy.push(test_set_accuracy);
            rule_attribute_indicies.push(attribute_index);
            rules.push(rule);
        }
    }

    println!(
        "Mean test set accuracy: {:.3}",
        accuracy.iter().map(|a| a.0).sum::<f64>() / accuracy.len() as f64
    );

    /*println!("Rules found:");
    for (i, (rule, &attribute_index)) ins
        rules.iter().zip(rule_attribute_indicies.iter()).enumerate()
    {
        println!("{}:\n{}", i, print::as_if_then(&rule, attribute_index, &dataset));
    }
    */
}
