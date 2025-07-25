use crate::http::error::{ClientError, GateApiError, HttpError};
use crate::hyper::Error;
use std::collections::HashMap;
use bytes::Bytes;
use http_body_util::BodyExt;

/// REST Response
#[derive(Debug)]
pub struct Response {
    inner_response: hyper::Response<http_body_util::combinators::BoxBody<Bytes, Box<dyn std::error::Error + Send + Sync>>>,
}

impl Response {
    pub async fn into_body_str(self) -> Result<String, Error> {
        let status = self.inner_response.status().as_u16();
        if 400 <= status {
            let headers: HashMap<String, String> =
                self.inner_response
                    .headers()
                    .iter()
                    .fold(HashMap::new(), |mut headers, (k, v)| {
                        headers.entry(k.as_str().to_owned()).or_insert_with(|| {
                            // Assume all Gate response headers can convert to String.
                            v.to_str()
                                .expect("Failed to convert response header value to string")
                                .to_owned()
                        });
                        headers
                    });

            let content = hyper_body_to_string(self.inner_response.into_body()).await?;
            if 500 <= status {
                Err(Error::Server(HttpError::new(status, content, headers)))
            } else {
                let client_error = match serde_json::from_str::<GateApiError>(&content) {
                    Ok(err) => ClientError::Structured(HttpError::new(status, err, headers)),
                    Err(_) => ClientError::Raw(HttpError::new(status, content, headers)),
                };

                Err(Error::Client(client_error))
            }
        } else {
            Ok(hyper_body_to_string(self.inner_response.into_body()).await?)
        }
    }
}

impl<B> From<hyper::Response<B>> for Response 
where
    B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(response: hyper::Response<B>) -> Response {
        let (parts, body) = response.into_parts();
        let boxed_body = body.map_err(|e| e.into()).boxed();
        Response {
            inner_response: hyper::Response::from_parts(parts, boxed_body),
        }
    }
}


async fn hyper_body_to_string<B>(body: B) -> Result<String, Error>
where
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    use http_body_util::BodyExt;
    
    // Collect body bytes
    let collected = body.collect()
        .await
        .map_err(|e| Error::Send(e.into()))?;
    let bytes = collected.to_bytes();

    // Convert to string
    let content = String::from_utf8(bytes.to_vec())
        .map_err(|e| Error::Send(Box::new(e)))?;

    Ok(content)
}
