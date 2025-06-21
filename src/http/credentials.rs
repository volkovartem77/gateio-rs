/// Binance API Credentials.
///
/// Communication with Binance API USER_DATA endpoints requires
/// valid API credentials.
///
/// Note: Production and TESTNET API Credentials are not
/// interchangeable.
///
/// [API Documentation](https://developers.binance.com/docs/rebate/quick-start#api-key-restrictions)
///
#[derive(PartialEq, Eq, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
}

impl Credentials {
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
