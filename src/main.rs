//! Rustcat (rc)
//! Licence: MIT

use structopt::StructOpt;

mod input;
mod listener;
mod utils;

use utils::print_error;

#[cfg(unix)]
mod unixshell;

fn main() {
    let opts = input::Opts::from_args();

    let (opt_host, opt_port) = if let Some(port) = opts.port {
        ("0.0.0.0".to_string(), port)
    } else if let [host, port] = &opts.host[..] {
        (host.to_string(), port.to_string())
    } else {
        print_error("Missing port number");
        return;
    };

    // Reverse Shell
    if let Some(rshell) = opts.rshell {
        // Block usage on windows
        #[cfg(windows)]
        {
            print_error("Reverse shells are currently not supported for windows");
            return;
        }
        #[cfg(unix)]
        if let Err(err) = unixshell::shell(opt_host, opt_port, rshell) {
            print_error(err);
            return;
        }
    }
    // Listen mode
    else if opts.listen_mode {
        let opts = utils::Opts {
            host: opt_host,
            port: opt_port,
            exec: opts.exec,
            transport: if opts.udp_mode {
                utils::Protocol::Udp
            } else {
                utils::Protocol::Tcp
            },
            mode: if opts.history {
                utils::Mode::History
            } else if opts.local_history {
                utils::Mode::LocalHistory
            } else {
                utils::Mode::Normal
            },
        };

        if let Err(err) = listener::listen(&opts) {
            print_error(err);
            return;
        };
    }
}

#[cfg(test)]
mod tests {

    #[cfg(unix)]
    use super::unixshell;

    // Panics if InvalidInput Not returned
    #[test]
    #[cfg(unix)]
    fn revshell_bad_port() {
        assert_eq!(
            unixshell::shell(
                "0.0.0.0".to_string(),
                "420692223".to_string(),
                "bash".to_string()
            )
            .map_err(|e| e.kind()),
            Err(std::io::ErrorKind::InvalidInput)
        )
    }
}
