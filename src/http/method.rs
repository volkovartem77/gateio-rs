#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
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
