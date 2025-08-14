#[derive(Debug)]
pub enum CsvError {
    IoError(std::io::Error),
    ParseError(csv::Error),
    FileNotFound(String),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Dtype {
    Str,
    Float,
    Integer,
    Boolean,
    Date,
    DateTime,
    Null,
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
