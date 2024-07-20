use colored::Colorize;
use rustyline::error::ReadlineError;
use std::io::{stdin, stdout, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread::{self, JoinHandle};

#[cfg(unix)]
mod termios_handler;

#[cfg(unix)]
use signal_hook::{consts, iterator::Signals};

pub struct Opts {
    pub host: String,
    pub port: String,
    pub exec: Option<String>,
    pub block_signals: bool,
    pub mode: Mode,
}

pub enum Mode {
    Normal,
    Interactive,
    LocalInteractive,
}

fn print_connection_received() {
    log::info!("Connection Received");
}

// It will complain on unix systems without this lint rule.
#[allow(dead_code)]
fn print_feature_not_supported() {
    log::error!("This feature is not supported on your platform");
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
                    log::warn!("Connection lost");

                    exit(0);
                }
                Ok(len) => {
                    if let Err(err) = w.write_all(&buffer[..len]) {
                        log::error!("{}", err);

                        exit(1);
                    }
                }
                Err(err) => {
                    log::error!("{}", err);

                    exit(1);
                }
            }

            w.flush().unwrap();
        }
    })
}

fn listen_tcp_normal(stream: TcpStream, opts: &Opts) -> Result<()> {
    if let Some(exec) = &opts.exec {
        stream
            .try_clone()?
            .write_all(format!("{}\n", exec).as_bytes())?;
    }

    let (stdin_thread, stdout_thread) = (
        pipe_thread(stdin(), stream.try_clone()?),
        pipe_thread(stream, stdout()),
    );

    print_connection_received();

    stdin_thread.join().unwrap();
    stdout_thread.join().unwrap();

    Ok(())
}

fn block_signals(should_block: bool) -> Result<()> {
    if should_block {
        #[cfg(unix)]
        {
            Signals::new(&[consts::SIGINT])?;
        }

        #[cfg(not(unix))]
        {
            print_feature_not_supported();

            exit(1);
        }
    }

    Ok(())
}
// Listen on given host and port
pub fn listen(opts: &Opts) -> rustyline::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", opts.host, opts.port))?;

    #[cfg(not(unix))]
    {
        if let Mode::Interactive = opts.mode {
            print_feature_not_supported();

            exit(1);
        }
    }

    log::info!("Listening on {}:{}", opts.host.green(), opts.port.cyan());

    let (mut stream, _) = listener.accept()?;

    match &opts.mode {
        Mode::Interactive => {
            // It exists it if isn't unix above
            block_signals(opts.block_signals)?;

            #[cfg(unix)]
            {
                termios_handler::setup_fd()?;
                listen_tcp_normal(stream, opts)?;
            }
        }
        Mode::LocalInteractive => {
            let t = pipe_thread(stream.try_clone()?, stdout());

            print_connection_received();

            readline_decorator(|command| {
                stream
                    .write_all((command + "\n").as_bytes())
                    .expect("Failed to send TCP.");
            })?;

            t.join().unwrap();
        }
        Mode::Normal => {
            block_signals(opts.block_signals)?;
            listen_tcp_normal(stream, opts)?;
        }
    }

    Ok(())
}

/* readline_decorator takes in a function, A mutable closure
 * which will perform the sending of data depending on the transport protocol. */
fn readline_decorator(mut f: impl FnMut(String)) -> rustyline::Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(command) => {
                rl.add_history_entry(command.clone().as_str())?;
                f(command);
            }
            Err(err) => match err {
                ReadlineError::Interrupted | ReadlineError::Eof => exit(0),
                err => {
                    log::error!("{}", err);

                    exit(1);
                }
            },
        }
    }
}
