use crate::http::{Credentials, Method, request::Request};

pub struct GetInsuranceHistory {
    pub business: String,
    pub currency: String,
    pub from: i64,
    pub to: i64,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub credentials: Option<Credentials>,
}

impl GetInsuranceHistory {
    pub fn new(business: &str, currency: &str, from: i64, to: i64) -> Self {
        Self {
            business: business.to_owned(),
            currency: currency.to_owned(),
            from,
            to,
            limit: None,
            page: None,
            credentials: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetInsuranceHistory> for Request {
    fn from(req: GetInsuranceHistory) -> Request {
        let mut params = Vec::new();

        params.push(("business".to_owned(), req.business));
        params.push(("currency".to_owned(), req.currency));
        params.push(("from".to_owned(), req.from.to_string()));
        params.push(("to".to_owned(), req.to.to_string()));
        if let Some(limit) = req.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }
        if let Some(page) = req.page {
            params.push(("page".to_owned(), page.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/insurance_history".into(),
            params,
            payload: String::new(),
            x_gate_exp_time: None,
            credentials: req.credentials,
            sign: true,
        }
    }
}
