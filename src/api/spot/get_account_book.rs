use crate::http::{Credentials, Method, request::Request};

pub struct GetAccountBook {
    pub currency: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub page: Option<i32>,
    pub limit: Option<i64>,
    pub book_type: Option<String>,
    pub code: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetAccountBook {
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

    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn book_type(mut self, book_type: &str) -> Self {
        self.book_type = Some(book_type.into());
        self
    }

    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.into());
        self
    }

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
