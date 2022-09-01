use super::method::{InvalidMethod, Method};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
};

#[derive(Debug)]
pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}{}",
            &self.method,
            &self.path,
            &self
                .query
                .as_ref()
                .map(|s| format!("?{}", s))
                .unwrap_or(String::default())
        )
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(bytes)?;

        fn get_next_word(request: &str) -> Option<(&str, &str)> {
            for (i, ch) in request.chars().enumerate() {
                if ch.is_ascii_whitespace() {
                    return Some((&request[..i], &request[i + 1..]));
                }
            }

            None
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest(request.to_string()))?;
        let (path_n_query, request) = get_next_word(request).ok_or(ParseError::InvalidRequest(request.to_string()))?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest(request.to_string()))?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol(protocol.to_string()));
        }

        let mut path_n_query = path_n_query.split('?');

        Ok(Request {
            method: method.parse::<Method>()?,
            path: path_n_query.next().ok_or(ParseError::InvalidRequest(request.to_string()))?.to_string(),
            query: path_n_query.next().map(str::to_string),
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest(String),
    InvalidEncoding,
    InvalidProtocol(String),
    InvalidMethod(String),
}

impl From<Utf8Error> for ParseError {
    #[inline]
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl From<InvalidMethod> for ParseError {
    #[inline]
    fn from(e: InvalidMethod) -> Self {
        ParseError::InvalidMethod(e.str().clone())
    }
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidRequest(e) => format!("Invalid Request: {}", e),
            Self::InvalidEncoding => String::from("Invalid Encoding"),
            Self::InvalidProtocol(e) => format!("Invalid Protocol: {}", e),
            Self::InvalidMethod(e) => format!("Invalid Method: {}", e),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
