/*
Name: input.rs
Description: Setup Structopt Struct.
*/

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcat", setting = structopt::clap::AppSettings::ArgRequiredElseHelp)]
pub struct Opts {
    /// Command history for tcp
    #[structopt(short = "H", long, conflicts_with = "rshell")]
    pub history: bool,

    /// Local history
    #[structopt(short = "L", long, conflicts_with_all = &["history", "exec"])]
    pub local_history: bool,

    /// Listen mode
    #[structopt(short, long = "listen", conflicts_with = "rshell")]
    pub listen_mode: bool,

    /// UDP mode
    #[structopt(short, long = "udp")]
    pub udp_mode: bool,

    /// Execute command when connection recieved
    #[structopt(short, long, value_name = "command", conflicts_with_all = &["local_history", "rshell"])]
    pub exec: Option<String>,

    /// Local port
    #[structopt(short, long)]
    pub port: Option<String>,

    /// Reverse shell
    #[structopt(short, long, value_name="shell")]
    pub rshell: Option<String>,

    // Host:ip
    #[structopt(hidden = true, max_values = 2)]
    pub host: Vec<String>,
}
