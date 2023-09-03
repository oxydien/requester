use colored::Colorize;

pub enum LogLevel {
    Info,
    Warn,
    Debug,
    Error,
}

#[allow(non_snake_case)]
pub fn LOG(level: LogLevel, fn_name: &str, message: String) {
    let log_level_str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Debug => "DEBUG",
        LogLevel::Error => "ERROR",
    };
    let colored_fn = match level {
        LogLevel::Info => fn_name.blue(),
        LogLevel::Warn => fn_name.yellow(),
        LogLevel::Debug => fn_name.green(),
        LogLevel::Error => fn_name.red(),
    };

    println!("[{}] {}: {}", colored_fn, log_level_str, message);
}
