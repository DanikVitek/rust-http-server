use std::{error::Error, fmt};

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl<'buf> fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'buf> TryFrom<&'buf str> for Method {
    type Error = InvalidMethodString<'buf>;

    fn try_from(value: &'buf str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(InvalidMethodString(value)),
        }
    }
}

#[derive(Debug)]
pub struct InvalidMethodString<'buf>(&'buf str);

impl<'buf> InvalidMethodString<'buf> {
    pub fn str(&self) -> &str {
        return &self.0;
    }
}

impl<'buf> fmt::Display for InvalidMethodString<'buf> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidMethod: {}", self.str())
    }
}

impl<'buf> Error for InvalidMethodString<'buf> {}
