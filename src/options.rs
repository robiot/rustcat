/*
options.rs
*/


pub struct Opts<'a> {
    pub host: &'a str,
    pub port: &'a str,
    pub transport: Protocol,
    pub mode: Mode,
}

pub enum Protocol {
    Tcp,
    Udp,
}

pub enum Mode {
    Normal,
    Beta,
}