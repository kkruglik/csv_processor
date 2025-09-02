use csv_processor::series::*;
use serde_json::{json, Value};

#[test]
fn test_sum_int() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(col.sum(), Some(6.0));
}

#[test]
fn test_max_int() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(col.max(), Some(3.0));
}

#[test]
fn test_mean_int() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    let mean_float: Option<f64> = col.mean();
    assert_eq!(mean_float, Some(2.0));
}

#[test]
fn test_sum_float() {
    let col = FloatColumn(vec![Some(1.0), Some(2.0), Some(3.0)]);
    assert_eq!(col.sum(), Some(6.0));
}

#[test]
fn test_max_float() {
    let col = FloatColumn(vec![Some(1.0), Some(2.0), Some(3.0)]);
    assert_eq!(col.max(), Some(3.0));
}

#[test]
fn test_min_float() {
    let col = FloatColumn(vec![Some(1.0), Some(2.0), Some(3.0)]);
    assert_eq!(col.min(), Some(1.0));
}

#[test]
fn test_mean_float() {
    let col = FloatColumn(vec![Some(1.0), Some(2.0), Some(3.0)]);
    let mean_float: Option<f64> = col.mean();
    assert_eq!(mean_float, Some(2.0));
}

#[test]
fn test_sum_bool() {
    let col = BooleanColumn(vec![Some(true), Some(false), Some(true)]);
    assert_eq!(col.sum(), Some(2.0));
}

#[test]
fn test_mean_bool() {
    let col = BooleanColumn(vec![Some(true), Some(false), Some(true)]);
    let mean_none = col.mean();
    assert_eq!(mean_none, Some(0.6666666666666666));
}

#[test]
fn test_max_bool() {
    let col = BooleanColumn(vec![Some(true), Some(false), Some(true)]);
    let col2 = BooleanColumn(vec![Some(false), Some(false), Some(false)]);
    let col3 = BooleanColumn(vec![Some(true), Some(true), Some(true)]);

    assert_eq!(col.max(), Some(1.0));
    assert_eq!(col2.max(), Some(0.0));
    assert_eq!(col3.max(), Some(1.0));
}

#[test]
fn test_min_bool() {
    let col = BooleanColumn(vec![Some(true), Some(false), Some(true)]);
    let col2 = BooleanColumn(vec![Some(false), Some(false), Some(false)]);
    let col3 = BooleanColumn(vec![Some(true), Some(true), Some(true)]);

    assert_eq!(col.min(), Some(0.0));
    assert_eq!(col2.min(), Some(0.0));
    assert_eq!(col3.min(), Some(1.0));
}

#[test]
fn test_str_mean() {
    let col = StringColumn(vec![
        Some("apple".to_string()),
        Some("banana".to_string()),
        Some("cherry".to_string()),
    ]);

    assert_eq!(col.mean(), None);
}

#[test]
fn test_str_sum() {
    let col = StringColumn(vec![
        Some("apple".to_string()),
        Some("banana".to_string()),
        Some("cherry".to_string()),
    ]);

    assert_eq!(col.sum(), None);
}

#[test]
fn test_str_max() {
    let col = StringColumn(vec![
        Some("apple".to_string()),
        Some("banana".to_string()),
        Some("cucumber".to_string()),
    ]);

    assert_eq!(col.max(), None);
}

#[test]
fn test_str_min() {
    let col = StringColumn(vec![
        Some("apple".to_string()),
        Some("banana".to_string()),
        Some("cucumber".to_string()),
    ]);

    assert_eq!(col.min(), None);
}

#[test]
fn test_import_string_col_to_json() {
    let col = StringColumn(vec![
        Some("apple".to_string()),
        Some("banana".to_string()),
        Some("cucumber".to_string()),
    ]);

    let result = vec![
        json!("apple".to_string()),
        json!("banana".to_string()),
        json!("cucumber".to_string()),
    ];

    assert_eq!(col.to_json(), result)
}

#[test]
fn test_import_int_col_to_json() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);

    let mut result = Vec::new();
    for i in 1..=5 {
        result.push(json!(i));
    }
    assert_eq!(col.to_json(), result)
}

#[test]
fn test_import_float_col_to_json() {
    let col = FloatColumn(vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)]);

    let mut result = Vec::new();
    for i in 1..=5 {
        result.push(json!(i as f64));
    }
    assert_eq!(col.to_json(), result)
}

#[test]
fn test_import_bool_col_to_json() {
    let col = BooleanColumn(vec![Some(true), Some(false), Some(true)]);
    let result = vec![json!(true), json!(false), json!(true)];

    assert_eq!(col.to_json(), result)
}

#[test]
fn test_import_bool_col_with_nan_to_json() {
    let col = BooleanColumn(vec![None, Some(false), Some(true)]);
    let result = vec![Value::Null, json!(false), json!(true)];

    assert_eq!(col.to_json(), result)
}

#[test]
fn test_import_all_null_col_to_json() {
    let col = FloatColumn(vec![None, None, None]);
    let result = vec![Value::Null, Value::Null, Value::Null];

    assert_eq!(col.to_json(), result)
}
