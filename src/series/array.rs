use crate::{CellValue, Dtype};

pub trait ColumnArray: std::fmt::Debug {
    fn len(&self) -> usize;
    fn dtype(&self) -> Dtype;
    fn get(&self, index: usize) -> Option<CellValue>;
    fn null_count(&self) -> usize;
    fn non_null_count(&self) -> usize {
        self.len() - self.null_count()
    }
    fn as_any(&self) -> &dyn std::any::Any;

    fn mean(&self) -> Option<f64> {
        None
    }
    fn sum(&self) -> Option<f64> {
        None
    }
    fn min(&self) -> Option<f64> {
        None
    }
    fn max(&self) -> Option<f64> {
        None
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct IntegerColumn(pub Vec<Option<i64>>);

#[derive(Debug)]
pub struct FloatColumn(pub Vec<Option<f64>>);

#[derive(Debug)]
pub struct StringColumn(pub Vec<Option<String>>);

#[derive(Debug)]
pub struct BooleanColumn(pub Vec<Option<bool>>);

impl ColumnArray for IntegerColumn {
    fn dtype(&self) -> Dtype {
        Dtype::Integer
    }

    fn get(&self, index: usize) -> Option<CellValue> {
        self.0.get(index)?.map(CellValue::Integer)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn null_count(&self) -> usize {
        self.0.iter().filter(|x| x.is_none()).count()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn sum(&self) -> Option<f64> {
        let sum: i64 = self.0.iter().filter_map(|&x| x).sum();
        Some(sum as f64)
    }

    fn max(&self) -> Option<f64> {
        self.0.iter().filter_map(|&x| x).max().map(|x| x as f64)
    }

    fn min(&self) -> Option<f64> {
        self.0.iter().filter_map(|&x| x).min().map(|x| x as f64)
    }

    fn mean(&self) -> Option<f64> {
        let count = self.non_null_count();
        if count == 0 {
            return None;
        }
        let sum: i64 = self.0.iter().filter_map(|&x| x).sum();
        Some(sum as f64 / count as f64)
    }
}

impl ColumnArray for FloatColumn {
    fn dtype(&self) -> Dtype {
        Dtype::Float
    }

    fn get(&self, index: usize) -> Option<CellValue> {
        self.0.get(index)?.map(CellValue::Float)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn null_count(&self) -> usize {
        self.0.iter().filter(|x| x.is_none()).count()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn sum(&self) -> Option<f64> {
        Some(
            self.0
                .iter()
                .filter_map(|&x| x)
                .filter(|x| !x.is_nan())
                .sum(),
        )
    }

    fn max(&self) -> Option<f64> {
        self.0
            .iter()
            .filter_map(|&x| x) // Remove None
            .filter(|x| !x.is_nan()) // Remove NaN
            .max_by(|a, b| a.partial_cmp(b).unwrap()) // Safe to unwrap now
    }

    fn min(&self) -> Option<f64> {
        self.0
            .iter()
            .filter_map(|&x| x)
            .filter(|x| !x.is_nan())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    fn mean(&self) -> Option<f64> {
        let valid_values: Vec<f64> = self
            .0
            .iter()
            .filter_map(|&x| x)
            .filter(|x| !x.is_nan())
            .collect();

        if valid_values.is_empty() {
            return Some(0.0);
        }

        let sum: f64 = valid_values.iter().sum();
        Some(sum / valid_values.len() as f64)
    }
}

impl ColumnArray for StringColumn {
    fn dtype(&self) -> Dtype {
        Dtype::Str
    }

    fn get(&self, index: usize) -> Option<CellValue> {
        self.0.get(index)?.clone().map(CellValue::Str)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn null_count(&self) -> usize {
        self.0.iter().filter(|x| x.is_none()).count()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ColumnArray for BooleanColumn {
    fn dtype(&self) -> Dtype {
        Dtype::Boolean
    }

    fn get(&self, index: usize) -> Option<CellValue> {
        self.0.get(index)?.map(CellValue::Boolean)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn null_count(&self) -> usize {
        self.0.iter().filter(|x| x.is_none()).count()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn sum(&self) -> Option<f64> {
        Some(self.0.iter().filter_map(|&x| x).filter(|&x| x).count() as f64)
    }

    fn min(&self) -> Option<f64> {
        if self.non_null_count() == 0 {
            return Some(0.0);
        }
        // let has_false = self.0.iter().any(|&x| x == Some(false));
        let has_false = self.0.contains(&Some(false));
        Some(if has_false { 0.0 } else { 1.0 })
    }

    fn max(&self) -> Option<f64> {
        if self.non_null_count() == 0 {
            return Some(0.0);
        }
        // let has_true = self.0.iter().any(|&x| x == Some(true));
        let has_true = self.0.contains(&Some(true));
        Some(if has_true { 1.0 } else { 0.0 })
    }
}

pub fn parse_column(column: Vec<&str>) -> Box<dyn ColumnArray> {
    if let Some(array) = parse_integers(&column) {
        return Box::new(IntegerColumn(array));
    }

    if let Some(array) = parse_floats(&column) {
        return Box::new(FloatColumn(array));
    }

    if let Some(array) = parse_bools(&column) {
        return Box::new(BooleanColumn(array));
    }

    Box::new(StringColumn(parse_strings(&column)))
}

fn parse_integers(raw_data: &[&str]) -> Option<Vec<Option<i64>>> {
    let mut result = Vec::new();

    for cell in raw_data {
        if cell.is_empty() || cell.to_lowercase() == "na" {
            result.push(None);
        } else {
            match cell.parse::<i64>() {
                Ok(val) => result.push(Some(val)),
                Err(_) => return None,
            }
        }
    }
    Some(result)
}

fn parse_floats(raw_data: &[&str]) -> Option<Vec<Option<f64>>> {
    let mut result = Vec::new();

    for cell in raw_data {
        if cell.is_empty() || cell.to_lowercase() == "na" {
            result.push(None);
        } else {
            match cell.parse::<f64>() {
                Ok(val) => result.push(Some(val)),
                Err(_) => return None,
            }
        }
    }
    Some(result)
}

fn parse_strings(raw_data: &[&str]) -> Vec<Option<String>> {
    raw_data
        .iter()
        .map(|x| {
            if x.is_empty() || x.to_lowercase() == "na" {
                None
            } else {
                Some(x.to_string())
            }
        })
        .collect()
}

fn parse_bools(raw_data: &[&str]) -> Option<Vec<Option<bool>>> {
    let mut result = Vec::new();

    for cell in raw_data {
        if cell.is_empty() || cell.to_lowercase() == "na" {
            result.push(None);
        } else {
            match cell.to_lowercase().as_str() {
                "true" | "1" | "yes" => result.push(Some(true)),
                "false" | "0" | "no" => result.push(Some(false)),
                _ => return None, // If ANY value fails, not a boolean column
            }
        }
    }
    Some(result)
}

impl From<Vec<i64>> for Box<dyn ColumnArray> {
    fn from(data: Vec<i64>) -> Self {
        let data: Vec<Option<i64>> = data.into_iter().map(Some).collect();
        Box::new(IntegerColumn(data))
    }
}

// Vec<Option<i64>> -> IntegerColumn
impl From<Vec<Option<i64>>> for Box<dyn ColumnArray> {
    fn from(data: Vec<Option<i64>>) -> Self {
        Box::new(IntegerColumn(data))
    }
}

// Vec<f64> -> FloatColumn
impl From<Vec<f64>> for Box<dyn ColumnArray> {
    fn from(data: Vec<f64>) -> Self {
        let data: Vec<Option<f64>> = data.into_iter().map(Some).collect();
        Box::new(FloatColumn(data))
    }
}

impl From<Vec<String>> for Box<dyn ColumnArray> {
    fn from(data: Vec<String>) -> Self {
        let data: Vec<Option<String>> = data.into_iter().map(Some).collect();
        Box::new(StringColumn(data))
    }
}

impl From<&[i64]> for Box<dyn ColumnArray> {
    fn from(data: &[i64]) -> Self {
        Vec::from(data).into()
    }
}

impl From<Vec<Option<f64>>> for Box<dyn ColumnArray> {
    fn from(data: Vec<Option<f64>>) -> Self {
        Box::new(FloatColumn(data))
    }
}

impl From<Vec<Option<String>>> for Box<dyn ColumnArray> {
    fn from(data: Vec<Option<String>>) -> Self {
        Box::new(StringColumn(data))
    }
}

// Vec<bool> -> BooleanColumn
impl From<Vec<bool>> for Box<dyn ColumnArray> {
    fn from(data: Vec<bool>) -> Self {
        let data: Vec<Option<bool>> = data.into_iter().map(Some).collect();
        Box::new(BooleanColumn(data))
    }
}

// Vec<Option<bool>> -> BooleanColumn
impl From<Vec<Option<bool>>> for Box<dyn ColumnArray> {
    fn from(data: Vec<Option<bool>>) -> Self {
        Box::new(BooleanColumn(data))
    }
}
