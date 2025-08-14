use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rustcat", version, arg_required_else_help(true))]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
    // #[clap(short, long)]
    // verbose: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    Tcp,
    Tls,
    Udp,
    Dtls,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start a listener for incoming connections
    #[clap(alias = "l")]
    Listen {
        /// Interactive
        #[clap(short, long, name = "interactive")]
        interactive: bool,

        /// Block exit signals like CTRL-C
        #[clap(short, long, conflicts_with = "local-interactive")]
        block_signals: bool,

        /// Local interactive
        #[clap(
            short,
            long,
            name = "local-interactive",
            conflicts_with = "interactive"
        )]
        local_interactive: bool,

        /// Execute command when connection received
        #[clap(short, long)] // hidden
        exec: Option<String>,

        // Host:ip, IP if only 1 value provided
        #[clap(num_args = ..=2)]
        host: Vec<String>,

        /// Protocol: tcp, tls, udp, dtls
        #[clap(long, default_value = "tcp")]
        protocol: String,

        /// Path to certificate file (PEM)
        #[clap(long)]
        cert: Option<String>,

        /// Path to private key file (PEM)
        #[clap(long)]
        key: Option<String>,
    },

    /// Connect to the controlling host
    #[clap(alias = "c")]
    Connect {
        /// The shell to use
        #[clap(short, long)]
        shell: String,

        // Host:ip, IP if only 1 value provided
        #[clap(num_args = ..=2)]
        host: Vec<String>,

        /// Protocol: tcp, tls, udp, dtls
        #[clap(long, default_value = "tcp")]
        protocol: String,

        /// Path to certificate file (PEM)
        #[clap(long)]
        cert: Option<String>,

        /// Path to private key file (PEM)
        #[clap(long)]
        key: Option<String>,
    },
}
