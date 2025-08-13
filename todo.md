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
- [ ] Unit tests for CSV loading (parser_tests.rs commented out - needs updating)

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

## Phase 5: Statistics Analysis 🔄 (Trait-Based Approach)
- [x] Create `ChunkAgg<T>` trait for column-level aggregations (`dataframe/aggregation.rs`)
- [x] Implement basic statistical operations for `IntegerColumn`:
  - [x] `sum()`, `min()`, `max()`, `mean()` methods
- [ ] Extend aggregation implementations:
  - [ ] Complete `FloatColumn` aggregations (sum, min, max, mean, std, variance)
  - [ ] Add `StringColumn` aggregations (count, mode, unique values)
  - [ ] Add `BooleanColumn` aggregations (count true/false, percentage)
- [ ] Add advanced statistical operations:
  - [ ] `median()`, `mode()`, `variance()` methods to `ChunkAgg` trait
  - [ ] Implement for all relevant column types
- [ ] Update `analyzer.rs` to orchestrate column aggregations:
  - [ ] `analyze_statistics()` function using trait methods
  - [ ] Remove old individual calculation functions
- [ ] Add statistics to result formatting

## Phase 6: Polish & Performance 📋
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
- [x] Generic traits with type parameters (`ChunkAgg<T>`)
- [x] Multiple trait implementations per type (e.g., `ChunkAgg<i64>` and `ChunkAgg<f64>` for `IntegerColumn`)
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

## Success Criteria
- Tool processes CSV files faster than equivalent Python scripts
- Clean, maintainable code following Rust idioms
- Comprehensive test coverage
- Helpful error messages and user experience
