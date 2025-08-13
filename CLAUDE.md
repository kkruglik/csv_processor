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
- `cargo test parser_tests` - Run parser tests

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
User Input → Config → Dataset → AnalysisResult → Formatted Output
```

### Module Structure
- `config.rs` - CLI parsing and user configuration with `Command` enum (CheckNAs, CalculateStatistics)
- `dataframe/` - Core data structures and CSV loading
  - `mod.rs` - `DataFrame` struct with headers, rows, and typed columns
  - `loader.rs` - CSV file loading with `load_dataframe()` function
  - `columns.rs` - Typed column arrays with `ColumnArray` trait and column implementations
  - `aggregation.rs` - `ChunkAgg` trait for column-level statistical operations
- `types.rs` - Core types (`CellValue`, `CsvError`, `Dtype`)
- `analyzer.rs` - High-level analysis functions that orchestrate column aggregations
- `parser.rs` - CSV parsing utilities
- `reporter.rs` - Output formatting

### Key Data Structures
- `Config` - Holds command and filename from CLI args
- `DataFrame` - Main data container with headers, rows, metadata, and typed columns
- `ColumnArray` trait - Polymorphic column storage for different data types
- `ChunkAgg<T>` trait - Defines statistical operations (sum, min, max, mean) for column types
- Concrete column types: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
- Custom error types: `ConfigError`, `CsvError`

### Analysis Architecture

The analyzer follows a trait-based aggregation pattern:

1. **Column-Level Operations**: `ChunkAgg<T>` trait provides statistical operations directly on typed columns
2. **Type-Specific Implementations**: Each column type (`IntegerColumn`, `FloatColumn`, etc.) implements aggregations appropriate for its data type
3. **Analyzer Orchestration**: `analyzer.rs` coordinates column-level aggregations into dataset-wide analysis
4. **Functional Design**: Pure functions that transform column data rather than mutating state

Example flow:
```rust
// Column-level aggregation
let mean_value = integer_column.mean(); // ChunkAgg trait method

// Analyzer orchestrates multiple columns
let analysis = analyze_statistics(&dataframe); // Uses ChunkAgg implementations
```

### Current Implementation Status
- **Foundation & Data Loading**: Complete with typed column system
- **Column Aggregations**: Basic statistical operations implemented for `IntegerColumn`
- **Analysis Orchestration**: In progress - transitioning to trait-based approach
- **Reporting**: Early stages

See `todo.md` for development roadmap.