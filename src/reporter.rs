use crate::DataFrame;

pub fn generate_wide_report(df: &DataFrame) -> DataFrame {
    let mut headers = vec!["Metric".to_string()];

    // df.headers() returns &[String], not Option
    let df_headers = df.headers();
    if !df_headers.is_empty() {
        for header in df_headers {
            headers.push(header.clone());
        }
    } else {
        // Fallback to generic names
        for i in 0..df.shape().1 {
            headers.push(format!("Column_{}", i));
        }
    }

    let mut rows = vec![];

    // Mean row
    let mut mean_row = vec!["mean".to_string()];
    for i in 0..df.shape().1 {
        let mean_val = df.get_column(i).unwrap().mean();
        mean_row.push(match mean_val {
            Some(val) => format!("{:.2}", val),
            None => "N/A".to_string(),
        });
    }
    rows.push(mean_row);

    // Max row
    let mut max_row = vec!["max".to_string()];
    for i in 0..df.shape().1 {
        let max_val = df.get_column(i).unwrap().max();
        max_row.push(match max_val {
            Some(val) => format!("{:.2}", val),
            None => "N/A".to_string(),
        });
    }
    rows.push(max_row);

    // Min row
    let mut min_row = vec!["min".to_string()];
    for i in 0..df.shape().1 {
        let min_val = df.get_column(i).unwrap().min();
        min_row.push(match min_val {
            Some(val) => format!("{:.2}", val),
            None => "N/A".to_string(),
        });
    }
    rows.push(min_row);

    // Sum row
    let mut sum_row = vec!["sum".to_string()];
    for i in 0..df.shape().1 {
        let sum_val = df.get_column(i).unwrap().sum();
        sum_row.push(match sum_val {
            Some(val) => format!("{:.2}", val),
            None => "N/A".to_string(),
        });
    }
    rows.push(sum_row);

    DataFrame::new(Some(headers), Some(rows))
}

pub fn generate_long_report(df: &DataFrame) -> DataFrame {
    let headers = vec![
        "Column".to_string(),
        "Metric".to_string(),
        "Value".to_string(),
    ];

    let mut rows = vec![];

    for i in 0..df.shape().1 {
        let col_name = {
            let df_headers = df.headers();
            if !df_headers.is_empty() && i < df_headers.len() {
                df_headers[i].clone()
            } else {
                format!("Column_{}", i)
            }
        };

        let column = df.get_column(i).unwrap();

        // Mean
        if let Some(mean) = column.mean() {
            rows.push(vec![
                col_name.clone(),
                "mean".to_string(),
                format!("{:.2}", mean),
            ]);
        } else {
            rows.push(vec![
                col_name.clone(),
                "mean".to_string(),
                "N/A".to_string(),
            ]);
        }

        // Max
        if let Some(max) = column.max() {
            rows.push(vec![
                col_name.clone(),
                "max".to_string(),
                format!("{:.2}", max),
            ]);
        } else {
            rows.push(vec![col_name.clone(), "max".to_string(), "N/A".to_string()]);
        }

        // Min
        if let Some(min) = column.min() {
            rows.push(vec![
                col_name.clone(),
                "min".to_string(),
                format!("{:.2}", min),
            ]);
        } else {
            rows.push(vec![col_name.clone(), "min".to_string(), "N/A".to_string()]);
        }

        // Sum
        if let Some(sum) = column.sum() {
            rows.push(vec![
                col_name.clone(),
                "sum".to_string(),
                format!("{:.2}", sum),
            ]);
        } else {
            rows.push(vec![col_name.clone(), "sum".to_string(), "N/A".to_string()]);
        }
    }

    DataFrame::new(Some(headers), Some(rows))
}
