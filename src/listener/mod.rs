/*
Name: listener.rs
Description: Listens on given arguments.
*/

use super::utils;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

#[cfg(unix)]
mod termios_handler;

fn print_started_listen(opts: &utils::Opts) {
    println!("Listening on {}:{}", opts.host.green(), opts.port.cyan());
}

fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where
    R: std::io::Read + Send + 'static,
    W: std::io::Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = match r.read(&mut buffer) {
                Ok(t) => t,
                Err(err) => {
                    utils::print_error(err);
                    std::process::exit(0);
                }
            };
            if len == 0 {
                println!("\n{} Connection lost", "[-]".red());
                std::process::exit(0);
            }
            match w.write_all(&buffer[..len]) {
                Ok(_) => (),
                Err(err) => {
                    utils::print_error(err);
                    std::process::exit(0);
                }
            };
            w.flush().unwrap();
        }
    })
}

fn listen_tcp_normal(stream: std::net::TcpStream) -> std::io::Result<()> {
    let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
    let t2 = pipe_thread(stream, std::io::stdout());
    t1.join().unwrap();
    t2.join().unwrap();
    Ok(())
}

/* Listen on given host and port */
pub fn listen(opts: &utils::Opts) -> std::io::Result<()> {
    match opts.transport {
        utils::Protocol::Tcp => {
            let listener = std::net::TcpListener::bind(format!("{}:{}", opts.host, opts.port))?;
            print_started_listen(opts);

            let (mut stream, _) = listener.accept()?;
            match opts.mode {
                utils::Mode::History => {
                    if cfg!(unix) {
                        #[cfg(unix)]
                        termios_handler::setup_fd()?;
                        listen_tcp_normal(stream)?;
                    } else {
                        let t = pipe_thread(stream.try_clone().unwrap(), std::io::stdout());
                        let mut rl = Editor::<()>::new();
                        loop {
                            let readline = rl.readline(">> ");
                            match readline {
                                Ok(command) => {
                                    rl.add_history_entry(command.as_str());
                                    let command = command.clone() + "\n";
                                    stream
                                        .write_all(command.as_bytes())
                                        .expect("Faild to send TCP.");
                                }
                                Err(ReadlineError::Interrupted) => {
                                    break;
                                }
                                Err(ReadlineError::Eof) => {
                                    break;
                                }
                                Err(err) => {
                                    utils::print_error(err);
                                    break;
                                }
                            }
                        }
                        t.join().unwrap();
                    }
                }
                utils::Mode::Normal => {
                    listen_tcp_normal(stream)?;
                },
            }
        }

        utils::Protocol::Udp => {
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
                            break;
                        }
                        Err(ReadlineError::Eof) => {
                            break;
                        }
                        Err(err) => {
                            utils::print_error(err);
                            break;
                        }
                    }
                }
            }
        }
    }
    return Ok(());
}
