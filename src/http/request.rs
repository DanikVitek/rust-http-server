use super::{method::{InvalidMethodString, Method}, QueryString};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}{}",
            &self.method,
            &self.path,
            &self
                .query_string
                .as_ref()
                .map(|s| format!("?{}", s))
                .unwrap_or(String::default())
        )
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError<'buf>;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(bytes: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(bytes)?;

        let (method, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;
        let (path_n_query, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;
        let (protocol, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol(protocol));
        }

        let mut path_n_query = path_n_query.split('?');

        let method = method
            .try_into()
            .map_err(|_| ParseError::InvalidMethod(method))?;
        let path = path_n_query
            .next()
            .ok_or(ParseError::InvalidRequest(request))?;
        let query_string = path_n_query.next().map(|s| s.into());

        Ok(Self {
            method,
            path,
            query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, ch) in request.chars().enumerate() {
        if ch.is_ascii_whitespace() {
            let word = &request[..i];
            let rest = &request[i + ch.len_utf8()..];
            if word.is_empty() {
                return get_next_word(rest);
            }
            return Some((word, rest));
        }
    }

    None
}

#[derive(Debug)]
pub enum ParseError<'buf> {
    InvalidRequest(&'buf str),
    InvalidEncoding,
    InvalidProtocol(&'buf str),
    InvalidMethod(&'buf str),
}

impl<'buf> From<Utf8Error> for ParseError<'buf> {
    #[inline]
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl<'buf> From<&'buf InvalidMethodString<'buf>> for ParseError<'buf> {
    #[inline]
    fn from(e: &'buf InvalidMethodString) -> Self {
        ParseError::InvalidMethod(e.str())
    }
}

impl<'buf> ParseError<'buf> {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidRequest(e) => format!("Invalid Request: {}", e),
            Self::InvalidEncoding => String::from("Invalid Encoding"),
            Self::InvalidProtocol(e) => format!("Invalid Protocol: {}", e),
            Self::InvalidMethod(e) => format!("Invalid Method: {}", e),
        }
    }
}

impl<'buf> Display for ParseError<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'buf> Error for ParseError<'buf> {}
