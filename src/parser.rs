use crate::Config;
use csv::Reader;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    file_size: usize,
    rows_count: usize,
    cols_count: usize,
    pub columns: Vec<Vec<CellValue>>,
}

fn parse_cell(cell: &str) -> CellValue {
    if cell.is_empty() || cell.to_lowercase() == "na" {
        return CellValue::Null;
    }

    if let Ok(val) = cell.parse::<i64>() {
        return CellValue::Integer(val);
    }

    if let Ok(val) = cell.parse::<f64>() {
        return CellValue::Float(val);
    }

    match cell.to_lowercase().as_str() {
        "true" | "1" | "yes" => return CellValue::Boolean(true),
        "false" | "0" | "no" => return CellValue::Boolean(false),
        _ => {}
    }

    CellValue::Str(cell.to_string())
}

impl Dataset {
    fn new(headers: Vec<String>, rows: Vec<Vec<String>>, file_size: usize) -> Dataset {
        let rows_count = rows.len();
        let cols_count = headers.len();
        let columns = {
            let mut result = Vec::new();
            for index in 0..cols_count {
                let new_col = rows
                    .iter()
                    .map(|row| {
                        row.get(index)
                            .map(|s| parse_cell(s))
                            .unwrap_or(CellValue::Null)
                    })
                    .collect();
                result.push(new_col);
            }
            result
        };

        Dataset {
            headers,
            rows,
            file_size,
            rows_count,
            cols_count,
            columns,
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

    fn get_column(&self, column_index: usize) -> Vec<&str> {
        self.rows
            .iter()
            .map(|row| row.get(column_index).map(|s| s.as_str()).unwrap_or(""))
            .collect()
    }

    fn get_row(&self, row_index: usize) -> Option<&Vec<String>> {
        self.rows().get(row_index)
    }

    pub fn get_column_by_name(&self, column_name: &str) -> Option<Vec<&str>> {
        self.headers()
            .iter()
            .position(|h| h == column_name)
            .map(|index| self.get_column(index))
    }

    pub fn get_column_by_index(&self, column_index: usize) -> Option<Vec<&str>> {
        if column_index < self.headers().len() {
            Some(self.get_column(column_index))
        } else {
            None
        }
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
