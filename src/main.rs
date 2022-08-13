use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};

mod input;
mod listener;

// use colored::Colorize;

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
    // Configure logger
    if let Err(err) = fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                .warn(Color::Yellow)
                .info(Color::BrightGreen)
                .error(Color::Red);

            out.finish(format_args!(
                "{}{} {}",
                colors.color(record.level()).to_string().to_lowercase(), // Hardcoded red .red().bold()
                ":",
                message
            ))
        })
        // .level(log::LevelFilter::Error) // For if verbose mode is implemented
        .chain(std::io::stdout())
        .apply()
    {
        println!("Failed to initialize logger: {}", { err });

        return;
    }

    let opts = input::Opts::parse();

    match opts.command {
        input::Command::Listen {
            interactive,
            block_signals,
            local_interactive,
            exec,
            host,
        } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    log::error!("{}", err);

                    return;
                }
            };

            let opts = listener::Opts {
                host,
                port,
                exec,
                block_signals,
                mode: if interactive {
                    listener::Mode::Interactive
                } else if local_interactive {
                    listener::Mode::LocalInteractive
                } else {
                    listener::Mode::Normal
                },
            };

            if let Err(err) = listener::listen(&opts) {
                log::error!("{}", err);

                return;
            };
        }
        input::Command::Connect { shell, host } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    log::error!("{}", err);

                    return;
                }
            };

            #[cfg(unix)]
            if let Err(err) = unixshell::shell(host, port, shell) {
                log::error!("{}", err);

                return;
            }

            // Block usage on other operating systems
            #[cfg(not(unix))]
            {
                log::error!("This feature is not supported on your platform");

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
