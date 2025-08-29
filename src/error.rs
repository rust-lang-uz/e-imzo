use native_tls::TlsStream;
use std::io;
use std::net::TcpStream;
use thiserror::Error;
use tungstenite::ClientHandshake;

#[derive(Error, Debug)]
pub enum EIMZOError {
    #[error("A websocket server went down: {0}")]
    Io(#[from] io::Error),
    #[error("JsonParse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("UrlParseError  error: {0}")]
    UrlParseError(#[from] url::ParseError),
    // native_tls
    #[error("TlsError  error: {0}")]
    TlsError(#[from] native_tls::Error),
    #[error("TlsHandshakeError  error: {0}")]
    TlsHandshakeError(#[from] native_tls::HandshakeError<TcpStream>),
    #[error("HandshakeError error: {0}")]
    HandshakeError(#[from] tungstenite::HandshakeError<ClientHandshake<TlsStream<TcpStream>>>),
}
