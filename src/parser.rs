use crate::Config;
use std::fs;

struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    row_count: Option<usize>,
    file_size: Option<usize>,
}

pub fn load_dataset(config: &Config) {
    let content = fs::read_to_string(config.filename()).unwrap();
    println!("Successfuly loaded csv from {}", config.filename());
    println!("{content}");
}
