use chrono::*;
use colored::*;
use std::fmt::Display;

/// Prints text, no mystery
fn print<T: Display>(text: T, prefix: String) {
    let txt = format!(
        "[ {} ] [ {} ] {}",
        Local::now().format("%+").to_string().bright_black(),
        prefix,
        text
    );

    if prefix == "ERROR" {
        eprintln!("{}", txt)
    } else {
        println!("{}", txt)
    }
}

/// Print as log
pub fn info<T: Display>(text: T) {
    print(text, "INFO ".bold().cyan().to_string())
}

/// Print as ok

pub fn ok<T: Display>(text: T) {
    print(text, "OK   ".bold().green().to_string())
}

/// Print as warn
pub fn warn<T: Display>(text: T) {
    print(text, "WARN ".bold().yellow().to_string())
}

/// Print as error (stderr)
pub fn error<T: Display>(text: T) {
    print(text, "ERROR".bold().red().to_string())
}
