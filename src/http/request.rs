use crate::http::{Credentials, Method};

/// HTTP request representation for Gate.io API calls
#[derive(PartialEq, Eq, Debug)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) params: Vec<(String, String)>,
    pub(crate) payload: String,
    pub(crate) x_gate_exp_time: Option<u128>,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) sign: bool,
}

impl Request {
    /// Returns the HTTP method for this request
    pub fn method(&self) -> &Method {
        &self.method
    }
    /// Returns the API path for this request
    pub fn path(&self) -> &str {
        &self.path
    }
    /// Returns the query parameters for this request
    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }
    /// Returns the request body payload
    pub fn payload(&self) -> &str {
        &self.payload
    }
    /// Returns the Gate expiration time if set
    pub fn x_gate_exp_time(&self) -> &Option<u128> {
        &self.x_gate_exp_time
    }
    /// Returns the API credentials if set
    pub fn credentials(&self) -> &Option<Credentials> {
        &self.credentials
    }
    /// Returns whether this request should be signed
    pub fn sign(&self) -> &bool {
        &self.sign
    }
}

/// API HTTP Request
///
/// A low-level request builder for API integration
/// decoupled from any specific underlying HTTP library.
pub struct RequestBuilder {
    method: Method,
    path: String,
    params: Vec<(String, String)>,
    payload: String,
    credentials: Option<Credentials>,
    x_gate_exp_time: Option<u128>,
    sign: bool,
}

impl RequestBuilder {
    /// Creates a new request builder with method and path
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_owned(),
            params: vec![],
            payload: "".to_owned(),
            x_gate_exp_time: None,
            credentials: None,
            sign: false,
        }
    }

    /// Append `params` to the request's query string. Parameters may
    /// share the same key, and will result in a query string with one or
    /// more duplicated query parameter keys.
    pub fn params<'a>(mut self, params: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        self.params.extend(
            params
                .into_iter()
                .map(|param| (param.0.to_owned(), param.1.to_owned())),
        );
        self
    }
    /// Sets the request body payload
    pub fn payload(mut self, payload: &str) -> Self {
        self.payload = payload.to_owned();
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Sets the Gate expiration time
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

    /// Marks this request to be signed with HMAC
    pub fn sign(mut self) -> Self {
        self.sign = true;
        self
    }
}

impl From<RequestBuilder> for Request {
    fn from(builder: RequestBuilder) -> Request {
        Request {
            method: builder.method,
            path: builder.path,
            params: builder.params,
            payload: builder.payload,
            x_gate_exp_time: builder.x_gate_exp_time,
            credentials: builder.credentials,
            sign: builder.sign,
        }
    }
}
