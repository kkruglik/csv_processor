# CSV Analytics Tool - Application Design

## Project Description

A command-line tool for CSV data analysis built in Rust. Features automatic type inference, embedded statistical operations, and a professional module architecture following industry patterns from Polars and Apache Arrow.

## Architecture

### Core Principles
- **Industry-aligned**: Module structure following Polars/Arrow patterns
- **Embedded Analysis**: Statistical operations integrated into column types (no separate analyzer)
- **Functional design**: Immutable data flow with pure functions
- **Single responsibility**: Each module handles one concern
- **Ergonomic API**: Direct method calls on trait objects without complex downcasting

### Data Flow
```
CLI Args → Config → DataFrame (self-analyzing columns) → Formatted Output
```

## Project Structure

```
src/
├── lib.rs           # ✅ Public API, re-exports
├── config.rs        # ✅ CLI parsing (Command enum, Config struct)
├── types.rs         # ✅ Core types (Dtype, CsvError)
├── series/          # ✅ Column-oriented data structures (Polars pattern)
│   ├── mod.rs       # ✅ Re-exports for series functionality
│   └── array.rs     # ✅ ColumnArray trait with embedded statistical operations
├── frame/           # ✅ DataFrame operations and I/O
│   ├── mod.rs       # ✅ DataFrame struct with headers, rows, typed columns
│   └── io.rs        # ✅ CSV file loading with load_dataframe()
├── scalar/          # ✅ Cell-level operations and values
│   └── mod.rs       # ✅ CellValue enum with utility methods
├── reporter.rs      # 🔜 Output formatting
└── main.rs          # ✅ CLI entry point
```

## Key Design Decisions

### Unified Column System (Following Polars Patterns)
- `ColumnArray` trait provides both data access AND statistical operations
- Self-analyzing columns with embedded statistical methods
- Automatic type inference (Integer → Float → Boolean → String)
- All statistical operations return `Option<f64>` for consistency
- No separate analyzer - analysis is embedded in column types

### Error Handling
- Custom `CsvError` enum for specific error types
- `Result<T, E>` pattern throughout for explicit error handling

### Core Components
- **Config**: CLI argument parsing with Command enum
- **DataFrame**: Main data container with metadata and self-analyzing typed columns
- **ColumnArray**: Unified trait for data access AND statistical operations
- **CellValue**: Enhanced enum with utility methods (is_null, data_type, Display)
- **Series Module**: Column-oriented structures following industry patterns
- **Frame Module**: DataFrame operations and CSV I/O
- **Scalar Module**: Cell-level operations and conversions

## Current Status
- ✅ **Foundation & Data Loading**: Complete with typed column system
- ✅ **Module Architecture**: Reorganized following Polars/Arrow patterns
- ✅ **Column System**: Complete with unified `ColumnArray` trait
- ✅ **Statistical Operations**: Complete for all column types (Integer, Float, Boolean, String)
  - All types implement: `sum()`, `min()`, `max()`, `mean()` returning `Option<f64>`
  - Proper null handling and NaN filtering
- ✅ **Analysis Architecture**: Complete - embedded in column trait system (no separate analyzer)
- ✅ **API Design**: Ergonomic trait object interface with direct method calls
- 🔜 **Output Formatting**: Ready for implementation
- 🔜 **Memory Optimization**: Remove duplicate row storage from DataFrame
