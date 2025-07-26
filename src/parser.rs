use crate::Config;
use csv::Reader;
use std::fs;

#[derive(Debug)]
pub enum CsvError {
    IoError(std::io::Error),
    ParseError(csv::Error),
    FileNotFound(String),
}

#[derive(Debug)]
pub struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    file_size: usize,
    rows_count: usize,
    cols_count: usize,
}

impl Dataset {
    fn new(headers: Vec<String>, rows: Vec<Vec<String>>, file_size: usize) -> Dataset {
        let rows_count = rows.len();
        let cols_count = headers.len();

        Dataset {
            headers,
            rows,
            file_size,
            rows_count,
            cols_count,
        }
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }

    pub fn row_count(&self) -> usize {
        self.rows_count
    }

    pub fn col_count(&self) -> usize {
        self.cols_count
    }
}

impl From<std::io::Error> for CsvError {
    fn from(error: std::io::Error) -> Self {
        CsvError::IoError(error)
    }
}

impl From<csv::Error> for CsvError {
    fn from(error: csv::Error) -> Self {
        CsvError::ParseError(error)
    }
}

pub fn load_dataset(config: &Config) -> Result<Dataset, CsvError> {
    let mut reader = Reader::from_path(config.filename())?;
    let file_metadata = fs::metadata(config.filename())?;
    let file_size = file_metadata.len() as usize;

    println!("Successfuly loaded csv from {}", config.filename());

    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();

    let mut rows = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row = record.iter().map(|r| r.to_string()).collect();
        rows.push(row);
    }

    Ok(Dataset::new(headers, rows, file_size))
}
