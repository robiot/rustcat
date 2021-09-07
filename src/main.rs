//
// Rustcat (rc)
// Licence: MIT
//

use structopt::StructOpt;
use termion::color;

mod input;
mod listener;
mod options;
mod revshell;

fn print_error(err: String) {
    eprintln!(
        "{}rc:{} {}",
        color::Fg(color::LightRed),
        color::Fg(color::Reset),
        err
    );
}

fn main() {
    let opts = input::Opts::from_args();

    let (opt_host, opt_port) = if opts.port != None {
        ("0.0.0.0".to_string(), opts.port.unwrap())
    } else if opts.host.len() == 2 {
        (opts.host[0].to_string(), opts.host[1].to_string())
    } else {
        print_error("Missing port number".to_string());
        return;
    };

    // Reverse Shell
    if opts.rshell != None {
        if let Err(err) = revshell::unixshell(opt_host, opt_port, opts.rshell.unwrap()) {
            print_error(err.to_string());
        }
        return;
    }
    // Listen mode
    else if opts.listen_mode {
        let opts = options::Opts {
            host: opt_host,
            port: opt_port,
            transport: if opts.udp_mode {
                options::Protocol::Udp
            } else {
                options::Protocol::Tcp
            },
            mode: if opts.history {
                options::Mode::History
            } else {
                options::Mode::Normal
            },
        };

        if let Err(err) = listener::listen(&opts) {
            print_error(err.to_string());
            return;
        };
    }
}
