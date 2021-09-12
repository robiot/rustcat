/*
Name: utils.rs
Description: Functions and variables that is used from multiple files.
*/

use colored::Colorize;

/* Public Variables */
pub struct Opts {
    pub host: String,
    pub port: String,
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
}

/* Public Functions */
pub fn print_error<T: std::string::ToString>(err: T) {
    eprintln!("{} {}", "rc:".red(), err.to_string());
}
