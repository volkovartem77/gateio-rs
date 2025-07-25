use crate::http::error::{ClientError, HttpError as GateHttpError};
use http::{Error as HttpError, uri::InvalidUri};

/// Communication error with the server.
#[derive(Debug)]
pub enum Error {
    /// 4XX error from the server.
    Client(ClientError),
    /// 5XX error from the server.
    Server(GateHttpError<String>),
    /// The format of the API secret is invalid.
    InvalidApiSecret,
    Parse(HttpError),
    Send(Box<dyn std::error::Error + Send + Sync>),
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::Parse(err.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Client(e) => write!(f, "Client error: {:?}", e),
            Error::Server(e) => write!(f, "Server error: {:?}", e),
            Error::InvalidApiSecret => write!(f, "Invalid API secret"),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
            Error::Send(e) => write!(f, "Send error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
