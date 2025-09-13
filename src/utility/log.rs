use colored::{ColoredString, Colorize};
use console::strip_ansi_codes;

pub struct Log;

impl Log {
    pub fn hello() {
        println!();
        println!("{} - simple image processing CLI", "Pixel".italic().cyan());
        println!();
    }

    pub fn info(message: &str) {
        println!("{} - {}", "Info".cyan().bold(), message);
    }

    pub fn warn(message: &str) {
        println!("{} - {}", "Warning".yellow().bold(), message);
    }

    pub fn error(message: &str) {
        panic!("{} - {}", "Error".red().bold(), message);
    }

    pub fn command(command: &str, message: &str) {
        let prefix = "pixel".bold();

        let mut colored_command: Vec<ColoredString> = Vec::new();

        let mut parsed_command = command
            .split(" ")
            .map(|s| s.into())
            .collect::<Vec<ColoredString>>();

        for (_, part) in parsed_command.iter_mut().enumerate() {
            if part.starts_with("--") {
                colored_command.push(part.clone().bright_black());
            } else if part.starts_with("-") {
                colored_command.push(part.clone().bright_black());
            } else if part.starts_with("<") && part.ends_with(">") {
                colored_command.push(part.clone().cyan());
            } else if part.starts_with("[") && part.ends_with("]") {
                colored_command.push(part.clone().red());
            } else {
                colored_command.push(part.clone());
            }
        }

        let colored_command = colored_command
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let plain = command.to_string();
        let width = strip_ansi_codes(&plain).len();

        println!(
            "{}{:<7} {}{} {}",
            " ".repeat(5),
            prefix,
            colored_command,
            " ".repeat(35 - width),
            message
        );
    }

    pub fn flag(command: &str, message: &str) {
        let mut colored_command: Vec<ColoredString> = Vec::new();

        let mut parsed_command = command
            .split(" ")
            .map(|s| s.into())
            .collect::<Vec<ColoredString>>();

        for (_, part) in parsed_command.iter_mut().enumerate() {
            if part.starts_with("--") {
                colored_command.push(part.clone().red());
            } else if part.starts_with("-") {
                colored_command.push(part.clone().bright_black());
            } else if part.starts_with("<") && part.ends_with(">") {
                colored_command.push(part.clone().cyan());
            } else if part.starts_with("[") && part.ends_with("]") {
                colored_command.push(part.clone().red());
            } else {
                colored_command.push(part.clone());
            }
        }

        let colored_command = colored_command
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let plain = command.to_string(); // z kolorami
        let width = strip_ansi_codes(&plain).len();

        println!(
            "{}{}{} {}",
            " ".repeat(13),
            colored_command,
            " ".repeat(35 - width),
            message
        );
    }
}
