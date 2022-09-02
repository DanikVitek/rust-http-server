#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

use http::{Response, StatusCode};
use server::{Handler, Server};
use std::env;
use website_handler::WebsiteHandler;

const DEFAULT_PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let public_path: String = env::var("PUBLIC_PATH").unwrap_or(format!(
        "{DEFAULT_PATH}{path_separator}public",
        path_separator = std::path::MAIN_SEPARATOR
    ));
    println!("public path: {}", public_path);

    let server = Server::new(String::from("localhost"), 24_133);
    server.run(WebsiteHandler::new(public_path));
}

struct TestHandler;
impl Handler for TestHandler {
    fn handle_request(&mut self, request: http::Request) -> http::Response {
        println!("{}", request);
        Response::new(StatusCode::Ok, Some("<h1>Hello World</h1>".to_string()))
    }
}
