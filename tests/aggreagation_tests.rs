use csv_processor::dataframe::*;

#[test]
fn test_sum() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(col.sum(), Some(6));
}

#[test]
fn test_max() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(col.max(), Some(3));
}

#[test]
fn test_mean() {
    let col = IntegerColumn(vec![Some(1), Some(2), Some(3)]);
    let mean_float: Option<f64> = col.mean();
    let mean_none: Option<i64> = col.mean();
    assert_eq!(mean_none, None);
    assert_eq!(mean_float, Some(2.0));
}
