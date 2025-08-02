# CSV Processor

A command-line tool for CSV data analysis built in Rust.

## Usage

```bash
# Check for missing values
cargo run check_na sample.csv

# Calculate statistics
cargo run calculate_statistics sample.csv
```

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check code
cargo check
```

## Features

- Missing value detection
- Statistical analysis
- Typed column system with automatic type inference
- Fast CSV processing