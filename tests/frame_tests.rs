use csv_processor::frame::DataFrameError;
use csv_processor::*;

#[test]
fn test_empty_dataframe_shape() {
    let df = DataFrame::empty();
    assert_eq!(df.shape(), (0, 0));
}

#[test]
fn test_load_df_from_file() {
    let df = DataFrame::from_csv("samples/sample.csv").unwrap();
    assert_eq!(df.shape(), (10, 8))
}

#[test]
#[should_panic]
fn test_load_df_from_wrong_path() {
    DataFrame::from_csv("samples/not_exists.csv").unwrap();
}

#[test]
fn test_headers_and_cols_correct_shape() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let cols = vec![
        vec!["a1".to_string(), "a2".to_string(), "a3".to_string()],
        vec!["b1".to_string(), "b2".to_string(), "b3".to_string()],
    ];
    let df = DataFrame::new(Some(headers), cols).unwrap();
    assert_eq!(df.shape(), (3, 2))
}

#[test]
fn test_no_headers_correct_shape() {
    let cols = vec![
        vec!["a1".to_string(), "a2".to_string(), "a3".to_string()],
        vec!["b1".to_string(), "b2".to_string(), "b3".to_string()],
    ];
    let df = DataFrame::new(None, cols).unwrap();
    assert_eq!(df.shape(), (3, 2))
}

#[test]
#[should_panic]
fn test_headers_and_cols_wrong_shape() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let cols = vec![
        vec!["a1".to_string(), "a2".to_string(), "a3".to_string()],
        vec!["b1".to_string(), "b2".to_string(), "b3".to_string()],
        vec!["c1".to_string(), "c2".to_string(), "c3".to_string()],
    ];
    DataFrame::new(Some(headers), cols).unwrap();
}

#[test]
#[should_panic]
fn test_cols_wrong_shape() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let cols = vec![
        vec!["a1".to_string(), "a2".to_string(), "a3".to_string()],
        vec!["b1".to_string(), "b2".to_string()],
    ];
    DataFrame::new(Some(headers), cols).unwrap();
}

// Test DataFrame Display formatting
#[test]
fn test_dataframe_display_output() {
    let headers = vec!["name".to_string(), "age".to_string()];
    let cols = vec![
        vec!["Alice".to_string(), "Bob".to_string()],
        vec!["25".to_string(), "30".to_string()],
    ];
    let df = DataFrame::new(Some(headers), cols).unwrap();
    let output = format!("{}", df);
    assert!(output.contains("name"));
    assert!(output.contains("Alice"));
}

// Test column access
#[test]
fn test_get_column_valid_index() {
    let cols = vec![
        vec!["1".to_string(), "2".to_string()],
        vec!["a".to_string(), "b".to_string()],
    ];
    let df = DataFrame::new(None, cols).unwrap();
    assert!(df.get_column(0).is_some());
    assert!(df.get_column(1).is_some());
}

#[test]
fn test_get_column_invalid_index() {
    let cols = vec![vec!["1".to_string(), "2".to_string()]];
    let df = DataFrame::new(None, cols).unwrap();
    assert!(df.get_column(5).is_none());
}

// Test typed column operations
#[test]
fn test_integer_column_statistics() {
    let cols: Vec<Vec<i64>> = vec![vec![1, 2, 3]];
    let df = DataFrame::new(None, cols).unwrap();
    let column = df.get_column(0).unwrap();
    assert_eq!(column.sum(), Some(6.0));
    assert_eq!(column.mean(), Some(2.0));
}

#[test]
fn test_mixed_type_column() {
    let cols = vec![vec!["1".to_string(), "hello".to_string(), "3".to_string()]];
    let df = DataFrame::from_values(None, cols).unwrap();
    let column = df.get_column(0).unwrap();
    // Should be parsed as StringColumn since "hello" can't be parsed as number
    assert_eq!(column.sum(), None);
}

#[test]
fn test_column_with_nulls() {
    let cols = vec![vec![Some(1), Some(2), None, Some(4)]];
    let df = DataFrame::new(None, cols).unwrap();
    let column = df.get_column(0).unwrap();
    assert_eq!(column.null_count(), 1);
    assert_eq!(column.len(), 4);
    assert_eq!(df.shape(), (4, 1));
}

// Test float column with NaN
#[test]
fn test_float_column_with_nan() {
    let cols = vec![vec![
        "1.5".to_string(),
        "NaN".to_string(),
        "2.5".to_string(),
    ]];
    let df = DataFrame::from_values(None, cols).unwrap();
    let column = df.get_column(0).unwrap();
    assert_eq!(column.sum(), Some(4.0)); // Should filter out NaN
}

// Test boolean column
#[test]
fn test_boolean_column_operations() {
    let cols = vec![vec![
        "true".to_string(),
        "false".to_string(),
        "true".to_string(),
    ]];
    let df = DataFrame::from_values(None, cols).unwrap();
    let column = df.get_column(0).unwrap();
    assert_eq!(column.sum(), Some(2.0)); // Count of true values
}

// Test edge case: single row
#[test]
fn test_single_row_dataframe() {
    let cols = vec![vec!["single".to_string()]];
    let df = DataFrame::new(None, cols).unwrap();
    assert_eq!(df.shape(), (1, 1));
}

// Test edge case: empty columns
#[test]
fn test_empty_columns() {
    let cols: Vec<Vec<String>> = vec![vec![], vec![]];
    let df = DataFrame::new(None, cols).unwrap();
    assert_eq!(df.shape(), (0, 2));
}

// Test typed integer column with DataFrame::new()
#[test]
fn test_typed_integer_column() {
    let cols: Vec<Vec<i64>> = vec![vec![1, 2, 3]];
    let df = DataFrame::new(None, cols).unwrap();
    let col = df.get_column(0).unwrap();
    assert_eq!(col.sum(), Some(6.0));
}

// Test typed float column with DataFrame::new()
#[test]
fn test_typed_float_column() {
    let cols = vec![vec![1.5, 2.5, 3.5]];
    let df = DataFrame::new(None, cols).unwrap();
    let col = df.get_column(0).unwrap();
    assert_eq!(col.sum(), Some(7.5));
}

// Test typed boolean column with DataFrame::new()
#[test]
fn test_typed_boolean_column() {
    let cols = vec![vec![true, false, true]];
    let df = DataFrame::new(None, cols).unwrap();
    let col = df.get_column(0).unwrap();
    assert_eq!(col.sum(), Some(2.0)); // Count of true values
}

#[test]
fn test_create_df_from_homog_columns() {
    let col1: Vec<i64> = vec![1, 2, 3];
    let col2: Vec<i64> = vec![4, 5, 6];
    let col3: Vec<i64> = vec![7, 8, 9];
    let col4: Vec<i64> = vec![10, 11, 12];

    let columns: Vec<Box<dyn ColumnArray>> =
        vec![col1.into(), col2.into(), col3.into(), col4.into()];

    let df = DataFrame::from_columns(None, columns).unwrap();

    assert_eq!(df.shape(), (3, 4));
    assert_eq!(df.get_column(0).unwrap().mean().unwrap(), 2.0);
}

#[test]
fn test_create_df_from_heter_columns() {
    let col0: Vec<i64> = vec![1, 2, 3];
    let col1 = vec![4.0, 5.0, 6.0];
    let col2 = vec!["7".to_string(), "8".to_string(), "9".to_string()];
    let col3 = vec![true, false, true];
    let col4 = vec![Some(true), Some(false), None];

    let columns: Vec<Box<dyn ColumnArray>> = vec![
        col0.into(),
        col1.into(),
        col2.into(),
        col3.into(),
        col4.into(),
    ];

    let df = DataFrame::from_columns(None, columns).unwrap();

    assert_eq!(df.shape(), (3, 5));
    assert_eq!(df.get_column(0).unwrap().mean().unwrap(), 2.0);
    assert_eq!(df.get_column(3).unwrap().sum().unwrap(), 2.0);
    assert_eq!(df.get_column(4).unwrap().null_count(), 1);
}

// Tests for DataFrame::from_strings() method
#[test]
fn test_from_strings_basic_csv() {
    let csv_content = "name,age,salary\nAlice,28,75000.5\nBob,35,65000\nCharlie,42,82000";

    let df = DataFrame::from_strings(csv_content).unwrap();

    assert_eq!(df.shape(), (3, 3));
    assert_eq!(df.headers(), &["name", "age", "salary"]);

    // Test first column (name - should be StringColumn)
    let name_col = df.get_column(0).unwrap();
    assert_eq!(name_col.sum(), None); // String columns return None for sum

    // Test second column (age - should be IntegerColumn)
    let age_col = df.get_column(1).unwrap();
    assert_eq!(age_col.sum(), Some(105.0)); // 28 + 35 + 42 = 105
    assert_eq!(age_col.mean(), Some(35.0));

    // Test third column (salary - should be FloatColumn)
    let salary_col = df.get_column(2).unwrap();
    assert_eq!(salary_col.sum(), Some(222000.5)); // 75000.5 + 65000 + 82000
}

#[test]
fn test_from_strings_with_nulls() {
    let csv_content = "id,name,active\n1,Alice,true\n2,,false\n3,Charlie,";

    let df = DataFrame::from_strings(csv_content).unwrap();

    assert_eq!(df.shape(), (3, 3));

    // Check that empty values are handled as nulls
    let name_col = df.get_column(1).unwrap();
    assert_eq!(name_col.null_count(), 1); // One empty name

    let active_col = df.get_column(2).unwrap();
    assert_eq!(active_col.null_count(), 1); // One empty boolean
}

#[test]
fn test_from_strings_boolean_column() {
    let csv_content = "flag,status\ntrue,active\nfalse,inactive\ntrue,active";

    let df = DataFrame::from_strings(csv_content).unwrap();

    assert_eq!(df.shape(), (3, 2));

    // Test boolean column parsing
    let flag_col = df.get_column(0).unwrap();
    assert_eq!(flag_col.sum(), Some(2.0)); // Two true values
    assert_eq!(flag_col.mean(), Some(2.0 / 3.0)); // Proportion of true values
}

#[test]
fn test_from_strings_mixed_types() {
    let csv_content =
        "id,score,name,passed\n1,95.5,Alice,true\n2,87.2,Bob,false\n3,91.8,Charlie,true";

    let df = DataFrame::from_strings(csv_content).unwrap();

    assert_eq!(df.shape(), (3, 4));

    // id column (integer)
    let id_col = df.get_column(0).unwrap();
    assert_eq!(id_col.sum(), Some(6.0)); // 1 + 2 + 3

    // score column (float)
    let score_col = df.get_column(1).unwrap();
    assert_eq!(score_col.sum(), Some(274.5)); // 95.5 + 87.2 + 91.8

    // name column (string)
    let name_col = df.get_column(2).unwrap();
    assert_eq!(name_col.sum(), None); // Strings return None for sum

    // passed column (boolean)
    let passed_col = df.get_column(3).unwrap();
    assert_eq!(passed_col.sum(), Some(2.0)); // Two true values
}

#[test]
fn test_from_strings_single_row() {
    let csv_content = "col1,col2\nvalue1,42";

    let df = DataFrame::from_strings(csv_content).unwrap();

    assert_eq!(df.shape(), (1, 2));
    assert_eq!(df.headers(), &["col1", "col2"]);
}

#[test]
fn test_from_strings_headers_only() {
    let csv_content = "col1,col2,col3";

    let df = DataFrame::from_strings(csv_content).unwrap();

    // When there are only headers, the DataFrame should have 0 rows and 0 columns
    // because no columns are created without data
    assert_eq!(df.shape(), (0, 0));
    assert_eq!(df.headers(), &["col1", "col2", "col3"]);
}

#[test]
fn test_from_strings_malformed_csv() {
    // Row with wrong number of columns
    let csv_content = "col1,col2\nvalue1,value2\nextra_value,missing,column";

    let result = DataFrame::from_strings(csv_content);

    assert!(result.is_err());
    match result {
        Err(DataFrameError::RowLengthMismatch {
            index,
            expected,
            actual,
        }) => {
            assert_eq!(index, 2); // Second data row (1-indexed)
            assert_eq!(expected, 2);
            assert_eq!(actual, 3);
        }
        Err(DataFrameError::CsvError(_)) => {
            // CSV crate might handle this as a parsing error instead
            // This is also acceptable behavior
        }
        other => panic!("Expected RowLengthMismatch or CsvError, got: {:?}", other),
    }
}

#[test]
fn test_from_strings_empty_content() {
    let csv_content = "";

    let result = DataFrame::from_strings(csv_content);

    // CSV crate behavior with empty content - it might succeed or fail
    // Let's test that it handles empty content gracefully
    match result {
        Ok(df) => {
            // Empty content might result in empty dataframe
            assert_eq!(df.shape(), (0, 0));
        }
        Err(DataFrameError::CsvError(_)) => {
            // CSV parsing error is also acceptable for empty content
        }
        other => panic!("Expected success or CsvError, got: {:?}", other),
    }
}
