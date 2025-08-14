# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run <command> <filename>` - Run the CLI tool
  - `cargo run check_na sample.csv` - Check for missing values in CSV
  - `cargo run calculate_statistics sample.csv` - Calculate statistics for CSV

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo test config_tests` - Run configuration tests
- `cargo test columns_tests` - Run series/array statistical operation tests

### Development
- `cargo check` - Check code without building
- `cargo clippy` - Run linter (if installed)
- `cargo fmt` - Format code (if installed)

## Architecture

### Core Design Principles
- **Functional design**: Data structures + pure functions over object-oriented patterns
- **Single responsibility**: Each module handles one concern
- **Immutable data flow**: Transform data rather than mutate state
- **Rust idioms**: Leverage ownership system and error handling

### Data Flow
```
User Input → Config → DataFrame (self-analyzing columns) → Formatted Output
```

### Module Structure
- `config.rs` - CLI parsing and user configuration with `Command` enum (CheckNAs, CalculateStatistics)
- `series/` - Column-oriented data structures (following Polars patterns)
  - `array.rs` - `ColumnArray` trait with statistical operations, type inference, and parsing
  - `mod.rs` - Re-exports for series functionality
- `frame/` - DataFrame operations and I/O
  - `mod.rs` - `DataFrame` struct with headers, rows, metadata, and typed columns
  - `io.rs` - CSV file loading with `load_dataframe()` function
- `scalar/` - Cell-level operations and values
  - `mod.rs` - `CellValue` enum and scalar operations
- `types.rs` - Core types (`CsvError`, `Dtype`)
- `reporter.rs` - Output formatting

**Check these documents and update them when you finish working on a feature:**
@app_design.md - Application Design and main principles we need to achive in development
@todo.md - Current tasks,  progress and development roadmap

### Key Data Structures
- `Config` - Holds command and filename from CLI args
- `DataFrame` - Main data container with headers, rows, metadata, and typed columns
- `ColumnArray` trait - Unified interface for polymorphic column storage AND statistical operations
- `CellValue` - Enum for individual cell values with type information
- Concrete column types: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
- Custom error types: `ConfigError`, `CsvError`

### Analysis Architecture

Analysis is now **embedded directly in the column system** - no separate analyzer needed:

1. **Self-Analyzing Columns**: Each column type implements its own statistical operations
2. **Unified Interface**: `ColumnArray` trait provides both data access AND complete analysis
3. **Polymorphic Operations**: All statistical methods return `Option<f64>` for consistency
4. **Type-Specific Logic**: Each column type implements operations appropriate for its data type
5. **Ergonomic API**: Direct method calls on trait objects without complex downcasting
6. **No Orchestration Layer**: Analysis happens at the column level, aggregated at DataFrame level

Example flow:
```rust
// Direct statistical operations on any column type
let column: &dyn ColumnArray = dataframe.get_column(0).unwrap();
let mean_value = column.mean();    // Column analyzes itself
let sum_value = column.sum();      // No external analyzer needed
let null_count = column.null_count(); // Built into the column

// DataFrame-level analysis is just iteration over self-analyzing columns
for (i, column) in dataframe.columns().iter().enumerate() {
    println!("Column {}: mean={:?}, nulls={}", i, column.mean(), column.null_count());
}
```

### Current Implementation Status
- **Foundation & Data Loading**: Complete with typed column system
- **Column System**: Complete with unified `ColumnArray` trait
- **Statistical Operations**: Complete for `IntegerColumn`, `FloatColumn`, and `BooleanColumn`
  - All types implement: `sum()`, `min()`, `max()`, `mean()` returning `Option<f64>`
  - Proper null handling and edge case management
  - NaN filtering for float operations
- **API Design**: Complete - ergonomic trait object interface
- **Analysis Architecture**: Complete - embedded in column trait system (no separate analyzer needed)
- **Module Architecture**: Complete - reorganized to follow industry patterns (Polars/Arrow style)
- **Reporting**: Early stages
