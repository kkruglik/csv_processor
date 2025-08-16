pub mod io;

use crate::series::{ColumnArray, parse_column};
pub use io::load_dataframe;
use std::fmt;

#[derive(Debug)]
pub struct DataFrame {
    headers: Option<Vec<String>>,
    rows: Option<Vec<Vec<String>>>,
    columns: Vec<Box<dyn ColumnArray>>,
}

impl DataFrame {
    pub fn new(headers: Option<Vec<String>>, rows: Option<Vec<Vec<String>>>) -> Self {
        if let Some(rows) = &rows
            && let Some(headers) = &headers
        {
            if headers.len() != rows[0].len() {
                panic!(
                    "Headers and columns have different size: headers={}, columns={}",
                    headers.len(),
                    rows[0].len()
                );
            }
        }

        let cols_count = match (&headers, &rows) {
            (Some(h), _) => h.len(),
            (None, Some(r)) if !r.is_empty() => r[0].len(),
            _ => 0,
        };

        let columns: Vec<Box<dyn ColumnArray>> = {
            match &rows {
                Some(rows) if !rows.is_empty() => {
                    let mut result = Vec::new();
                    for index in 0..cols_count {
                        let raw_column = rows
                            .iter()
                            .map(|row| row.get(index).map(|s| s.as_str()).unwrap_or(""))
                            .collect();
                        result.push(parse_column(raw_column));
                    }
                    result
                }
                _ => Vec::new(),
            }
        };

        DataFrame {
            headers,
            rows,
            columns,
        }
    }

    fn rows_count(&self) -> usize {
        self.rows.as_ref().map_or(0, |r| r.len())
    }

    fn columns_count(&self) -> usize {
        let cols_count = match (&self.headers, &self.rows) {
            (Some(h), _) => h.len(),
            (None, Some(r)) if !r.is_empty() => r[0].len(),
            _ => 0,
        };
        cols_count
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows_count(), self.columns_count())
    }

    pub fn headers(&self) -> &[String] {
        self.headers.as_deref().unwrap_or(&[])
    }

    pub fn rows(&self) -> &[Vec<String>] {
        self.rows.as_deref().unwrap_or(&[])
    }

    pub fn columns(&self) -> &[Box<dyn ColumnArray>] {
        &self.columns
    }

    pub fn get_column(&self, column_index: usize) -> Option<&Box<dyn ColumnArray>> {
        self.columns.get(column_index)
    }

    pub fn get_row(&self, row_index: usize) -> Option<&Vec<String>> {
        self.rows().get(row_index)
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

            max_width = max_width.max(8).min(20);
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
