#![allow(dead_code)]

mod server;
mod http;

use server::Server;

fn main() {
    let server = Server::new(String::from("localhost"), 24_133);
    server.run();
}
