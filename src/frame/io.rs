use super::DataFrame;
use crate::{Config, CsvError};
use csv::Reader;

pub fn load_dataframe(config: &Config) -> Result<DataFrame, CsvError> {
    let mut reader = Reader::from_path(config.filename())?;

    println!("Successfuly loaded csv from {}", config.filename());

    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();

    let mut rows = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row = record.iter().map(|r| r.to_string()).collect();
        rows.push(row);
    }

    Ok(DataFrame::new(Some(headers), Some(rows)))
}
