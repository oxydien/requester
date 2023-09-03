use crate::{
    filesys::{check_app_path, config::check_app_version, history::create_histories},
    utils::log::LOG,
};
use colored::Colorize;

pub mod log;

pub fn on_start() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("                             Welcome to:");
    println!(
        "██████  ███████{}██    ██ ███████ ███████ ████████ ███████ ██████  ",
        "  ██████  ".truecolor(255, 138, 0)
    );
    println!(
        "██   ██ ██     {}██    ██ ██      ██         ██    ██      ██   ██ ",
        " ██    ██ ".truecolor(255, 138, 0)
    );
    println!(
        "██████  █████  {}██    ██ █████   ███████    ██    █████   ██████  ",
        " ██     █ ".truecolor(255, 138, 0)
    );
    println!(
        "██   ██ ██     {}██    ██ ██           ██    ██    ██      ██   ██ ",
        " ██   ██  ".truecolor(255, 138, 0)
    );
    println!(
        "██   ██ ███████{} ██████  ███████ ███████    ██    ███████ ██   ██ ",
        "  ████ ██ ".truecolor(255, 138, 0)
    );
    println!("");
    println!(
        "[{}] This application provides a graphical user interface (GUI) for creating",
        "!".red()
    );
    println!(
        " requests. Please note that it {} command-line interface ({}). ",
        "doesn't offer".bold(),
        "CLI".bold()
    );
    println!(" If you're looking for terminal-based request creation, this might not be ");
    println!(" the right choice for you.");
    println!("");
    println!("  ╔═══════════════════════════════════════════════════╗");
    println!(
        "  ║ Started by: {}                               ║",
        "oxydien".truecolor(255, 138, 0)
    );
    println!(
        "  ║ Github: <{}>    ║",
        "https://github.com/oxydien/requester".blue().underline()
    );
    println!("  ╚═══════════════════════════════════════════════════╝");
    println!("");

    println!("Checking app path...");
    check_app_path();
    println!("Checking app version...");
    check_app_version();
    println!("Checking histories...");
    create_histories();
    println!("Everything should be good to go!");
    LOG(
        log::LogLevel::Info,
        "on_start",
        "Starting tauri.".to_string(),
    );
}

pub fn open_file(path: &str) {
    #[cfg(target_os = "windows")]
    let command = "notepad.exe";

    #[cfg(target_os = "macos")]
    let command = "open";

    #[cfg(target_os = "linux")]
    let command = "xdg-open";

    std::process::Command::new(command)
        .arg(path)
        .spawn()
        .expect("Failed to open the file");
}
