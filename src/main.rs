use csv_processor::{Command, load_dataframe, parse_config};
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

    let dataframe = load_dataframe(&config).unwrap();
    println!("{:?}", dataframe);
    println!("{:?}", dataframe.rows());
    println!("{:?}", dataframe.columns());
}
