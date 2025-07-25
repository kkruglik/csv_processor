# CSV Analytics Tool - Application Design

## Project Description

A high-performance command-line tool for CSV data analysis built in Rust. Provides fast analytics operations like missing value detection and statistical calculations on CSV datasets.

## Architecture

### Core Principles
- **Functional design**: Data structures + pure functions over object-oriented patterns
- **Single responsibility**: Each module handles one concern
- **Immutable data flow**: Transform data rather than mutate state
- **Rust idioms**: Leverage ownership system and error handling

### Data Flow
```
User Input → Config → Dataset → AnalysisResult → Formatted Output
```

## Project Structure

```
src/
├── lib.rs           # Public API, re-exports
├── config.rs        # ✅ CLI parsing, user configuration
├── parser.rs        # 🔜 CSV loading, Dataset struct
├── analyzer.rs      # 🔜 Analysis functions
├── reporter.rs      # 🔜 Output formatting
└── main.rs          # CLI entry point
```

## Core Data Structures

### Configuration Layer
```rust
struct Config {
    command: Command,
    filename: String,
}

enum Command {
    CheckNAs,
    CalculateStatistics,
}
```

### Data Layer
```rust
struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}
```

### Analysis Layer
```rust
enum AnalysisResult {
    NAAnalysis(NAResult),
    Statistics(StatisticsResult),
}

struct NAResult {
    total_cells: usize,
    na_count: usize,
    na_by_column: HashMap<String, usize>,
}
```

## Core Functions

### Processing Pipeline
```rust
// Load CSV data
fn load_dataset(filename: &str) -> Result<Dataset, CsvError>

// Analysis operations
fn analyze_nas(dataset: &Dataset) -> NAResult
fn analyze_statistics(dataset: &Dataset) -> StatisticsResult

// Output formatting
fn format_result(result: &AnalysisResult) -> String
```

### Main Workflow
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = parse_config(&args)?;
    let dataset = load_dataset(config.filename())?;
    let result = analyze(&dataset, config.command());
    println!("{}", format_result(&result));
}
```

## Design Benefits

- **Testable**: Each component can be tested in isolation
- **Extensible**: Easy to add new commands and analysis types
- **Memory efficient**: Process data without excessive copying
- **Performance**: Rust's zero-cost abstractions and ownership system
- **Maintainable**: Clear separation of concerns
