# CSV Analytics Tool - Development Roadmap

## Phase 1: Foundation ✅
- [x] Project setup with Cargo
- [x] CLI argument parsing (`config.rs`)
- [x] Custom error types
- [x] Basic test framework
- [x] Module structure planning

## Phase 2: Data Loading ✅ (Mostly Complete)
- [x] Create `DataFrame` struct (`dataframe/mod.rs`) 
- [x] Implement CSV file reading with `csv` crate (`dataframe/loader.rs`)
- [x] Add CSV parsing error handling (`types.rs` - `CsvError`)
- [x] Implement typed column system (`dataframe/columns.rs`)
  - [x] `ColumnArray` trait for polymorphic columns
  - [x] Typed columns: `IntegerColumn`, `FloatColumn`, `StringColumn`, `BooleanColumn`
  - [x] Automatic type inference and parsing
- [x] Basic data validation and null handling
- [x] Unit tests for column operations (columns_tests.rs created)
- [ ] Unit tests for CSV loading (needs implementation in loader module)

## Phase 3: NA Analysis 🔄 (Trait-Based Approach)
- [x] `ColumnArray` trait provides `null_count()` method for missing value analysis
- [ ] Implement `analyze_nans()` function using trait methods:
  - [ ] Use `ColumnArray.null_count()` for missing value counts
  - [ ] Calculate NA percentages per column using `len()` and `null_count()`
  - [ ] Return structured analysis results
- [ ] Integration tests with sample data
- [ ] Wire up analysis to main CLI flow

## Phase 4: Output Formatting 🔜
- [ ] Create basic reporter (`reporter.rs`)
- [ ] Format NA analysis results
- [ ] Table output formatting
- [ ] CLI integration and testing

## Phase 5: Statistics Analysis ✅ (Unified ColumnArray Approach)
- [x] **ARCHITECTURAL SHIFT**: Consolidated statistical operations into `ColumnArray` trait
- [x] Removed separate `ChunkAgg<T>` trait in favor of unified interface
- [x] Implement comprehensive statistical operations:
  - [x] `IntegerColumn`: `sum()`, `min()`, `max()`, `mean()` with proper null handling
  - [x] `FloatColumn`: `sum()`, `min()`, `max()`, `mean()` with NaN filtering
  - [x] `BooleanColumn`: `sum()` (count true), `min()`/`max()` (0.0/1.0 logic)
  - [x] `StringColumn`: Statistical operations return `None` (appropriate fallback)
- [x] Standardized return type: All statistical operations return `Option<f64>`
- [x] Enhanced API ergonomics: Direct method calls on trait objects
- [x] Made all column types publicly accessible
- [x] **ARCHITECTURE DECISION**: No separate analyzer module needed
  - [x] Analysis is embedded directly in `ColumnArray` trait methods
  - [x] Each column type implements its own statistical operations
  - [x] DataFrame-level analysis is simple iteration over self-analyzing columns
- [ ] Add advanced statistical operations (median, mode, variance)
- [ ] Add statistics to result formatting

## Phase 6: Memory Optimization & Performance 📋
- [ ] **Memory Optimization**: Remove duplicate `rows` storage from `DataFrame`
  - [ ] Keep only parsed `columns: Vec<Box<dyn ColumnArray>>`
  - [ ] Implement `get_row()` reconstruction from columns when needed
  - [ ] Update `DataFrame::new()` to not store raw rows
- [ ] Error message improvements
- [ ] Performance optimization for large files
- [ ] Better CLI help and usage
- [ ] Documentation improvements
- [ ] Edge case handling

## Phase 7: Advanced Features (Future) 📋
- [ ] Multiple output formats (JSON, CSV)
- [ ] Data type inference
- [ ] Column filtering options
- [ ] Streaming for very large files
- [ ] Configuration file support

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
- Comprehensive test coverage
- Helpful error messages and user experience
