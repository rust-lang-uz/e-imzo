use native_tls::TlsStream;
use std::io;
use std::net::TcpStream;
use thiserror::Error;
use tungstenite::ClientHandshake;

/// Result type for this crate's use.
///
/// Instead of using Rust's defaulted Result type, we abstracted
/// Result type to accept a single type parameter by defaulting
/// our error type
pub(crate) type Result<T, E = EIMZOError> = std::result::Result<T, E>;

/// E-IMZO crate's error type collection.
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

    /// To be used only if you get despaired.
    #[error("Something aggressive is going on")]
    Unknown,
}
