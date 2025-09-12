# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run --bin csv_processor -- <command> <filename>` - Run the CLI tool
  - `cargo run --bin csv_processor -- na sample.csv` - Check for missing values in CSV
  - `cargo run --bin csv_processor -- info sample.csv` - Calculate statistics for CSV

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo test config_tests` - Run configuration tests
- `cargo test columns_tests` - Run series/array statistical operation tests
- `cargo test frame_tests` - Run DataFrame functionality tests

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
- `config.rs` - CLI parsing and user configuration with `Command` enum (Na, Info)
- `series/` - Column-oriented data structures (following Polars patterns)
  - `array.rs` - `ColumnArray` trait with statistical operations, type inference, and parsing
  - `mod.rs` - Re-exports for series functionality
- `frame/` - DataFrame operations and I/O
  - `mod.rs` - `DataFrame` struct with Display trait, shape methods, and typed columns
  - `error.rs` - `DataFrameError` enum with proper error handling (Display, Error traits)
  - `io.rs` - CSV file loading with `load_dataframe()` function
- `scalar/` - Cell-level operations and values
  - `mod.rs` - `CellValue` enum and scalar operations
- `types.rs` - Core types (`CsvError`, `Dtype`)
- `reporter.rs` - Statistical report generation (wide and long formats)

**Check these documents and update them when you finish working on a feature:**
@app_design.md - Application Design and main principles we need to achive in development
@todo.md - Current tasks,  progress and development roadmap

### Key Data Structures
- `Config` - Holds command and filename from CLI args
- `DataFrame` - Main data container with optional headers/rows, typed columns, and Display formatting
- `ColumnArray` trait - Unified interface for polymorphic column storage AND statistical operations
- `CellValue` - Enum for individual cell values with type information and utility methods
- Concrete column types: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
- Custom error types: `ConfigError`, `DataFrameError` (with variants: HeadersColumnsLengthMismatch, ColumnsLengthMismatch, RowLengthMismatch, CsvError, IoError, JsonError)
- `JsonExportOrient` enum - JSON export orientations (Columns, Records, Values)

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

// JSON export with multiple orientations
let json_columns = dataframe.to_json(JsonExportOrient::Columns)?;
let json_records = dataframe.to_json(JsonExportOrient::Records)?;
let json_values = dataframe.to_json(JsonExportOrient::Values)?;
```

### Current Implementation Status
- **Foundation & Data Loading**: Complete with typed column system
- **Column System**: Complete with unified `ColumnArray` trait including `is_empty()` method
- **Statistical Operations**: Complete for all column types (`IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`)
  - All types implement: `sum()`, `min()`, `max()`, `mean()` returning `Option<f64>`
  - Proper null handling and edge case management
  - NaN filtering for float operations
  - Boolean mean calculation (proportion of true values)
  - Type conversion traits (`From<Vec<usize>>`, explicit `Vec<i64>` in tests)
- **API Design**: Complete - ergonomic trait object interface
- **Analysis Architecture**: Complete - embedded in column trait system (no separate analyzer needed)
- **Module Architecture**: Complete - reorganized to follow industry patterns (Polars/Arrow style)
- **DataFrame Display**: Complete with formatted table output and proper truncation
- **Statistical Reporting**: Complete with wide and long format report generation
- **Error Handling**: Complete with proper Result types throughout DataFrame operations
  - Custom `DataFrameError` enum with specific error variants
  - Display and Error trait implementations for user-friendly error messages
  - Proper error conversion and propagation using `map_err` and `?` operator
  - Clean module organization with `frame/error.rs` and public re-exports
- **Testing**: Complete with comprehensive test suites for config, columns, and DataFrame functionality (39 tests passing)
  - All statistical operations verified including boolean mean calculations
  - Type conversion and trait implementations tested
  - Compilation issues resolved with explicit integer types
- **Code Quality**: Production-ready with idiomatic Rust patterns and clippy compliance
- **JSON Export**: Complete with multiple orientation support (Columns, Records, Values)
  - Column-level `to_json()` methods for all column types
  - DataFrame-level JSON export with `JsonExportOrient` enum
  - Proper type preservation (integers, floats, booleans, strings, nulls)
  - JsonError variant added to DataFrameError for comprehensive error handling
- **CLI Integration**: Complete with `na` and `info` commands, comprehensive help system
- **Library Architecture**: Clean separation between library (`src/lib.rs`) and binary (`src/bin/csv_processor.rs`)

## Project Status: 🎉 COMPLETE

### **✅ All Core Features Implemented**
The project is now **100% complete** with all major functionality implemented:

1. **CLI Integration**: ✅ **COMPLETED**
   - Full command routing with `na` and `info` commands
   - Comprehensive help system with `--help`, `-h`, and `help` flags
   - Professional error handling and user experience

2. **JSON Export System**: ✅ **COMPLETED**
   - Multiple export orientations (Columns, Records, Values)
   - Column-level and DataFrame-level JSON serialization
   - Proper type preservation and null handling
   - JsonError integration with error handling system

3. **Statistical Engine**: ✅ **COMPLETED**
   - Self-analyzing columns with embedded statistical operations
   - Complete statistical operations for all column types
   - Professional reporting system (wide/long formats)

4. **Library Architecture**: ✅ **COMPLETED**
   - Clean library + binary separation
   - Comprehensive documentation and API design
   - Production-ready code quality

### **📋 Future Enhancements (Optional)**
- Advanced statistical operations (median, mode, variance)
- Extended output formats (CSV export, Parquet support)
- Performance optimizations for very large files

**Note**: All sophisticated architectural work is complete. The project is publication-ready with idiomatic Rust patterns and comprehensive functionality.

- i am a only beginning to learn rust. so dont act like a am senior developer and pay attention to explain rust core, when i ask related questions or dont understand something