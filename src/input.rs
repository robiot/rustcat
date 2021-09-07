/*
Name: input.rs
Description: Setup Structopt Struct.
*/

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcat", setting = structopt::clap::AppSettings::ArgRequiredElseHelp)]
pub struct Opts {
    /// Command history for tcp
    #[structopt(short = "H", long)]
    pub history: bool,

    /// Listen mode
    #[structopt(short, long = "listen")]
    pub listen_mode: bool,

    /// UDP mode
    #[structopt(short, long = "udp")]
    pub udp_mode: bool,

    /// Local port
    #[structopt(short, long)]
    pub port: Option<String>,

    /// Reverse shell
    #[structopt(short, long)]
    pub rshell: Option<String>,

    // Host:ip
    #[structopt(hidden = true, max_values = 2)]
    pub host: Vec<String>,
}
