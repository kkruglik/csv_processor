# CSV Analytics Tool - Development Roadmap

## Phase 1: Foundation ✅

- [x] Project setup with Cargo
- [x] CLI argument parsing (`config.rs`)
- [x] Custom error types
- [x] Basic test framework
- [x] Module structure planning

## Phase 2: Data Loading ✅ (Complete)

- [x] Create `DataFrame` struct (`frame/mod.rs`)
- [x] Implement CSV file reading with `csv` crate (`frame/io.rs`)
- [x] Add CSV parsing error handling (`types.rs` - `CsvError`)
- [x] Implement typed column system (`series/array.rs`)
  - [x] `ColumnArray` trait for polymorphic columns
  - [x] Typed columns: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
  - [x] Automatic type inference and parsing
- [x] Basic data validation and null handling
- [x] Unit tests for column operations (`tests/columns_tests.rs`)
- [x] Unit tests for DataFrame functionality (`tests/frame_tests.rs`)

## Phase 3: NA Analysis 🔄 → **MOVED TO CURRENT PRIORITY (Phase 6B)**

*This phase has been reorganized into current priority Phase 6B above*

## Phase 4: Output Formatting ✅ (Complete)

- [x] Create statistical reporter (`reporter.rs`)
- [x] Implement DataFrame Display trait with formatted table output
- [x] Add wide format statistical reports
- [x] Add long format statistical reports
- [x] Implement proper column width handling and truncation
- [ ] Format NA analysis results → **MOVED TO PHASE 6B**
- [ ] CLI integration and testing → **MOVED TO PHASE 6A**

## Phase 5: Statistics Analysis ✅ (Unified ColumnArray Approach)

- [x] **ARCHITECTURAL SHIFT**: Consolidated statistical operations into `ColumnArray` trait
- [x] Removed separate `ChunkAgg<T>` trait in favor of unified interface
- [x] Implement comprehensive statistical operations:
  - [x] `IntegerColumn`: `sum()`, `min()`, `max()`, `mean()` with proper null handling
  - [x] `FloatColumn`: `sum()`, `min()`, `max()`, `mean()` with NaN filtering
  - [x] `BooleanColumn`: `sum()` (count true), `min()`/`max()` (0.0/1.0 logic), `mean()` (proportion of true values)
  - [x] `StringColumn`: Statistical operations return `None` (appropriate fallback)
- [x] Standardized return type: All statistical operations return `Option<f64>`
- [x] Enhanced API ergonomics: Direct method calls on trait objects
- [x] Made all column types publicly accessible
- [x] **ARCHITECTURE DECISION**: No separate analyzer module needed
  - [x] Analysis is embedded directly in `ColumnArray` trait methods
  - [x] Each column type implements its own statistical operations
  - [x] DataFrame-level analysis is simple iteration over self-analyzing columns
- [x] Add statistics to result formatting (wide and long report formats implemented)
- [x] **Type Conversion System**: Added `From` trait implementations for seamless column creation
  - [x] `From<Vec<usize>>` for converting indices/counts to `IntegerColumn`
  - [x] Explicit `Vec<i64>` usage in tests for type clarity and maintainability
- [x] **Statistical Operation Refinements**: Enhanced boolean mean calculation and edge case handling
- [ ] Add advanced statistical operations (median, mode, variance)

## **COMPLETED PHASES** ✅

### **Phase 6A: CLI Integration** ✅ (COMPLETED)

- [x] Wire `check_na` command to NA analysis functionality (implemented as `na` command)
- [x] Wire `calculate_statistics` command to statistical reporting system (implemented as `info` command)  
- [x] Remove debugging code from `main.rs` and implement proper command routing
- [x] Test end-to-end functionality with sample CSV files
- [x] Add comprehensive help system with `--help`, `-h`, and `help` flags
- [x] Update command parsing to use concise names (`na`, `info`)
- [x] Implement proper error handling in main.rs

### **Phase 7: Library Refactoring** ✅ (COMPLETED)

- [x] Separate library and binary using `src/lib.rs` and `src/bin/csv_processor.rs`
- [x] Add comprehensive library documentation with usage examples
- [x] Configure Cargo.toml for library + binary publication
- [x] Update README.md to reflect dual library/CLI nature
- [x] Add API reference documentation with code examples
- [x] Test library and binary both work correctly
- [x] Update project documentation (app_design.md, todo.md)

## Phase 6: Error Handling & Performance ✅ (COMPLETED)

- [x] **Error Handling Standardization**: Convert all DataFrame operations to use proper Result types
  - [x] Created `DataFrameError` enum with specific error variants (HeadersColumnsLengthMismatch, ColumnsLengthMismatch, RowLengthMismatch, CsvError, IoError)
  - [x] Implemented Display and Error traits for DataFrameError
  - [x] Replaced all `panic!` calls with proper `Result` returns
  - [x] Added proper error conversion using `map_err` for CSV and IO operations
  - [x] Refactored error types into separate `frame/error.rs` module with clean public API
- [x] **Memory Optimization**: Remove duplicate `rows` storage from `DataFrame`
  - [x] Keep only parsed `columns: Vec<Box<dyn ColumnArray>>`
  - [x] DataFrame now stores only headers and typed columns (no raw rows)
  - [x] All data access goes through efficient column-oriented storage
- [x] **Test Infrastructure Robustness**: Fixed compilation issues and type handling
  - [x] Resolved `From` trait implementation gaps for integer types
  - [x] Enhanced test coverage with proper type annotations
  - [x] All 39 tests passing with robust statistical operation verification
- [ ] Performance optimization for large files
- [ ] Better CLI help and usage
- [ ] Documentation improvements
- [ ] Edge case handling

## Phase 7: Advanced Statistics 📋 (Medium Priority)

- [ ] Add advanced statistical operations (median, mode, variance)
- [ ] Implement percentile calculations
- [ ] Add correlation analysis between columns
- [ ] Statistical significance testing

## Phase 8: JSON Export Implementation ✅ (COMPLETED)

- [x] Add serde_json dependency to Cargo.toml
- [x] Implement `to_json()` method for all ColumnArray types (IntegerColumn, FloatColumn, StringColumn, BooleanColumn)
- [x] Create DataFrame JSON export methods:
  - [x] `to_json()` method for compact JSON string output (columns format)
  - [ ] `to_json_pretty()` method for formatted JSON output  
  - [ ] `save_json()` method for writing JSON to file
- [x] Support JSON output format:
  - [x] Columns format: `{"headers": [...], "columns": [[...], [...]]}`
  - [ ] Records format: `[{"col1": "val1", "col2": "val2"}, ...]`
  - [ ] Index format: `{"0": {"col1": "val1"}, "1": {"col2": "val2"}}`
- [x] Add JsonError variant to DataFrameError enum for proper error handling
- [x] Proper null handling in JSON serialization (None -> JSON null)
- [x] CLI integration: JSON export demonstration in info command
- [ ] Add comprehensive JSON serialization tests covering all data types
- [ ] Update library documentation with JSON export usage examples
- [ ] Add optional CLI command for JSON export: `csv_processor export --format json`
- [ ] Integration testing with sample CSV files

## Phase 9: Enhanced Features 📋 (Future Priority)

- [ ] Multiple output formats (CSV, Parquet)
- [ ] Column filtering options  
- [ ] Streaming for very large files
- [ ] Configuration file support
- [ ] Better CLI help and usage documentation
- [ ] Enhanced error messages with suggestions

## Learning Goals

### Rust Fundamentals

- [x] Rust project structure and modules
- [x] Error handling with custom types (`Result<T,E>`, custom error enums)
- [x] Testing patterns in Rust
- [x] Working with external crates (`csv`)
- [x] Trait system and dynamic dispatch (`ColumnArray` trait)
- [x] Unified trait design with `ColumnArray` combining data access and operations
- [x] Trait object ergonomics and polymorphic method calls
- [x] API design decisions: type safety vs usability trade-offs
- [x] Type inference and automatic parsing integrated into column system
- [x] Module reorganization following industry patterns (Polars/Arrow)
- [x] Separation of concerns: series, frame, and scalar modules
- [x] Enums with data (`CellValue`, `Dtype`)
- [x] Display trait implementation for formatted output
- [x] **Error handling patterns** - Result<T,E>, custom error enums, Display/Error traits
- [x] **Module organization** - private modules with public re-exports for clean APIs
- [ ] Memory-efficient data processing
- [ ] Performance optimization techniques

### For Python Developers - Key Concepts to Master

- [x] **Ownership system** - understand borrowing vs Python's garbage collection
- [x] **Pattern matching** - `match` expressions vs Python's if/elif
- [x] **Traits vs Inheritance** - composition over inheritance patterns
- [x] **Option<T> vs None** - explicit null handling vs Python's implicit None
- [x] **Result<T,E> vs Exceptions** - explicit error handling vs try/catch
- [ ] **Lifetimes** - when working with references (not needed yet in this project)
- [ ] **Iterators and closures** - functional programming patterns

## Phase 6.5: Module Architecture Reorganization ✅ (COMPLETED)

- [x] **MAJOR REFACTOR**: Reorganized module structure to follow Polars/Arrow patterns
- [x] Created `series/` module for column-oriented data structures
  - [x] Moved `columns.rs` → `series/array.rs`
  - [x] Added proper re-exports in `series/mod.rs`
- [x] Created `frame/` module for DataFrame operations
  - [x] Moved DataFrame struct to `frame/mod.rs`
  - [x] Moved `loader.rs` → `frame/io.rs`
- [x] Created `scalar/` module for cell-level operations  
  - [x] Moved `CellValue` from `types.rs` → `scalar/mod.rs`
  - [x] Added utility methods (`is_null()`, `data_type()`, `Display` trait)
- [x] Updated all imports and re-exports throughout codebase
- [x] Verified all tests still pass
- [x] Removed obsolete `dataframe/` directory
- [x] Updated documentation to reflect new structure

**Benefits Achieved:**

- ✅ Better separation of concerns  
- ✅ Follows industry-standard patterns (Polars/Arrow)
- ✅ More modular and extensible architecture
- ✅ Clearer naming that matches functionality

## Success Criteria

- Tool processes CSV files faster than equivalent Python scripts
- Clean, maintainable code following Rust idioms
- **✅ Achieved**: Ergonomic API design following industry patterns (Polars/Arrow)
- **✅ Achieved**: Unified trait interface enabling polymorphic operations
- **✅ Achieved**: Professional module organization following industry standards
- **✅ Achieved**: Formatted output with DataFrame Display trait
- **✅ Achieved**: Statistical reporting in multiple formats (wide/long)
- **✅ Achieved**: Comprehensive test coverage for core functionality (39 tests passing)
- **✅ Achieved**: Robust statistical operations including boolean mean calculations
- **✅ Achieved**: Type conversion system with explicit integer handling
- **✅ Achieved**: Production-ready code quality following Rust idioms
- **✅ Achieved**: Complete NA analysis functionality (integrated into unified reporting)
- **✅ Achieved**: Helpful error messages and user experience (comprehensive DataFrameError with user-friendly Display messages)
- **✅ Achieved**: Memory optimization (removed duplicate row storage)
- **✅ Achieved**: Full CLI integration with `na` and `info` commands
- **✅ Achieved**: Professional help system (`--help`, `-h`, `help` flags)
- **✅ Achieved**: Crates.io publication readiness (metadata, documentation, clean repository)
- **✅ Achieved**: Library + binary refactoring with clean API separation
- **✅ Achieved**: Comprehensive library documentation and usage examples
