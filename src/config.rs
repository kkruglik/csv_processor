#[derive(Debug, PartialEq)]
pub enum Command {
    CheckNAs,
    CalculateStatistics,
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
                    "Unknown command '{}'. Available: check_na, calculate_statistics",
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
        "check_na" => Ok(Command::CheckNAs),
        "calculate_statistics" => Ok(Command::CalculateStatistics),
        _ => Err(ConfigError::UnknownCommand(command)),
    }
}

pub fn parse_config(args: &[String]) -> Result<Config, ConfigError> {
    if args.len() < 3 {
        return Err(ConfigError::MissingArguments(
            "Not enough arguments passed!".to_string(),
        ));
    }

    let command = parse_command(args[1].clone())?;
    Ok(Config::new(command, args[2].clone()))
}
