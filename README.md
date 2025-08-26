# CSV Processor

A high-performance command-line tool for CSV data analysis built in Rust, featuring automatic type inference, statistical analysis, and professional reporting capabilities.

## Features

- **Automatic Type Inference**: Intelligently detects integers, floats, booleans, and strings
- **Missing Value Analysis**: Comprehensive NA/null detection and reporting
- **Statistical Operations**: Built-in sum, mean, min, max calculations for all numeric types
- **Professional Output**: Formatted tables and statistical reports
- **Fast Processing**: Rust-powered performance for large CSV files
- **Self-Analyzing Columns**: Each column type implements its own statistical operations
- **Comprehensive Testing**: 37+ tests ensuring reliability

## Installation

```bash
git clone https://github.com/kkruglik/csv_processor
cd csv_processor
cargo build --release
```

## Usage

```bash
# Check for missing values
cargo run na sample.csv

# Calculate comprehensive statistics
cargo run info sample.csv

# Run with release build for better performance
cargo run --release info large_file.csv
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

## Architecture

### Core Design Principles
- **Functional Design**: Pure functions over object-oriented patterns
- **Self-Analyzing Columns**: Statistical operations embedded in column types
- **Immutable Data Flow**: Transform data rather than mutate state
- **Rust Idioms**: Leverage ownership system and proper error handling

### Module Structure
```
src/
├── config.rs        # CLI parsing and configuration
├── series/          # Column-oriented data structures (Polars-style)
│   └── array.rs     # ColumnArray trait with statistical operations
├── frame/           # DataFrame operations and CSV I/O
│   └── mod.rs       # Main DataFrame implementation
├── scalar/          # Cell-level operations and values
├── reporter.rs      # Statistical report generation
└── main.rs          # CLI entry point
```

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

### Command Examples
```bash
# Analyze missing values
cargo run na employee_data.csv

# Generate statistical report
cargo run info sales_data.csv

# Process large files with release build
cargo run --release info large_dataset.csv
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
