pub mod io;

use crate::series::{ColumnArray, parse_column};
pub use io::load_dataframe;

#[derive(Debug)]
pub struct DataFrame {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    file_size: usize,
    rows_count: usize,
    cols_count: usize,
    columns: Vec<Box<dyn ColumnArray>>,
}

impl DataFrame {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>, file_size: usize) -> DataFrame {
        let rows_count = rows.len();
        let cols_count = headers.len();
        let columns: Vec<Box<dyn ColumnArray>> = {
            let mut result = Vec::new();
            for index in 0..cols_count {
                let raw_column = rows
                    .iter()
                    .map(|row| row.get(index).map(|s| s.as_str()).unwrap_or(""))
                    .collect();
                result.push(parse_column(raw_column));
            }
            result
        };

        DataFrame {
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

    pub fn columns(&self) -> &[Box<dyn ColumnArray>] {
        &self.columns
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

    pub fn get_column(&self, column_index: usize) -> Option<&Box<dyn ColumnArray>> {
        self.columns.get(column_index)
    }

    pub fn get_row(&self, row_index: usize) -> Option<&Vec<String>> {
        self.rows().get(row_index)
    }
}