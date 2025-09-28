mod error;

use std::fmt;

use crate::{
    series::{parse_column, ColumnArray},
    CellValue,
};
use csv::Reader;
pub use error::DataFrameError;
use serde_json::json;

#[derive(Debug)]
pub struct DataFrame {
    headers: Option<Vec<String>>,
    columns: Vec<Box<dyn ColumnArray>>,
}

pub enum JsonExportOrient {
    Records,
    Values,
    Columns,
}

impl DataFrame {
    pub fn new<C>(headers: Option<Vec<String>>, columns: Vec<C>) -> Result<Self, DataFrameError>
    where
        C: Into<Box<dyn ColumnArray>>,
    {
        let columns: Vec<Box<dyn ColumnArray>> =
            columns.into_iter().map(|col| col.into()).collect();

        let headers = headers.unwrap_or_default();

        if !headers.is_empty() && !columns.is_empty() && headers.len() != columns.len() {
            return Err(DataFrameError::HeadersColumnsLengthMismatch {
                headers: headers.len(),
                columns: columns.len(),
            });
        }

        // Validate all columns have same length
        if let Some(expected_len) = columns.first().map(|col| col.len()) {
            for (i, col) in columns.iter().enumerate() {
                if col.len() != expected_len {
                    let header_name = headers.get(i).map(|s| s.as_str()).unwrap_or("unknown");
                    return Err(DataFrameError::ColumnsLengthMismatch {
                        column: header_name.to_string(),
                        expected: expected_len,
                        actual: col.len(),
                    });
                }
            }
        }

        Ok(DataFrame {
            headers: Some(headers),
            columns,
        })
    }

    pub fn from_columns(
        headers: Option<Vec<String>>,
        columns: Vec<Box<dyn ColumnArray>>,
    ) -> Result<Self, DataFrameError> {
        let headers = headers.unwrap_or_default();

        if !headers.is_empty() && !columns.is_empty() && headers.len() != columns.len() {
            return Err(DataFrameError::HeadersColumnsLengthMismatch {
                headers: headers.len(),
                columns: columns.len(),
            });
        }

        // Validate all columns have same length
        if let Some(expected_len) = columns.first().map(|col| col.len()) {
            for (i, col) in columns.iter().enumerate() {
                if col.len() != expected_len {
                    let header_name = headers.get(i).map(|s| s.as_str()).unwrap_or("unknown");
                    return Err(DataFrameError::ColumnsLengthMismatch {
                        column: header_name.to_string(),
                        expected: expected_len,
                        actual: col.len(),
                    });
                }
            }
        }

        Ok(DataFrame {
            headers: Some(headers),
            columns,
        })
    }

    pub fn from_values(
        headers: Option<Vec<String>>,
        raw_columns: Vec<Vec<String>>,
    ) -> Result<Self, DataFrameError> {
        // Convert raw string columns to properly typed columns using parse_column
        let columns: Vec<Box<dyn ColumnArray>> = raw_columns
            .into_iter()
            .map(|col| {
                let str_refs: Vec<&str> = col.iter().map(|s| s.as_str()).collect();
                parse_column(str_refs)
            })
            .collect();

        let headers = headers.unwrap_or_default();

        if !headers.is_empty() && !columns.is_empty() && headers.len() != columns.len() {
            return Err(DataFrameError::HeadersColumnsLengthMismatch {
                headers: headers.len(),
                columns: columns.len(),
            });
        }

        Ok(DataFrame {
            headers: Some(headers),
            columns,
        })
    }

    pub fn from_csv(filename: &str) -> Result<Self, DataFrameError> {
        let mut reader =
            Reader::from_path(filename).map_err(|e| DataFrameError::IoError(e.to_string()))?;

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| DataFrameError::CsvError(e.to_string()))?
            .iter()
            .map(|h| h.to_string())
            .collect();

        let cols_count = headers.len();

        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result.map_err(|e| DataFrameError::CsvError(e.to_string()))?;
            let row: Vec<String> = record.iter().map(|r| r.to_string()).collect();

            if row.len() != cols_count {
                return Err(DataFrameError::RowLengthMismatch {
                    index: rows.len() + 1,
                    expected: cols_count,
                    actual: row.len(),
                });
            }
            rows.push(row);
        }

        // Convert rows to columns
        let columns: Vec<Box<dyn ColumnArray>> = if !rows.is_empty() {
            (0..cols_count)
                .map(|col_index| {
                    let raw_column: Vec<&str> = rows
                        .iter()
                        .map(|row| row.get(col_index).map(|s| s.as_str()).unwrap_or(""))
                        .collect();
                    parse_column(raw_column)
                })
                .collect()
        } else {
            Vec::new()
        };

        Self::new(Some(headers), columns)
    }

    pub fn from_strings(content: &str) -> Result<Self, DataFrameError> {
        let mut reader = Reader::from_reader(content.as_bytes());
        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| DataFrameError::CsvError(e.to_string()))?
            .iter()
            .map(|h| h.to_string())
            .collect();

        let cols_count = headers.len();

        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result.map_err(|e| DataFrameError::CsvError(e.to_string()))?;
            let row: Vec<String> = record.iter().map(|r| r.to_string()).collect();

            if row.len() != cols_count {
                return Err(DataFrameError::RowLengthMismatch {
                    index: rows.len() + 1,
                    expected: cols_count,
                    actual: row.len(),
                });
            }
            rows.push(row);
        }

        // Convert rows to columns
        let columns: Vec<Box<dyn ColumnArray>> = if !rows.is_empty() {
            (0..cols_count)
                .map(|col_index| {
                    let raw_column: Vec<&str> = rows
                        .iter()
                        .map(|row| row.get(col_index).map(|s| s.as_str()).unwrap_or(""))
                        .collect();
                    parse_column(raw_column)
                })
                .collect()
        } else {
            Vec::new()
        };

        Self::new(Some(headers), columns)
    }

    pub fn empty() -> Self {
        DataFrame {
            headers: None,
            columns: Vec::new(),
        }
    }

    fn columns_count(&self) -> usize {
        self.columns.len()
    }

    fn rows_count(&self) -> usize {
        if !self.columns.is_empty() {
            return self.columns[0].len();
        }
        0
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows_count(), self.columns_count())
    }

    pub fn headers(&self) -> &[String] {
        self.headers.as_deref().unwrap_or(&[])
    }

    pub fn columns(&self) -> &[Box<dyn ColumnArray>] {
        &self.columns
    }

    pub fn get_column(&self, column_index: usize) -> Option<&Box<dyn ColumnArray>> {
        self.columns.get(column_index)
    }

    fn _to_json_with_columns(&self) -> serde_json::Value {
        let headers = {
            match self.headers.as_ref() {
                Some(headers) => headers.clone(),
                None => (0..self.shape().1).map(|i| i.to_string()).collect(),
            }
        };
        let mut columns: Vec<Vec<serde_json::Value>> = Vec::new();

        let mut output_json = json!({
            "headers": headers,
            "columns": columns
        });

        for col_idx in 0..self.shape().1 {
            let column = self.get_column(col_idx).unwrap();
            let json_column = column.to_json();
            columns.push(json_column);
        }

        output_json["columns"] = json!(columns);
        output_json
    }

    fn _to_json_with_records(&self) -> serde_json::Value {
        let mut output_json: Vec<serde_json::Value> = vec![];
        let headers = {
            match self.headers.as_ref() {
                Some(headers) => headers.clone(),
                None => (0..self.shape().1).map(|i| i.to_string()).collect(),
            }
        };
        for row_idx in 0..self.shape().0 {
            let mut json_row = json!({});

            for col_idx in 0..self.shape().1 {
                let column = self.get_column(col_idx).unwrap();
                let cell_value = column.get(row_idx).unwrap_or(crate::CellValue::Null);
                match cell_value {
                    CellValue::Str(s) => json_row[&headers[col_idx]] = json!(s),
                    CellValue::Float(f) => json_row[&headers[col_idx]] = json!(f),
                    CellValue::Integer(i) => json_row[&headers[col_idx]] = json!(i),
                    CellValue::Boolean(b) => json_row[&headers[col_idx]] = json!(b),
                    CellValue::Date(d) => json_row[&headers[col_idx]] = json!(d),
                    CellValue::Null => json_row[&headers[col_idx]] = json!(null),
                }
            }
            output_json.push(json_row);
        }

        json!(output_json)
    }

    fn _to_json_with_values(&self) -> serde_json::Value {
        let mut output_json: Vec<serde_json::Value> = vec![];

        for row_idx in 0..self.shape().0 {
            let mut json_row = json!({});

            for col_idx in 0..self.shape().1 {
                let column = self.get_column(col_idx).unwrap();
                let cell_value = column.get(row_idx).unwrap_or(crate::CellValue::Null);
                match cell_value {
                    CellValue::Str(s) => json_row[format!("{col_idx}")] = json!(s),
                    CellValue::Float(f) => json_row[format!("{col_idx}")] = json!(f),
                    CellValue::Integer(i) => json_row[format!("{col_idx}")] = json!(i),
                    CellValue::Boolean(b) => json_row[format!("{col_idx}")] = json!(b),
                    CellValue::Date(d) => json_row[format!("{col_idx}")] = json!(d),
                    CellValue::Null => json_row[format!("{col_idx}")] = json!(null),
                }
            }
            output_json.push(json_row);
        }

        json!(output_json)
    }

    pub fn to_json(&self, orient: JsonExportOrient) -> Result<String, DataFrameError> {
        let result = match orient {
            JsonExportOrient::Columns => self._to_json_with_columns(),
            JsonExportOrient::Records => self._to_json_with_records(),
            JsonExportOrient::Values => self._to_json_with_values(),
        };
        match serde_json::to_string(&result) {
            Ok(s) => Ok(s),
            Err(e) => Err(DataFrameError::JsonError(e.to_string())),
        }
    }
}

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rows, cols) = self.shape();

        if cols == 0 {
            return writeln!(f, "0 rows × 0 columns\n(empty)");
        }

        let headers = self.headers();
        let max_display_rows = 10;
        let show_truncated = rows > max_display_rows;
        let display_rows = if show_truncated {
            max_display_rows - 1
        } else {
            rows
        };

        let mut col_widths = Vec::new();

        for (i, header) in headers.iter().enumerate() {
            let mut max_width = header.len();

            let column = &self.columns[i];
            for row_idx in 0..display_rows {
                if let Some(cell_value) = column.get(row_idx) {
                    let cell_str = format!("{}", cell_value);
                    max_width = max_width.max(cell_str.len());
                }
            }

            if show_truncated && display_rows < rows - 1 {
                if let Some(cell_value) = column.get(rows - 1) {
                    let cell_str = format!("{}", cell_value);
                    max_width = max_width.max(cell_str.len());
                }
            }

            // max_width = max_width.max(8).min(20);
            max_width = max_width.clamp(8, 20);
            col_widths.push(max_width);
        }

        write!(f, "┌")?;
        for (i, &width) in col_widths.iter().enumerate() {
            write!(f, "{}", "─".repeat(width + 2))?;
            if i < col_widths.len() - 1 {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        write!(f, "│")?;
        for (header, &width) in headers.iter().zip(&col_widths) {
            let truncated_header = if header.len() > width {
                format!("{}…", &header[..width - 1])
            } else {
                header.clone()
            };
            write!(f, " {:^width$} │", truncated_header, width = width)?;
        }
        writeln!(f)?;

        write!(f, "├")?;
        for (i, &width) in col_widths.iter().enumerate() {
            write!(f, "{}", "─".repeat(width + 2))?;
            if i < col_widths.len() - 1 {
                write!(f, "┼")?;
            }
        }
        writeln!(f, "┤")?;

        for row_idx in 0..display_rows {
            write!(f, "│")?;
            for (col_idx, &width) in col_widths.iter().enumerate() {
                let cell_str = if let Some(cell_value) = self.columns[col_idx].get(row_idx) {
                    let full_str = format!("{}", cell_value);
                    if full_str.len() > width {
                        format!("{}…", &full_str[..width - 1])
                    } else {
                        full_str
                    }
                } else {
                    "null".to_string()
                };

                write!(f, " {:^width$} │", cell_str, width = width)?;
            }
            writeln!(f)?;
        }

        if show_truncated {
            write!(f, "│")?;
            for &width in &col_widths {
                write!(f, " {:^width$} │", "⋮", width = width)?;
            }
            writeln!(f)?;

            write!(f, "│")?;
            for (col_idx, &width) in col_widths.iter().enumerate() {
                let cell_str = if let Some(cell_value) = self.columns[col_idx].get(rows - 1) {
                    let full_str = format!("{}", cell_value);
                    if full_str.len() > width {
                        format!("{}…", &full_str[..width - 1])
                    } else {
                        full_str
                    }
                } else {
                    "null".to_string()
                };

                write!(f, " {:^width$} │", cell_str, width = width)?;
            }
            writeln!(f)?;
        }

        write!(f, "└")?;
        for (i, &width) in col_widths.iter().enumerate() {
            write!(f, "{}", "─".repeat(width + 2))?;
            if i < col_widths.len() - 1 {
                write!(f, "┴")?;
            }
        }
        writeln!(f, "┘")?;

        write!(f, "{} rows × {} columns", rows, cols)?;

        Ok(())
    }
}
