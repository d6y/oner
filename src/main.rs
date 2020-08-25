mod config;
mod dataset;
mod print;
mod quantize;
use anyhow::Result;
use config::Config;
use dataset::Dataset;
use ndarray::{s, Zip};
use oner_induction::{discover, evaluate, interpret, Rule};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::fs::File;
use std::io::Write;
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

    let csv_dataset = dataset::load(&config.data)?;

    let dataset = quantize::auto_quantize(
        csv_dataset,
        1 + config.distinct_above,
        config.small,
        &config.missing,
    );

    if config.use_whole_dataset {
        let (attribute_index, rule) = run_once(&dataset); // Using all the data means no-need to sample
        println!("// Training set accuracy: {:.3}", &rule.accuracy.0);
        if let Some(csv_filename) = config.csv_predictions {
            let csv = print::print_predictions(
                &config.missing,
                predict(attribute_index, &rule, &dataset),
            );
            let mut file = File::create(csv_filename)?;
            file.write_all(csv.as_bytes())?;
        }
        if !config.hide_rules {
            println!("{}", print::as_if_then(&rule, attribute_index, &dataset));
        }
    } else {
        let (attribute_indicies, rules) =
            run_many(&mut rng, &dataset, config.training_fraction, config.repeats);
        if !config.hide_rules {
            println!("Rules found:");
            for (i, (rule, attribute_index)) in
                rules.iter().zip(attribute_indicies.iter()).enumerate()
            {
                println!(
                    "{}:\n// Training set accuracy: {:.3}\n{}",
                    i,
                    rule.accuracy.0,
                    print::as_if_then(rule, *attribute_index, &dataset)
                );
            }
        }
    }

    Ok(())
}

fn run_once(dataset: &Dataset) -> (usize, Rule<String, String>) {
    if let Some((attribute_index, rule)) =
        discover(&dataset.attributes.view(), &dataset.classes.view())
    {
        (attribute_index, rule)
    } else {
        panic!("No rule discovered (no data?)");
    }
}

fn run_many<R: Rng + ?Sized>(
    rng: &mut R,
    dataset: &Dataset,
    training_fraction: f64,
    repeats: usize,
) -> (Vec<usize>, Vec<Rule<String, String>>) {
    let mut accuracy = Vec::with_capacity(repeats);
    let mut rules = Vec::with_capacity(repeats);
    let mut rule_attribute_indicies = Vec::with_capacity(repeats);

    for _r in 0..repeats {
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

    (rule_attribute_indicies, rules)
}

fn predict(
    attribute_index: usize,
    rule: &Rule<String, String>,
    dataset: &Dataset,
) -> Vec<(String, Option<String>)> {
    let mut predictions = Vec::with_capacity(dataset.attributes.len());

    let attribute_values = dataset.attributes.column(attribute_index);

    Zip::from(attribute_values).and(&dataset.classes).apply(|attribute_value, class| {
        let prediction = interpret(&rule.cases, attribute_value);
        predictions.push((class.to_owned(), prediction.map(|str| str.to_owned())));
    });

    predictions
}
