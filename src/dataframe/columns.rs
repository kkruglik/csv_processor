use super::ChunkAgg;
use crate::{CellValue, CsvError, Dtype};

pub trait ColumnArray: std::fmt::Debug {
    fn len(&self) -> usize;
    fn dtype(&self) -> Dtype;
    fn get(&self, index: usize) -> Option<CellValue>;
    fn null_count(&self) -> usize;
    fn non_null_count(&self) -> usize {
        self.len() - self.null_count()
    }
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug)]
pub struct IntegerColumn(pub Vec<Option<i64>>);

#[derive(Debug)]
struct FloatColumn(Vec<Option<f64>>);

#[derive(Debug)]
struct StringColumn(Vec<Option<String>>);

#[derive(Debug)]
struct BooleanColumn(Vec<Option<bool>>);

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
}

impl ChunkAgg<i64> for IntegerColumn {
    fn sum(&self) -> Option<i64> {
        Some(self.0.iter().filter_map(|&x| x).sum())
    }

    fn max(&self) -> Option<i64> {
        self.0.iter().filter_map(|&x| x).max()
    }

    fn min(&self) -> Option<i64> {
        self.0.iter().filter_map(|&x| x).min()
    }
}

impl ChunkAgg<f64> for IntegerColumn {
    fn mean(&self) -> Option<f64> {
        let array_len = self.non_null_count();
        if array_len == 0 {
            return Some(0.0);
        }

        let sum: i64 = self.0.iter().filter_map(|&x| x).sum();

        Some(sum as f64 / array_len as f64)
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
