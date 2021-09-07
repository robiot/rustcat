/*
Name: options.rs
Description: Options for listener.
*/

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