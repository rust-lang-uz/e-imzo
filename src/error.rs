use native_tls::TlsStream;
use std::fmt::{Display, Formatter};
use std::io;
use std::net::TcpStream;
use tungstenite::ClientHandshake;

#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    JsonParse(serde_json::Error),
    UrlParseError(url::ParseError),
    // native_tls
    TlsError(native_tls::Error),
    TlsHandshakeError(native_tls::HandshakeError<TcpStream>),
    HandshakeError(tungstenite::HandshakeError<ClientHandshake<TlsStream<TcpStream>>>),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Io(e) => write!(f, "IO error: {}", e),
            CustomError::JsonParse(e) => write!(f, "JSON Parse error: {}", e),
            CustomError::UrlParseError(e) => write!(f, "url error: {}", e),
            CustomError::TlsError(e) => write!(f, "Tls error: {}", e),
            CustomError::TlsHandshakeError(e) => write!(f, "TlsHandshakeError error: {}", e),
            CustomError::HandshakeError(e) => write!(f, "HandshakeError error: {}", e),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> Self {
        CustomError::Io(e)
    }
}
impl From<serde_json::Error> for CustomError {
    fn from(e: serde_json::Error) -> Self {
        CustomError::JsonParse(e)
    }
}

impl From<url::ParseError> for CustomError {
    fn from(e: url::ParseError) -> Self {
        CustomError::UrlParseError(e)
    }
}

impl From<native_tls::Error> for CustomError {
    fn from(e: native_tls::Error) -> Self {
        CustomError::TlsError(e)
    }
}

impl From<native_tls::HandshakeError<TcpStream>> for CustomError {
    fn from(e: native_tls::HandshakeError<TcpStream>) -> Self {
        CustomError::TlsHandshakeError(e)
    }
}

impl From<tungstenite::HandshakeError<ClientHandshake<TlsStream<TcpStream>>>> for CustomError {
    fn from(e: tungstenite::HandshakeError<ClientHandshake<TlsStream<TcpStream>>>) -> Self {
        CustomError::HandshakeError(e)
    }
}
