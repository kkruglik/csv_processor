#[derive(Debug, PartialEq)]
pub enum Command {
    CheckNAs,
    Info,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnknownCommand(String),
    MissingArguments(String),
    FileNotFound(String),
}

#[derive(Debug)]
pub struct Config {
    command: Command,
    filename: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::UnknownCommand(cmd) => {
                write!(
                    f,
                    "Unknown command '{}'. Available: na, info",
                    cmd
                )
            }
            ConfigError::MissingArguments(msg) => write!(f, "{}", msg),
            ConfigError::FileNotFound(file) => write!(f, "File '{}' not found", file),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn new(command: Command, filename: String) -> Config {
        Config { command, filename }
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

pub fn parse_command(command: String) -> Result<Command, ConfigError> {
    match command.to_lowercase().as_str() {
        "na" => Ok(Command::CheckNAs),
        "info" => Ok(Command::Info),
        _ => Err(ConfigError::UnknownCommand(command)),
    }
}

pub fn print_help() {
    println!("CSV Analytics Tool");
    println!();
    println!("USAGE:");
    println!("    csv_processor <COMMAND> <FILE>");
    println!();
    println!("COMMANDS:");
    println!("    na      Check for missing values (NAs) in CSV file");
    println!("    info    Calculate statistics for CSV file");
    println!();
    println!("EXAMPLES:");
    println!("    csv_processor na sample.csv");
    println!("    csv_processor info sample.csv");
}

pub fn parse_config(args: &[String]) -> Result<Config, ConfigError> {
    // Check for help flags
    if args.len() == 1 || (args.len() == 2 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help")) {
        print_help();
        std::process::exit(0);
    }

    if args.len() < 3 {
        return Err(ConfigError::MissingArguments(
            "Not enough arguments passed!".to_string(),
        ));
    }

    let command = parse_command(args[1].clone())?;
    Ok(Config::new(command, args[2].clone()))
}
