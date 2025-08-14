use csv_processor::dataframe::*;

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
    assert_eq!(mean_none, None);
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
