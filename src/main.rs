mod config;
mod dataset;
use config::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("{:?}", &config);

    match dataset::load(&config.data) {
        Ok(dataset) => println!("{:?}", dataset),
        Err(msg) => println!("Error reading data: {}", msg),
    };
}
