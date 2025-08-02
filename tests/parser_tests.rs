// use csv_processor::config::*;
// use csv_processor::parser::*;

// #[test]
// fn test_loading_dataset() {
//     let config = Config::new(Command::CheckNAs, "sample.csv".to_string());
//     let dataset = load_dataset(&config).unwrap();
//     assert_eq!(dataset.col_count(), 8);
//     assert_eq!(dataset.file_size(), 519);
//     assert_eq!(dataset.row_count(), 10);
// }

// #[test]
// fn test_load_dataset_csv_error() {
//     let config = Config::new(Command::CheckNAs, "not_exist.csv".to_string());
//     let dataset = load_dataset(&config);
//     match dataset {
//         Err(CsvError::ParseError(_)) => {}
//         _ => panic!("Expected ParseError error!"),
//     }
// }
