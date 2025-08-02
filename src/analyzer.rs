use crate::{CellValue, DataFrame, Dtype};

#[derive(Debug)]
struct DatasetInfo {
    pub total_rows: usize,
    pub total_columns: usize,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug)]
pub struct ColumnInfo {
    pub column_name: String,
    pub non_null_count: usize,
    pub dtype: Dtype,
    pub max: Option<CellValue>,
    pub min: Option<CellValue>,
    pub mean: Option<CellValue>,
    pub median: Option<CellValue>,
    pub std: Option<CellValue>,
    pub var: Option<CellValue>,
    pub first: Option<CellValue>,
    pub last: Option<CellValue>,
}

fn calculate_mean(column: &Vec<CellValue>) {}

fn calculate_median(column: &Vec<CellValue>) {}

fn calculate_std(column: &Vec<CellValue>) {}

fn calculate_variance(column: &Vec<CellValue>) {}

fn calculate_max(column: &Vec<CellValue>) {}

fn calculate_min(column: &Vec<CellValue>) {}

pub fn analyze_statistics(data: &DataFrame) {}

pub fn analyze_nans(data: &DataFrame) {}
