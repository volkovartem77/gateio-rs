/// HTTP methods supported by the Gate.io API
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Method {
    /// GET method for retrieving data
    Get,
    /// POST method for creating resources
    Post,
    /// PUT method for updating resources
    Put,
    /// DELETE method for removing resources
    Delete,
    /// PATCH method for partial updates
    Patch,
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match self {
            Method::Post => "POST",
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
        }
    }
}
