use super::StatusCode;
use std::{
    io::{Result as IoResult, Write},
};

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Self { status, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };
        write!(stream, "HTTP/1.1 {}\r\n\r\n{}", self.status, body)
    }
}
