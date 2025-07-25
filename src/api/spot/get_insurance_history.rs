use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving insurance history records
pub struct GetInsuranceHistory {
    /// Business type for insurance history query
    pub business: String,
    /// Currency to filter insurance records by
    pub currency: String,
    /// Start timestamp for the date range
    pub from: i64,
    /// End timestamp for the date range
    pub to: i64,
    /// Maximum number of records to return per page
    pub limit: Option<u32>,
    /// Page number for pagination
    pub page: Option<u32>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetInsuranceHistory {
    /// Creates a new GetInsuranceHistory request with required parameters
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

    /// Sets the maximum number of records per page
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the API credentials for authentication
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
