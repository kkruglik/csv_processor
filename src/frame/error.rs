use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataFrameError {
    HeadersColumnsLengthMismatch {
        headers: usize,
        columns: usize,
    },
    ColumnsLengthMismatch {
        column: String,
        expected: usize,
        actual: usize,
    },
    RowLengthMismatch {
        index: usize,
        expected: usize,
        actual: usize,
    },
    CsvError(String),
    IoError(String),
}

impl fmt::Display for DataFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataFrameError::HeadersColumnsLengthMismatch { headers, columns } => {
                write!(
                    f,
                    "Headers and columns have different size: headers={}, columns={}",
                    headers, columns
                )
            }
            DataFrameError::ColumnsLengthMismatch {
                column,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Column '{}' has length {} but expected {}",
                    column, actual, expected
                )
            }
            DataFrameError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
            DataFrameError::CsvError(msg) => {
                write!(f, "CSV error: {}", msg)
            }
            DataFrameError::RowLengthMismatch {
                index,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Row {} has {} columns but expected {} ",
                    index, actual, expected
                )
            }
        }
    }
}

impl Error for DataFrameError {}
