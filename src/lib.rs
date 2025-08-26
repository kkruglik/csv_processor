//! # CSV Processor
//!
//! A fast CSV analysis library with automatic type inference and comprehensive statistics.
//!
//! ## Features:
//!
//! - **Automatic Type Inference**: Intelligently detects integers, floats, booleans, and strings
//! - **Statistical Operations**: Built-in sum, mean, min, max calculations for all numeric types
//! - **Self-Analyzing Columns**: Each column type implements its own statistical operations
//! - **Professional Reporting**: Formatted statistical reports in multiple formats
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use csv_processor::{DataFrame, reporter::generate_info_report};
//!
//! // Load CSV file
//! let df = DataFrame::from_csv("data.csv")?;
//!
//! // Generate statistical report
//! let report = generate_info_report(&df);
//! println!("{}", report);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod config;
pub mod frame;
pub mod reporter;
pub mod scalar;
pub mod series;
pub mod types;

// Core data structures
pub use frame::DataFrame;
pub use scalar::CellValue;
pub use series::ColumnArray;
pub use types::{CsvError, Dtype};

// CLI-specific exports (optional for library users)
pub use config::{parse_command, parse_config, Command, Config, ConfigError};
