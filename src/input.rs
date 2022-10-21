use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "rustcat",
    version,
    setting(clap::AppSettings::ArgRequiredElseHelp),
)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,

    // #[clap(short, long)]
    // verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start a listener for incoming connections
    #[clap(alias = "l")]
    Listen {
        /// Interactive
        #[clap(short, long)]
        interactive: bool,

        /// Block exit signals like CTRL-C
        #[clap(short, long, conflicts_with = "local-interactive")]
        block_signals: bool,

        /// Local interactive
        #[clap(short, long, conflicts_with = "interactive")]
        local_interactive: bool,

        /// Execute command when connection received
        #[clap(short, long)] // hidden
        exec: Option<String>,

        // Host:ip, IP if only 1 value provided
        #[clap(max_values = 2)]
        host: Vec<String>,
    },

    /// Connect to the controlling host
    #[clap(alias = "c")]
    Connect {
        /// The shell to use
        #[clap(short, long)]
        shell: String,

        // Host:ip, IP if only 1 value provided
        #[clap(max_values = 2)]
        host: Vec<String>,
    },
}
