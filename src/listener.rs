/*
Name: listener.rs
Description: Listens on given arguments.
*/

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};
use termion::color;

use super::options;

/* Print when started listening */
fn print_started_listen(opts: &options::Opts) {
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
pub fn listen(opts: &options::Opts) -> std::io::Result<()> {
    match opts.transport {
        options::Protocol::Tcp => {
            let listener = std::net::TcpListener::bind(format!("{}:{}", opts.host, opts.port))?;
            print_started_listen(opts);

            let (mut stream, _) = listener.accept()?;
            match opts.mode {
                options::Mode::Normal => {
                    let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
                    let t2 = pipe_thread(stream, std::io::stdout());
                    t1.join().unwrap();
                    t2.join().unwrap();
                }
                options::Mode::History => {
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
                                println!("Error: {:?}", err.to_string());
                                break;
                            }
                        }
                    }
                    t.join().unwrap();
                }
            }
        }

        options::Protocol::Udp => {
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
                                socket.send_to(command.as_bytes(), addr)?;
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
                            break;
                        }
                    }
                }
            }
        }
    }
    return Ok(());
}
