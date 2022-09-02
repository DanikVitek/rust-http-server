#![allow(dead_code)]

mod http;
mod server;

use server::Server;

fn main() {
    let server = Server::new(String::from("localhost"), 24_133);
    server.run();
}
