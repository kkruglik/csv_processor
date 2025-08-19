#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Str(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    Date(String),
    Null,
}

impl CellValue {
    pub fn is_null(&self) -> bool {
        matches!(self, CellValue::Null)
    }

    pub fn data_type(&self) -> &'static str {
        match self {
            CellValue::Str(_) => "string",
            CellValue::Float(_) => "float",
            CellValue::Integer(_) => "integer",
            CellValue::Boolean(_) => "boolean",
            CellValue::Date(_) => "date",
            CellValue::Null => "null",
        }
    }
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Str(s) => write!(f, "{}", s),
            CellValue::Float(n) => write!(f, "{}", n),
            CellValue::Integer(n) => write!(f, "{}", n),
            CellValue::Boolean(b) => write!(f, "{}", b),
            CellValue::Date(d) => write!(f, "{}", d),
            CellValue::Null => write!(f, ""),
        }
    }
}
