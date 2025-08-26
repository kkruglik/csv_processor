use csv_processor::reporter::{generate_info_report, generate_na_report};
use csv_processor::{parse_config, Command, DataFrame};
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

    if let Ok(df) = DataFrame::from_csv(config.filename()) {
        match config.command() {
            Command::CheckNAs => {
                println!("Checking NAs in file: {}", config.filename());
                let report = generate_na_report(&df);
                println!("{}", report)
            }
            Command::Info => {
                println!("Calculating statistics for file: {}", config.filename());
                let report = generate_info_report(&df);
                println!("{}", report)
            }
        }
    } else {
        eprintln!("Error: Failed to read file");
        process::exit(1);
    }
}
