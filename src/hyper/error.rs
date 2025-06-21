use crate::http::error::{ClientError, HttpError as GateHttpError};
use http::{uri::InvalidUri, Error as HttpError};
use hyper::Error as HyperError;

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
    Send(HyperError),
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::Parse(err.into())
    }
}
