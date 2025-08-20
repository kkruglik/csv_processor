use crate::ColumnArray;
use crate::DataFrame;

pub fn generate_wide_report(df: &DataFrame) -> DataFrame {
    let original_headers = df.headers();

    // Include column names as the first column
    let headers = vec![
        "column".to_string(),
        "mean".to_string(),
        "sum".to_string(),
        "min".to_string(),
        "max".to_string(),
    ];

    let mut column_names: Vec<Option<String>> = Vec::new();
    let mut mean_col: Vec<Option<f64>> = Vec::new();
    let mut sum_col: Vec<Option<f64>> = Vec::new();
    let mut min_col: Vec<Option<f64>> = Vec::new();
    let mut max_col: Vec<Option<f64>> = Vec::new();

    for (col_idx, header) in original_headers.iter().enumerate() {
        if let Some(column) = df.get_column(col_idx) {
            column_names.push(Some(header.clone())); // Original column name
            mean_col.push(column.mean()); // Statistics
            sum_col.push(column.sum());
            min_col.push(column.min());
            max_col.push(column.max());
        }
    }

    let columns: Vec<Box<dyn ColumnArray>> = vec![
        column_names.into(), // Vec<Option<String>> -> Box<dyn ColumnArray>
        mean_col.into(),     // Vec<Option<f64>> -> Box<dyn ColumnArray>
        sum_col.into(),      // Vec<Option<f64>> -> Box<dyn ColumnArray>
        min_col.into(),      // Vec<Option<f64>> -> Box<dyn ColumnArray>
        max_col.into(),      // Vec<Option<f64>> -> Box<dyn ColumnArray>
    ];

    DataFrame::new(Some(headers), columns).unwrap()
}

// pub fn generate_long_report(df: &DataFrame) -> DataFrame {
//     let headers = vec![
//         "Column".to_string(),
//         "Metric".to_string(),
//         "Value".to_string(),
//     ];

//     let mut rows = vec![];

//     for i in 0..df.shape().1 {
//         let col_name = {
//             let df_headers = df.headers();
//             if !df_headers.is_empty() && i < df_headers.len() {
//                 df_headers[i].clone()
//             } else {
//                 format!("Column_{}", i)
//             }
//         };

//         let column = df.get_column(i).unwrap();

//         // Mean
//         if let Some(mean) = column.mean() {
//             rows.push(vec![
//                 col_name.clone(),
//                 "mean".to_string(),
//                 format!("{:.2}", mean),
//             ]);
//         } else {
//             rows.push(vec![
//                 col_name.clone(),
//                 "mean".to_string(),
//                 "N/A".to_string(),
//             ]);
//         }

//         // Max
//         if let Some(max) = column.max() {
//             rows.push(vec![
//                 col_name.clone(),
//                 "max".to_string(),
//                 format!("{:.2}", max),
//             ]);
//         } else {
//             rows.push(vec![col_name.clone(), "max".to_string(), "N/A".to_string()]);
//         }

//         // Min
//         if let Some(min) = column.min() {
//             rows.push(vec![
//                 col_name.clone(),
//                 "min".to_string(),
//                 format!("{:.2}", min),
//             ]);
//         } else {
//             rows.push(vec![col_name.clone(), "min".to_string(), "N/A".to_string()]);
//         }

//         // Sum
//         if let Some(sum) = column.sum() {
//             rows.push(vec![
//                 col_name.clone(),
//                 "sum".to_string(),
//                 format!("{:.2}", sum),
//             ]);
//         } else {
//             rows.push(vec![col_name.clone(), "sum".to_string(), "N/A".to_string()]);
//         }
//     }

//     DataFrame::new(Some(headers), Some(rows))
// }
