use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: Request) -> Response;

    fn handle_bad_request(&mut self, e: ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(
            StatusCode::BadRequest,
            Some("Failed to parse request".to_string()),
        )
    }
}

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub const fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener: TcpListener = TcpListener::bind((&self.host as &str, self.port)).unwrap();

        println!("Listening on {}:{}", &self.host, self.port);

        loop {
            match listener.accept() {
                Err(e) => println!("Failed to establish TCP connection: {}", e),

                Ok((stream, addr)) => Server::process_connection(stream, addr, &mut handler),
            }
        }
    }

    fn process_connection(mut stream: TcpStream, addr: SocketAddr, handler: &mut impl Handler) {
        println!("Established TCP connection with {}", addr);

        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Err(e) => println!("Failed to read the stream: {}", e),

            Ok(n) => {
                println!("Received {} bytes", n);
                if n == 0 {
                    return;
                }

                let response = match Request::try_from(&buf as &[u8]) {
                    Err(e) => handler.handle_bad_request(e),
                    Ok(request) => handler.handle_request(request),
                };

                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }
        };
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("Shutting down server");
    }
}
