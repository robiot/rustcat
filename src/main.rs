use clap::Parser;
use fern::Dispatch;
use fern::colors::{Color, ColoredLevelConfig};
use std::io::stdout;
use crate::listener::{Mode, Opts, listen};
use crate::input::Command;

mod input;
mod listener;

#[cfg(unix)]
mod unixshell;

#[cfg(windows)]
mod winshell;

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
    if let Err(err) = Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                .warn(Color::Yellow)
                .info(Color::BrightGreen)
                .error(Color::Red);

            out.finish(format_args!(
                "{}{} {}",
                colors.color(record.level()).to_string().to_lowercase(),
                ":",
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level(log::LevelFilter::Info)
        .chain(stdout())
        .apply()
    {
        println!("Failed to initialize logger: {}", { err });

        return;
    }

    let opts = input::Opts::parse();

    match opts.command {
        Command::Listen {
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

            let opts = Opts {
                host,
                port,
                exec,
                block_signals,
                mode: if interactive {
                    Mode::Interactive
                } else if local_interactive {
                    Mode::LocalInteractive
                } else {
                    Mode::Normal
                },
            };

            if let Err(err) = listen(&opts) {
                log::error!("{}", err);
            };
        }
        Command::Connect { shell, host } => {
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
            }

            #[cfg(windows)]
            if let Err(err) = winshell::shell(host, port, shell) {
                log::error!("{}", err);
            }

            #[cfg(not(any(unix, windows)))]
            {
                log::error!("This feature is not supported on your platform");
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(unix)]
    use super::unixshell;

    use std::io::ErrorKind;

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
            Err(ErrorKind::InvalidInput)
        )
    }
}
