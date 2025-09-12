use csv_processor::{
    parse_config,
    reporter::{generate_info_report, generate_na_report},
    Command, DataFrame, JsonExportOrient,
};
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
                let report = generate_info_report(&df);
                println!("{}", report);

                // for i in 0..df.shape().1 {
                //     let json_output = df.get_column(i).unwrap().to_json();
                //     println!("Json column index {} output: {:?}", i, json_output);
                // }
                //
                // println!("Json output: {}", df.to_json().unwrap());
            }
        }

        // let json_output_with_columns = df.to_json(JsonExportOrient::Columns).unwrap();
        // println!("Json columns output: {}", json_output_with_columns);
        //
        // let json_output_with_records = df.to_json(JsonExportOrient::Records).unwrap();
        // println!("Json records output: {}", json_output_with_records);
        //
        // let json_output_with_values = df.to_json(JsonExportOrient::Values).unwrap();
        // println!("Json values output: {}", json_output_with_values);
    } else {
        eprintln!("Error: Failed to read file");
        process::exit(1);
    }
}
