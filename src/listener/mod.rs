/*
Name: listener.rs
Description: Listens on given arguments.
*/

use super::utils::{self, print_error, Mode};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{stdin, stdout, Read, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::process::exit;
use std::thread::{self, JoinHandle};

#[cfg(unix)]
mod termios_handler;

fn print_started_listen(opts: &utils::Opts) {
    println!("Listening on {}:{}", opts.host.green(), opts.port.cyan());
}

fn print_connection_recieved() {
    println!("{} Connection Recived", "[+]".green());
}

fn pipe_thread<R, W>(mut r: R, mut w: W) -> JoinHandle<()>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        //w.write_all("echo test\n".as_bytes()).unwrap();>
        loop {
            match r.read(&mut buffer) {
                Ok(0) => {
                    println!("\n{} Connection lost", "[-]".red());
                    exit(0);
                }
                Ok(len) => {
                    if let Err(err) = w.write_all(&buffer[..len]) {
                        exit(print_error(err));
                    }
                }
                Err(err) => exit(print_error(err)),
            }
            w.flush().unwrap();
        }
    })
}

fn listen_tcp_normal(stream: TcpStream, opts: &utils::Opts) -> Result<()> {
    if let Some(exec) = &opts.exec {
        stream
            .try_clone()?
            .write_all(format!("{}\n", exec).as_bytes())?;
    }
    /*
    Using tuple assignment
    since t1 and t2 share the same type
    and are related identifiers
    */
    let (t1, t2) = (
        pipe_thread(stdin(), stream.try_clone()?),
        pipe_thread(stream, stdout()),
    );
    print_connection_recieved();
    t1.join().unwrap();
    t2.join().unwrap();
    Ok(())
}

/* Listen on given host and port */
pub fn listen(opts: &utils::Opts) -> Result<()> {
    match opts.transport {
        utils::Protocol::Tcp => {
            let listener = TcpListener::bind(format!("{}:{}", opts.host, opts.port))?;
            print_started_listen(opts);

            let (mut stream, _) = listener.accept()?;
            match &opts.mode {
                Mode::History => {
                    #[cfg(unix)]
                    {
                        termios_handler::setup_fd()?;
                        listen_tcp_normal(stream, opts)?;
                    }
                    print_error("Normal history is not supported on your platform");
                }
                Mode::LocalHistory => {
                    let t = pipe_thread(stream.try_clone()?, stdout());
                    print_connection_recieved();
                    readline_decorator(|command| {
                        stream
                            .write_all((command + "\n").as_bytes())
                            .expect("Failed to send TCP.");
                    })?;
                    t.join().unwrap();
                }
                Mode::Normal => {
                    listen_tcp_normal(stream, opts)?;
                }
            }
        }

        utils::Protocol::Udp => {
            // Can be made better probably...
            // Rustline is needed here because otherwise you can't delete characters
            let socket = UdpSocket::bind(format!("{}:{}", opts.host, opts.port))?;
            let socket_clone = socket.try_clone()?;

            print_started_listen(opts);

            use std::sync::{Arc, Mutex};
            let addr: Arc<Mutex<Option<SocketAddr>>> = Arc::from(Mutex::new(None));
            let addr_clone = addr.clone();

            std::thread::spawn(move || loop {
                let mut buffer = [0; 1024];
                if let Ok((len, src_addr)) = socket_clone.recv_from(&mut buffer) {
                    *addr_clone.lock().unwrap() = Some(src_addr);
                    stdout().write_all(&buffer[..len]).unwrap();
                    stdout().flush().unwrap();
                }
            });

            loop {
                readline_decorator(|command| {
                    if let Some(addr) = *addr.lock().unwrap() {
                        socket.send_to((command + "\n").as_bytes(), addr).unwrap();
                    }
                })?;
            }
        }
    }
    Ok(())
}

/* readline_decorator takes in a function, A mutable closure
 * which will perform the sending of data depending on the transport protocol. */
fn readline_decorator(mut f: impl FnMut(String)) -> Result<()> {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(">> ") {
            Ok(command) => {
                rl.add_history_entry(command.clone().as_str());
                f(command);
            }
            Err(err) => match err {
                ReadlineError::Interrupted | ReadlineError::Eof => exit(0),
                err => exit(utils::print_error(err)),
            },
        }
    }
}
