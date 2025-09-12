# CSV Processor

A high-performance **Rust library and CLI tool** for CSV data analysis, featuring automatic type inference, statistical analysis, and professional reporting capabilities.

## 📦 Library + CLI Tool

This project provides both:
- **📚 Rust Library** - For embedding CSV analysis in your applications
- **🖥️ CLI Tool** - For command-line data analysis

## Features

- **Automatic Type Inference**: Intelligently detects integers, floats, booleans, and strings
- **Missing Value Analysis**: Comprehensive NA/null detection and reporting
- **Statistical Operations**: Built-in sum, mean, min, max calculations for all numeric types
- **JSON Export**: Native JSON serialization with multiple orientations (Columns, Records, Values)
- **Professional Output**: Formatted tables and statistical reports
- **Fast Processing**: Rust-powered performance for large CSV files
- **Self-Analyzing Columns**: Each column type implements its own statistical operations
- **Comprehensive Testing**: 37+ tests ensuring reliability

## Installation

### As a Library
Add to your `Cargo.toml`:
```toml
[dependencies]
csv_processor = "0.1.0"
```

### As a CLI Tool
```bash
cargo install csv_processor

# Or build from source
git clone https://github.com/kkruglik/csv_processor
cd csv_processor
cargo build --release
```

## Usage

### 📚 Library Usage

```rust
use csv_processor::{DataFrame, JsonExportOrient, reporter::{generate_info_report, generate_na_report}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load CSV file
    let df = DataFrame::from_csv("data.csv")?;
    
    // Generate statistical report
    let stats_report = generate_info_report(&df);
    println!("Statistics:\n{}", stats_report);
    
    // Generate NA analysis report
    let na_report = generate_na_report(&df);
    println!("Missing Values:\n{}", na_report);
    
    // Export to JSON with different orientations
    let json_columns = df.to_json(JsonExportOrient::Columns)?;
    println!("JSON (Columns): {}", json_columns);
    
    let json_records = df.to_json(JsonExportOrient::Records)?;
    println!("JSON (Records): {}", json_records);
    
    let json_values = df.to_json(JsonExportOrient::Values)?;
    println!("JSON (Values): {}", json_values);
    
    // Access individual columns for custom analysis
    if let Some(column) = df.get_column(0) {
        println!("Column mean: {:?}", column.mean());
        println!("Column nulls: {}", column.null_count());
        let column_json = column.to_json();
        println!("Column as JSON: {:?}", column_json);
    }
    
    Ok(())
}
```

### 🖥️ CLI Usage

```bash
# Check for missing values
csv_processor na sample.csv

# Calculate comprehensive statistics  
csv_processor info sample.csv

# Get help
csv_processor --help
```

**Development Usage:**
```bash
# When developing/building from source
cargo run --bin csv_processor -- na sample.csv
cargo run --bin csv_processor -- info sample.csv
```

## Sample Output

### DataFrame Display
When loading a CSV file, data is displayed in a formatted table:

```
┌─────────────────┬──────────┬─────────┬────────────┬─────────────┬────────┬────────────┬───────┐
│      name       │   age    │ salary  │ department │   active    │ score  │    ...     │  ...  │
├─────────────────┼──────────┼─────────┼────────────┼─────────────┼────────┼────────────┼───────┤
│   Alice Smith   │    28    │ 75000.5 │Engineering │    true     │  8.7   │    ...     │  ...  │
│   Bob Johnson   │   null   │  65000  │ Marketing  │   false     │ null   │    ...     │  ...  │
│   Carol Davis   │    35    │  null   │Engineering │    true     │  9.2   │    ...     │  ...  │
│      null       │    29    │58000.75 │   Sales    │    true     │  7.8   │    ...     │  ...  │
│                 ⋮         │    ⋮    │    ⋮    │     ⋮      │      ⋮      │   ⋮    │     ⋮      │   ⋮   │
│  Henry Taylor   │    38    │  82000  │Engineering │   false     │  7.5   │    ...     │  ...  │
└─────────────────┴──────────┴─────────┴────────────┴─────────────┴────────┴────────────┴───────┘
10 rows × 8 columns
```

### Statistical Report (Wide Format)
```
┌────────────┬──────────┬─────────────┬───────────┬─────────────┐
│   column   │   mean   │     sum     │    min    │     max     │
├────────────┼──────────┼─────────────┼───────────┼─────────────┤
│     id     │   5.5    │    55.0     │    1.0    │    10.0     │
│    age     │  31.29   │   250.33    │   26.0    │    42.0     │
│   salary   │ 72571.5  │  507000.5   │  58000.75 │   95000.0   │
│  active    │   0.8    │     8.0     │    0.0    │     1.0     │
│   score    │   8.06   │    56.4     │    6.9    │     9.2     │
└────────────┴──────────┴─────────────┴───────────┴─────────────┘
5 rows × 5 columns
```

### Missing Value Analysis
```
Column Analysis:
- id: 0 missing values (0.0%)
- name: 2 missing values (20.0%)
- age: 2 missing values (20.0%)
- salary: 3 missing values (30.0%)
- department: 1 missing values (10.0%)
- active: 1 missing values (10.0%)
- start_date: 2 missing values (20.0%)
- score: 3 missing values (30.0%)
```

### JSON Export Formats

The library supports three JSON export orientations:

**Columns Format (Analytics-Optimized)**
```json
{
  "headers": ["id", "name", "age", "salary", "active"],
  "columns": [
    [1, 2, 3, 4, 5],
    ["Alice", "Bob", null, "David", "Emma"],
    [28, 35, null, 42, 31],
    [75000.5, 65000, null, 82000, 71500],
    [true, false, true, false, true]
  ]
}
```

**Records Format (Row-Oriented)**
```json
[
  {"id": 1, "name": "Alice", "age": 28, "salary": 75000.5, "active": true},
  {"id": 2, "name": "Bob", "age": 35, "salary": 65000, "active": false},
  {"id": 3, "name": null, "age": null, "salary": null, "active": true}
]
```

**Values Format (Indexed)**
```json
[
  {"0": 1, "1": "Alice", "2": 28, "3": 75000.5, "4": true},
  {"0": 2, "1": "Bob", "2": 35, "3": 65000, "4": false},
  {"0": 3, "1": null, "2": null, "3": null, "4": true}
]
```

## API Reference

### Core Types

```rust
use csv_processor::{DataFrame, ColumnArray, CellValue, JsonExportOrient, reporter};

// Main data container
let df = DataFrame::from_csv("data.csv")?;

// Access columns polymorphically  
let column: &dyn ColumnArray = df.get_column(0).unwrap();

// Statistical operations (all return Option<f64>)
let mean = column.mean();
let sum = column.sum();
let min = column.min();
let max = column.max();
let nulls = column.null_count();

// JSON export with multiple orientations
let json_columns = df.to_json(JsonExportOrient::Columns)?;
let json_records = df.to_json(JsonExportOrient::Records)?;
let json_values = df.to_json(JsonExportOrient::Values)?;
let column_json = column.to_json();

// Generate reports
let stats_report = reporter::generate_info_report(&df);
let na_report = reporter::generate_na_report(&df);
```

### Key Traits

- `ColumnArray` - Unified interface for column data, statistical operations, and JSON export
- `Display` - Formatted output for DataFrames and reports

## Architecture

### Library + Binary Structure
```
src/
├── lib.rs              # Library interface with documentation
├── bin/
│   └── csv_processor.rs # CLI binary
├── series/             # Column-oriented data structures (Polars-style)
│   └── array.rs        # ColumnArray trait with statistical operations
├── frame/              # DataFrame operations and CSV I/O
│   └── mod.rs          # Main DataFrame implementation  
├── scalar/             # Cell-level operations and values
├── reporter.rs         # Statistical report generation
└── config.rs           # CLI parsing (exported for advanced use)
```

### Core Design Principles
- **Library First**: Clean API for embedding in applications
- **Self-Analyzing Columns**: Statistical operations embedded in column types
- **Functional Design**: Pure functions over object-oriented patterns  
- **Rust Idioms**: Leverage ownership system and proper error handling

### Key Data Types
- **DataFrame**: Main container with typed columns and display formatting
- **ColumnArray**: Unified trait for data access AND statistical operations
- **Column Types**: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
- **CellValue**: Enum for individual cell values with type information

## Development

```bash
# Build the project
cargo build

# Run all tests (37+ test suite)
cargo test

# Run specific test suite
cargo test frame_tests
cargo test columns_tests

# Check code quality
cargo clippy

# Format code
cargo fmt

# Check without building
cargo check
```

## Performance

- **Fast Type Inference**: Automatic detection of optimal column types
- **Memory Efficient**: Column-oriented storage following Apache Arrow patterns
- **Zero-Cost Abstractions**: Rust's performance with high-level ergonomics
- **Parallel Processing Ready**: Architecture designed for future parallelization

## Examples

### Sample CSV Structure
The tool handles various data types and missing values:
```csv
id,name,age,salary,department,active,start_date,score
1,Alice Smith,28,75000.50,Engineering,true,2021-03-15,8.7
2,Bob Johnson,,65000,Marketing,false,2020-11-22,
3,Carol Davis,35,NA,Engineering,true,,9.2
```

### Usage Examples

**CLI Usage:**
```bash
# Analyze missing values
csv_processor na employee_data.csv

# Generate statistical report (includes JSON export demonstration)
csv_processor info sales_data.csv

# For development (building from source)
cargo run --bin csv_processor -- na employee_data.csv
```

**Library Usage:**
```rust
use csv_processor::{DataFrame, JsonExportOrient, reporter::generate_info_report};

let df = DataFrame::from_csv("sales_data.csv")?;
let report = generate_info_report(&df);
println!("{}", report);

// Export to different JSON formats
let json_columns = df.to_json(JsonExportOrient::Columns)?;
let json_records = df.to_json(JsonExportOrient::Records)?;
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes
4. Run the test suite (`cargo test`)
5. Ensure code quality (`cargo clippy`)
6. Commit your changes (`git commit -am 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
