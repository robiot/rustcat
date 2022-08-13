use clap::Parser;

mod input;
mod listener;
mod utils;

use utils::print_error;

#[cfg(unix)]
mod unixshell;

fn host_from_opts(host: Vec<String>) -> Result<(String, String), String> {
    let fixed_host = if host.len() == 1 {
        ("0.0.0.0".to_string(), host.get(0).unwrap().to_string()) // Safe to unwrap here
    } else if let [host, port] = &host[..] {
        (host.to_string(), port.to_string())
    } else {
        return Err("Missing host".to_string());
    };

    Ok(fixed_host)
}

fn main() {
    let opts = input::Opts::parse();

    match opts.command {
        input::Command::Listen {
            interactive,
            local_interactive,
            exec,
            host,
        } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    print_error(err);

                    return;
                }
            };
                let opts = listener::Opts {
                    host,
                    port,
                    exec,
                    mode: if interactive {
                        listener::Mode::Interactive
                    } else if local_interactive {
                        listener::Mode::LocalInteractive
                    } else {
                        listener::Mode::Normal
                    },
                };

                if let Err(err) = listener::listen(&opts) {
                    print_error(err);
                    return;
                };
        }
        input::Command::Connect { shell, host } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    print_error(err);

                    return;
                }
            };

            // Block usage on windows
            #[cfg(windows)]
            {
                print_error("This feature is not supported for windows");

                return;
            }

            #[cfg(unix)]
            if let Err(err) = unixshell::shell(host, port, shell) {
                print_error(err);

                return;
            }
        }
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
