////////////////////////////
// Rustcat (rc)
// by: robiot & contributors
/////////////////////////////


use getopts::Options;
use termion::color;

mod options;
mod listener;
mod revshell;


/* Global Variables */
const __VERSION__: &'static str = env!("CARGO_PKG_VERSION");

/* Help -h */
fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [destination] [port]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(0);
}

/* Prints error */
fn print_error(err: String) {
    eprintln!(
        "{}rc:{} {}",
        color::Fg(color::LightRed),
        color::Fg(color::Reset),
        err
    );
}

/* Main */
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "This help text");
    opts.optflag("v", "version", "Application Version");
    opts.optflag("H", "history", "Command history for tcp");
    opts.optflag("l", "", "Listen mode");
    opts.optflag("p", "", "Port");
    opts.optflag("u", "", "UDP mode");
    opts.optflag("r", "", "Reverse Shell");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            print_error(err.to_string());
            return;
        }
    };

    // Help/about
    if matches.opt_present("h") {
        print_help(&program, opts);
        return;
    } else if matches.opt_present("v") {
        println!("Rustcat v{}", __VERSION__);
        return;
    }

    // Reverse Shell
    if matches.opt_present("r") {
        let opt_host: String;
        let opt_port: String;
        let opt_shell: String;

        if matches.free.len() == 2 && matches.opt_present("p") {
            opt_host = "0.0.0.0".to_string();
            opt_port = matches.free[0].to_string();
            opt_shell = matches.free[1].to_string();
        } else if matches.free.len() == 3 {
            opt_host = matches.free[0].to_string();
            opt_port = matches.free[1].to_string();
            opt_shell = matches.free[2].to_string();
        } else {
            print_error("Invalid Reverse Shell Mode Usage [ip] [port] [shell]".to_string());
            return;
        };
        
        let revshell_return = revshell::shell(opt_host, opt_port, opt_shell);
        if !revshell_return.is_empty(){
            print_error(revshell_return)
        }
        return;
    }

    // Listen mode
    if matches.opt_present("l") {
        let (opt_host, opt_port) = if matches.free.len() == 1 && matches.opt_present("p") {
            ("0.0.0.0", matches.free[0].as_str())
        } else if matches.free.len() == 2 {
            (matches.free[0].as_str(), matches.free[1].as_str())
        } else {
            print_error("Invalid Listen Mode Usage [ip] [port]".to_string());
            ("", "");
            return;
        };

        let opts = options::Opts {
            host: opt_host,
            port: opt_port,
            transport: if matches.opt_present("u") {
                options::Protocol::Udp
            } else {
                options::Protocol::Tcp
            },
            mode: if matches.opt_present("H") {
                options::Mode::Beta
            } else {
                options::Mode::Normal
            },
        };

        if let Err(err) = listener::listen(&opts) {
            print_error(err.to_string());
            return;
        };
    } else {
        print_help(&program, opts);
        return;
    };
}