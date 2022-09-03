use super::{Headers, StatusCode};
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response<'ans> {
    status: StatusCode,
    headers: Option<Headers<'ans>>,
    body: Option<String>,
}

impl<'ans> Response<'ans> {
    pub fn new(status: StatusCode, headers: Option<Headers<'ans>>, body: Option<String>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };
        write!(stream, "HTTP/1.1 {}\r\n\r\n{}", self.status, body)
    }
}
