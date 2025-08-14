use std::io::{Read, Write, Result};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::sync::Arc;

use rustls::{ClientConfig, ServerConfig, StreamOwned, ServerConnection, ClientConnection};
use rustls::pki_types::ServerName;
use udp_dtls::{DtlsAcceptor, DtlsConnector, Identity, Certificate, DtlsStream};

// Minimal wrapper to adapt UdpSocket to Read/Write for udp-dtls
#[derive(Debug)]
pub struct UdpSocketChannel(pub UdpSocket);

impl Read for UdpSocketChannel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.recv(buf)
    }
}

impl Write for UdpSocketChannel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.send(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn accept_tls(stream: TcpStream, config: Arc<ServerConfig>) -> Result<StreamOwned<ServerConnection, TcpStream>> {
    let conn = ServerConnection::new(config).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(StreamOwned::new(conn, stream))
}

pub fn connect_tls(stream: TcpStream, config: Arc<ClientConfig>, server_name: &str) -> Result<StreamOwned<ClientConnection, TcpStream>> {
    let server_name = ServerName::try_from(server_name.to_owned())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid server name"))?;
    let conn = ClientConnection::new(config, server_name)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(StreamOwned::new(conn, stream))
}

pub fn accept_dtls(socket: UdpSocket, identity: Identity) -> Result<DtlsStream<UdpSocketChannel>> {
    let acceptor = DtlsAcceptor::builder(identity)
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let channel = UdpSocketChannel(socket);
    let stream = acceptor.accept(channel)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    Ok(stream)
}

pub fn connect_dtls(socket: UdpSocket, addr: SocketAddr, identity: Identity, peer_cert: Certificate) -> Result<DtlsStream<UdpSocketChannel>> {
    let connector = DtlsConnector::builder()
        .identity(identity)
        .add_root_certificate(peer_cert)
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let channel = UdpSocketChannel(socket);
    let stream = connector.connect(&addr.to_string(), channel)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    Ok(stream)
}
