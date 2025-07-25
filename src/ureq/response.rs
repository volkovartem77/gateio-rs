use crate::http::error::{ClientError, GateApiError, HttpError};
use crate::ureq::Error;
use std::collections::HashMap;
use ureq::Body;

/// REST Response
pub struct Response {
    inner_response: http::Response<Body>,
}

impl Response {
    pub fn new(inner_response: http::Response<Body>) -> Self {
        Self { inner_response }
    }

    /// Fetch the data received from the API.
    pub fn into_body_str(self) -> Result<String, Box<Error>> {
        let status = self.inner_response.status().as_u16();
        if 400 <= status {
            let headers: HashMap<String, String> =
                self.inner_response
                    .headers()
                    .iter()
                    .fold(HashMap::new(), |mut headers, (k, v)| {
                        headers
                            .entry(k.as_str().to_owned())
                            .or_insert_with(|| v.to_str().unwrap_or("").to_owned());
                        headers
                    });

            let (_, mut body) = self.inner_response.into_parts();
            let content = body
                .read_to_string()
                .expect("Response failed UTF-8 encoding.");
            if 500 <= status {
                Err(Box::new(Error::Server(HttpError::new(
                    status, content, headers,
                ))))
            } else {
                let client_error = match serde_json::from_str::<GateApiError>(&content) {
                    Ok(err) => ClientError::Structured(HttpError::new(status, err, headers)),
                    Err(_) => ClientError::Raw(HttpError::new(status, content, headers)),
                };

                Err(Box::new(Error::Client(client_error)))
            }
        } else {
            let (_, mut body) = self.inner_response.into_parts();
            Ok(body
                .read_to_string()
                .expect("Response failed UTF-8 encoding."))
        }
    }
}

impl From<http::Response<Body>> for Response {
    fn from(response: http::Response<Body>) -> Response {
        Response {
            inner_response: response,
        }
    }
}

impl From<Response> for http::Response<Body> {
    fn from(response: Response) -> http::Response<Body> {
        response.inner_response
    }
}
