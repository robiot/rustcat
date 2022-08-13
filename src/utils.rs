/*
Name: utils.rs
Description: Functions and variables that is used from multiple files.
*/

use colored::Colorize;

pub fn print_error<T: std::string::ToString>(err: T) {
    eprintln!("{} {}", "rc:".red(), err.to_string());
}
