use std::{error::Error, fmt, str::FromStr};

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

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for Method {
    type Error = InvalidMethod;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
            _ => Err(InvalidMethod(value.to_string())),
        }
    }
}

impl FromStr for Method {
    type Err = InvalidMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Method::try_from(s)
    }
}

#[derive(Debug)]
pub struct InvalidMethod(String);

impl InvalidMethod {
    pub fn str(&self) -> &String {
        return &self.0;
    }
}

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidMethod: {}", self.str())
    }
}

impl Error for InvalidMethod {}
