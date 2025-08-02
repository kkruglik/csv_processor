# CSV Analytics Tool - Application Design

## Project Description

A command-line tool for CSV data analysis built in Rust. Supports missing value detection and statistical calculations on CSV datasets.

## Architecture

### Core Principles
- Functional design with immutable data flow
- Single responsibility per module
- Typed column system for performance

### Data Flow
```
CLI Args → Config → DataFrame → Analysis → Output
```

## Project Structure

```
src/
├── lib.rs           # ✅ Public API, re-exports
├── config.rs        # ✅ CLI parsing (Command enum, Config struct)
├── types.rs         # ✅ Core types (CellValue, Dtype, CsvError)
├── dataframe/       # ✅ Data structures and CSV loading
│   ├── mod.rs       # ✅ DataFrame with typed columns
│   ├── loader.rs    # ✅ CSV file loading
│   └── columns.rs   # ✅ ColumnArray trait, typed column implementations
├── analyzer.rs      # 🔜 Analysis functions (structure ready)
├── reporter.rs      # 🔜 Output formatting
└── main.rs          # ✅ CLI entry point
```

## Key Design Decisions

### Typed Column System
- `DataFrame` stores both raw string data and typed columns
- `ColumnArray` trait provides polymorphic access to different column types
- Automatic type inference (Integer → Float → Boolean → String)

### Error Handling
- Custom `CsvError` enum for specific error types
- `Result<T, E>` pattern throughout for explicit error handling

### Core Components
- **Config**: CLI argument parsing with Command enum
- **DataFrame**: Main data container with metadata and typed columns  
- **ColumnArray**: Trait for type-specific column operations
- **CellValue**: Enum for all possible cell values including nulls

## Current Status
- ✅ Data loading and parsing complete
- ✅ Typed column system implemented
- 🔜 Analysis functions (structure ready, implementation needed)
- 🔜 Output formatting
