use csv_processor::config::*;

#[test]
fn test_parse_command_success() {
    assert_eq!(parse_command("na".to_string()).unwrap(), Command::CheckNAs);
    assert_eq!(parse_command("info".to_string()).unwrap(), Command::Info);
}

#[test]
fn test_parse_command_errors() {
    assert_eq!(
        parse_command("invalid".to_string()),
        Err(ConfigError::UnknownCommand("invalid".to_string()))
    );

    assert_eq!(
        parse_command("calculate_something".to_string()),
        Err(ConfigError::UnknownCommand(
            "calculate_something".to_string()
        ))
    );
}
