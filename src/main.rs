use csv_processor::reporter::{generate_long_report, generate_wide_report};
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
    // println!("{:?}", dataframe);
    // println!("{:?}", dataframe.rows());
    // println!("{:?}", dataframe.columns());

    // println!(
    //     "Len of column 0: {}",
    //     dataframe.get_column(0).unwrap().len()
    // );
    // print!(
    //     "Mean of column 0: {:?}",
    //     dataframe.get_column(0).unwrap().mean()
    // );

    // for idx in 0..dataframe.shape().1 {
    //     let col_mean = dataframe.get_column(idx).unwrap().mean().unwrap_or(0.0);
    //     let col_name = dataframe.headers()[idx].clone();
    //     println!("Header: {}, mean: {}", col_name, col_mean);
    // }

    let wide_report = generate_wide_report(&dataframe);
    println!("{}", wide_report);

    let long_report = generate_long_report(&dataframe);
    println!("{}", long_report)
}
