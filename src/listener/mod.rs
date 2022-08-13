/*
Name: listener.rs
Description: Listens on given arguments.
*/

use super::utils::{self, print_error, Mode};
use colored::Colorize;
use rustyline;
use rustyline::error::ReadlineError;
use std::io::{stdin, stdout, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
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


    let (stdin_thread, stdout_thread) = (
        pipe_thread(stdin(), stream.try_clone()?),
        pipe_thread(stream, stdout()),
    );

    print_connection_recieved();

    stdin_thread.join().unwrap();
    stdout_thread.join().unwrap();

    Ok(())
}

/* Listen on given host and port */
pub fn listen(opts: &utils::Opts) -> rustyline::Result<()> {
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

    Ok(())
}

/* readline_decorator takes in a function, A mutable closure
 * which will perform the sending of data depending on the transport protocol. */
fn readline_decorator(mut f: impl FnMut(String)) -> rustyline::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;

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
