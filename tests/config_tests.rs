use csv_processor::config::*;

#[test]
fn test_parse_command_success() {
    assert_eq!(
        parse_command("check_na".to_string()).unwrap(),
        Command::CheckNAs
    );
    assert_eq!(
        parse_command("calculate_statistics".to_string()).unwrap(),
        Command::CalculateStatistics
    );
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
