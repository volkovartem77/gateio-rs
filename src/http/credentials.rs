/// Gate.io API Credentials.
///
/// Communication with Gate.io API authenticated endpoints requires
/// valid API credentials. These credentials are used for HMAC SHA-512
/// signing of requests to ensure authenticity.
///
/// Note: Production and testnet API credentials are not interchangeable.
/// Make sure to use the appropriate credentials for your environment.
///
/// [API Documentation](https://www.gate.com/docs/developers/apiv4/#authentication)
///
#[derive(PartialEq, Eq, Clone)]
pub struct Credentials {
    /// API key for authentication
    pub api_key: String,
    /// API secret for HMAC signing
    pub api_secret: String,
}

impl Credentials {
    /// Creates new API credentials
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Credentials {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &"api_secret")
            .finish()
    }
}
