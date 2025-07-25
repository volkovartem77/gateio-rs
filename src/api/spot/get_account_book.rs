use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving spot account transaction history
pub struct GetAccountBook {
    /// Currency to filter transactions by
    pub currency: Option<String>,
    /// Start timestamp for transaction history range
    pub from: Option<i64>,
    /// End timestamp for transaction history range
    pub to: Option<i64>,
    /// Page number for pagination
    pub page: Option<i32>,
    /// Maximum number of records to return per page
    pub limit: Option<i64>,
    /// Type of account book entry to filter by
    pub book_type: Option<String>,
    /// Specific transaction code to filter by
    pub code: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetAccountBook {
    /// Creates a new GetAccountBook request
    pub fn new() -> Self {
        Self {
            currency: None,
            from: None,
            to: None,
            page: None,
            limit: None,
            book_type: None,
            code: None,
            credentials: None,
        }
    }

    /// Sets the currency filter for transaction history
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets the start timestamp for the date range filter
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the end timestamp for the date range filter
    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sets the maximum number of records per page
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets the account book entry type filter
    pub fn book_type(mut self, book_type: &str) -> Self {
        self.book_type = Some(book_type.into());
        self
    }

    /// Sets the transaction code filter
    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetAccountBook> for Request {
    fn from(request: GetAccountBook) -> Request {
        let mut params = Vec::new();

        if let Some(currency) = request.currency {
            params.push(("currency".into(), currency.to_string()));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(page) = request.page {
            params.push(("page".into(), page.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(book_type) = request.book_type {
            params.push(("type".into(), book_type.to_string()));
        }

        if let Some(code) = request.code {
            params.push(("code".into(), code.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/account_book".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
