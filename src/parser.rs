struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    row_count: Option<usize>,
    file_size: Option<usize>,
}
