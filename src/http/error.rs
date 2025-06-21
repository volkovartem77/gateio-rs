use serde::Deserialize;
use std::collections::HashMap;

/// Unsuccessful response from the Gate API.
#[derive(Debug)]
pub enum ClientError {
    /// API server error complying with the error schema.
    Structured(HttpError<GateApiError>),
    /// API server error not complying with the error schema.
    Raw(HttpError<String>),
}

/// Generic Http Error
#[derive(Debug)]
pub struct HttpError<T> {
    /// Http status code
    pub status_code: u16,
    /// Response body content
    pub data: T,
    /// Response headers
    pub headers: HashMap<String, String>,
}

impl<T> HttpError<T> {
    pub fn new(status_code: u16, data: T, headers: HashMap<String, String>) -> Self {
        Self {
            status_code,
            data,
            headers,
        }
    }
}

/// Structured Gate server error
#[derive(Deserialize, Debug)]
pub struct GateApiError {
    /// Error code
    ///
    /// [API Documentation](https://www.gate.com/docs/developers/apiv4/#accountbook-code)

    /// non-2xx status code
    #[serde(rename(deserialize = "label"))]
    pub label: String,

    /// detailed error message
    #[serde(rename(deserialize = "message"))]
    pub message: String,
}
