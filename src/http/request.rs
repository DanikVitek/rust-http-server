use super::{
    method::{InvalidMethodString, Method},
    Headers, HeadersError, QueryString,
};
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
    headers: Headers<'buf>,
    body: Option<&'buf str>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &'buf str {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{method} {path}{query_string} HTTP/1.1{headers}{body}",
            method = &self.method,
            path = &self.path,
            query_string = &self
                .query_string
                .as_ref()
                .map(|s| format!("?{}", s))
                .unwrap_or(String::default()),
            headers = if self.headers.is_empty() {
                String::default()
            } else {
                format!("\r\n{}", &self.headers)
            },
            body = if let Some(body) = &self.body {
                format!("\r\n\r\n{}", body)
            } else {
                String::default()
            },
        )
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError<'buf>;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(bytes: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(bytes)?.trim_end_matches('\0').trim();

        let (method, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;
        let (path_n_query, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;
        let (protocol, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest(request))?;
        let (headers, body) = extract_headers_and_body(request);

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol(protocol));
        }

        let mut path_n_query = path_n_query.split('?');

        let method: Method = method.try_into()?;
        let path = path_n_query
            .next()
            .ok_or(ParseError::InvalidRequest(request))?;
        let query_string = path_n_query.next().map(|s| s.into());
        let headers = headers.try_into()?;

        Ok(Self {
            method,
            path,
            query_string,
            headers,
            body: if body.is_empty() { None } else { Some(body) },
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, ch) in request.chars().enumerate() {
        if !ch.is_ascii_whitespace() {
            continue;
        }

        let word = &request[..i];
        let rest = &request[i + ch.len_utf8()..];
        if word.is_empty() {
            return get_next_word(rest);
        }
        return Some((word, rest));
    }

    None
}

fn extract_headers_and_body(request: &str) -> (&str, &str) {
    match request.find("\r\n\r\n") {
        Some(ind) => {
            let (headers, skip_and_body) = request.split_at(ind);
            (headers, &skip_and_body[4..])
        }
        None => (request, ""),
    }
}

#[derive(Debug)]
pub enum ParseError<'buf> {
    InvalidEncoding,
    InvalidRequest(&'buf str),
    InvalidMethod(&'buf str),
    InvalidProtocol(&'buf str),
    InvalidHeaders(&'buf str),
}

impl<'buf> From<Utf8Error> for ParseError<'buf> {
    #[inline]
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl<'buf> From<InvalidMethodString<'buf>> for ParseError<'buf> {
    #[inline]
    fn from(e: InvalidMethodString<'buf>) -> Self {
        ParseError::InvalidMethod(&e.0)
    }
}

impl<'buf> From<HeadersError<'buf>> for ParseError<'buf> {
    #[inline]
    fn from(e: HeadersError<'buf>) -> Self {
        ParseError::InvalidHeaders(&e.0)
    }
}

impl<'buf> ParseError<'buf> {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidEncoding => String::from("Invalid Encoding"),
            Self::InvalidRequest(e) => format!("Invalid Request: {}", e),
            Self::InvalidMethod(e) => format!("Invalid Method: {}", e),
            Self::InvalidProtocol(e) => format!("Invalid Protocol: {}", e),
            Self::InvalidHeaders(e) => format!("Invalid Headers: {}", e),
        }
    }
}

impl<'buf> Display for ParseError<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'buf> Error for ParseError<'buf> {}
