#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

use http::{Response, StatusCode};
use server::{Handler, Server};

fn main() {
    let server = Server::new(String::from("localhost"), 24_133);
    server.run(TestHandler);
}

struct TestHandler;
impl Handler for TestHandler {
    fn handle_request(&mut self, request: http::Request) -> http::Response {
        println!("{}", request);
        Response::new(StatusCode::Ok, Some("<h1>Hello World</h1>".to_string()))
    }
}
