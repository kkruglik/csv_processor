use csv_processor::*;

#[test]
fn test_empty_dataframe_shape() {
    let df = DataFrame::new(None, None);
    assert_eq!(df.shape(), (0, 0));
}

#[test]
fn test_only_headers_dataframe_shape() {
    let headers = Some(vec!["h1".to_string(), "h2".to_string()]);
    let df = DataFrame::new(headers, None);
    assert_eq!(df.shape(), (0, 2));
}

#[test]
#[should_panic]
fn test_headers_and_cols_different_shape() {
    let headers = vec!["h1".to_string(), "h2".to_string()];
    let rows = vec![
        vec!["h1".to_string(), "h2".to_string(), "h3".to_string()],
        vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
    ];
    DataFrame::new(Some(headers), Some(rows));
}

#[test]
fn test_headers_and_cols_correct_shape() {
    let headers = vec!["h1".to_string(), "h2".to_string(), "h3".to_string()];
    let rows = vec![
        vec!["h1".to_string(), "h2".to_string(), "h3".to_string()],
        vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
    ];
    let df = DataFrame::new(Some(headers), Some(rows));
    assert_eq!(df.shape(), (2, 3))
}
