use crate::ColumnArray;
use crate::DataFrame;

pub fn generate_info_report(df: &DataFrame) -> DataFrame {
    let original_headers = df.headers();

    // Include column names as the first column
    let headers = vec![
        "column".to_string(),
        "mean".to_string(),
        "sum".to_string(),
        "min".to_string(),
        "max".to_string(),
        "null_count".to_string(),
        "dtype".to_string(),
    ];

    let mut column_names: Vec<Option<String>> = Vec::new();
    let mut mean_col: Vec<Option<f64>> = Vec::new();
    let mut sum_col: Vec<Option<f64>> = Vec::new();
    let mut min_col: Vec<Option<f64>> = Vec::new();
    let mut max_col: Vec<Option<f64>> = Vec::new();
    let mut null_count_col: Vec<usize> = Vec::new();
    let mut dtype_col: Vec<String> = Vec::new();

    for (col_idx, header) in original_headers.iter().enumerate() {
        if let Some(column) = df.get_column(col_idx) {
            column_names.push(Some(header.clone())); // Original column name
            mean_col.push(column.mean()); // Statistics
            sum_col.push(column.sum());
            min_col.push(column.min());
            max_col.push(column.max());
            null_count_col.push(column.null_count());
            dtype_col.push(format!("{:?}", column.dtype()));
        }
    }

    let columns: Vec<Box<dyn ColumnArray>> = vec![
        column_names.into(),   // Vec<Option<String>> -> Box<dyn ColumnArray>
        mean_col.into(),       // Vec<Option<f64>> -> Box<dyn ColumnArray>
        sum_col.into(),        // Vec<Option<f64>> -> Box<dyn ColumnArray>
        min_col.into(),        // Vec<Option<f64>> -> Box<dyn ColumnArray>
        max_col.into(),        // Vec<Option<f64>> -> Box<dyn ColumnArray>
        null_count_col.into(), // Vec<Option<i64>> -> Box<dyn ColumnArray>
        dtype_col.into(),
    ];

    DataFrame::new(Some(headers), columns).unwrap()
}

pub fn generate_na_report(df: &DataFrame) -> DataFrame {
    let original_headers = df.headers();

    // Include column names as the first column
    let mut column_names: Vec<Option<String>> = Vec::new();
    let headers = vec!["column".to_string(), "null_count".to_string()];
    let mut null_count_col: Vec<usize> = Vec::new();

    for (col_idx, header) in original_headers.iter().enumerate() {
        if let Some(column) = df.get_column(col_idx) {
            column_names.push(Some(header.clone())); // Original column name
            null_count_col.push(column.null_count());
        }
    }

    let columns: Vec<Box<dyn ColumnArray>> = vec![
        column_names.into(),   // Vec<Option<String>> -> Box<dyn ColumnArray>
        null_count_col.into(), // Vec<Option<i64>> -> Box<dyn ColumnArray>
    ];

    DataFrame::new(Some(headers), columns).unwrap()
}
