/*
Name: utils.rs
Description: Functions and variables that is used from multiple files.
*/

use colored::Colorize;

pub struct Opts {
    pub host: String,
    pub port: String,
    pub exec: Option<String>,
    pub transport: Protocol,
    pub mode: Mode,
}

pub enum Protocol {
    Tcp,
    Udp,
}

pub enum Mode {
    Normal,
    History,
    LocalHistory,
}

pub fn print_error<T: std::string::ToString>(err: T) -> i32 {
    eprintln!("{} {}", "rc:".red(), err.to_string());
    0
}
