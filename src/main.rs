////////////////////////////
// Rustcat (rc)
// by: robiot & contributors
/////////////////////////////

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};
use getopts::Options;
use termion::color;

/* Global Variables */
const __VERSION__: &'static str = env!("CARGO_PKG_VERSION");

/* Use laters */
struct Opts<'a> {
    host: &'a str,
    port: &'a str,
    transport: Protocol,
    mode: Mode,
}

enum Protocol {
    Tcp,
    Udp,
}

enum Mode {
    Normal,
    Beta,
}

/* Help -h */
fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [destination] [port]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(0);
}

/* Prints error */
fn print_error(err: &str) {
    eprintln!(
        "{}rc:{} {}",
        color::Fg(color::LightRed),
        color::Fg(color::Reset),
        err
    );
}

/* Print when connection recieved */
fn print_started_listen(opts: &Opts) {
    println!(
        "Listening on {}{}{}:{}{}{}",
        color::Fg(color::LightGreen),
        opts.host,
        color::Fg(color::Reset),
        color::Fg(color::LightCyan),
        opts.port,
        color::Fg(color::Reset)
    );
}

/* Piped thread */
fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where
    R: std::io::Read + Send + 'static,
    W: std::io::Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = r.read(&mut buffer).unwrap();
            if len == 0 {
                println!(
                    "\n{}[-]{}Connection lost",
                    color::Fg(color::LightRed),
                    color::Fg(color::Reset)
                );
                std::process::exit(0);
            }
            
            w.write_all(&buffer[..len]).unwrap();
            w.flush().unwrap();
        }
    })
}

/* Listen on given host and port */
fn listen(opts: &Opts) -> std::io::Result<()> {
    match opts.transport {
        Protocol::Tcp => {
            let listener = std::net::TcpListener::bind(format!("{}:{}", opts.host, opts.port))?;
            print_started_listen(opts);

            let (mut stream, _) = listener.accept()?;
            match opts.mode {
                Mode::Normal => {
                    let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
                    let t2 = pipe_thread(stream, std::io::stdout());
                    t1.join().unwrap();
                    t2.join().unwrap();
                }
                Mode::Beta => {
                    // For command line history there is a better way of doing it
                    // You can constantly send the input to the revshell and let it do the stuff
                    // instead of doing the history locally and getting the line deleted
                    let t = pipe_thread(stream.try_clone().unwrap(), std::io::stdout()); 
                    let mut rl = Editor::<()>::new();
                
                    loop {
                        let readline = rl.readline(">> "); // &buffer[..len] ?
                        match readline {
                            Ok(command) => {
                                rl.add_history_entry(command.as_str());
                                let command = command.clone() + "\n";
                                stream
                                    .write_all(command.as_bytes())
                                    .expect("Faild to send TCP.");
                            }
                            Err(ReadlineError::Interrupted) => {
                                std::process::exit(0);
                            }
                            Err(ReadlineError::Eof) => {
                                std::process::exit(0);
                            }
                            Err(err) => {
                                println!("Error: {:?}", err);
                                break;
                            }
                        }
                    }
                
                    t.join().unwrap();
                }
            }
        }

        Protocol::Udp => {
            // Can be made better probably...
            // Rustline is needed here because else you cant delete characters
            let socket = std::net::UdpSocket::bind(format!("{}:{}", opts.host, opts.port))?;
            print_started_listen(opts);
        
            use std::sync::{Arc, Mutex};
            let addr: Arc<Mutex<Option<std::net::SocketAddr>>> = Arc::from(Mutex::new(None));
        
            let addr_clone = addr.clone();
            let socket_clone = socket.try_clone().unwrap();
            std::thread::spawn(move || loop {
                let mut buffer = [0; 1024];
                let (len, src_addr) = socket_clone.recv_from(&mut buffer).unwrap();
                *addr_clone.lock().unwrap() = Some(src_addr);

                io::stdout().write_all(&buffer[..len]).unwrap();
                io::stdout().flush().unwrap();
            });
        
            loop {
                let mut rl = Editor::<()>::new();
                loop {
                    let readline = rl.readline(">> ");
                    match readline {
                        Ok(command) => {
                            rl.add_history_entry(command.as_str());
                            let command = command.clone() + "\n";
                            
                            let addr_option = *addr.lock().unwrap();
                            if let Some(addr) = addr_option {
                                socket.send_to(command.as_bytes(), addr).unwrap();
                            } else {
                                panic!("Cannot send udp packet");
                            }
                        }
                        Err(ReadlineError::Interrupted) => {
                            std::process::exit(0);
                        }
                        Err(ReadlineError::Eof) => {
                            std::process::exit(0);
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                            std::process::exit(0);
                        }
                    }
                }
            }
        }
    }
    return Ok(());
}

/* Main */
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "This help text");
    opts.optflag("v", "version", "Application Version");
    opts.optflag("H", "history", "Command history for tcp (Beta)");
    opts.optflag("l", "", "Listen mode");
    opts.optflag("p", "", "Listen port");
    opts.optflag("u", "", "UDP mode (Beta)");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            print_error(&err.to_string());
            return;
        }
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        return;
    } else if matches.opt_present("v") {
        println!("Rustcat v{}", __VERSION__);
        return;
    }

    if matches.opt_present("l") {
        let (opt_host, opt_port) = if matches.free.len() == 1 && matches.opt_present("p") {
            ("0.0.0.0", matches.free[0].as_str())
        } else if matches.free.len() == 2 {
            (matches.free[0].as_str(), matches.free[1].as_str())
        } else {
            print_help(&program, opts);
            ("", "")
        };

        let opts = Opts {
            host: opt_host,
            port: opt_port,
            transport: if matches.opt_present("u") {
                Protocol::Udp
            } else {
                Protocol::Tcp
            },
            mode: if matches.opt_present("H") {
                Mode::Beta
            } else {
                Mode::Normal
            },
        };

        if let Err(err) = listen(&opts) {
            print_error(&err.to_string());
            return;
        };
    } else {
        print_help(&program, opts);
        return;
    };
}
