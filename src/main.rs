use csv_processor::reporter::generate_wide_report;
use csv_processor::{Command, DataFrame, parse_config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_config(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    match config.command() {
        Command::CheckNAs => {
            println!("Checking NAs in file: {}", config.filename());
        }
        Command::CalculateStatistics => {
            println!("Calculating statistics for file: {}", config.filename());
        }
    }

    let df = DataFrame::from_csv(config.filename()).unwrap();

    println!("{}", df);

    let long_report = generate_wide_report(&df);
    println!("{}", long_report)
}
