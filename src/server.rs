use std::{
    io::{Read, Result as IoResult},
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::http::{Request, Response, StatusCode};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub const fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn run(self) {
        let listener: TcpListener = TcpListener::bind((&self.host as &str, self.port)).unwrap();

        println!("Listening on {}:{}", &self.host, self.port);

        loop {
            match listener.accept() {
                Err(e) => println!("Failed to establish TCP connection: {}", e),

                Ok((stream, addr)) => Server::process_connection(stream, addr),
            }
        }
    }

    fn process_connection(mut stream: TcpStream, addr: SocketAddr) {
        println!("Established TCP connection with {}", addr);

        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Err(e) => println!("Failed to read the stream: {}", e),

            Ok(n) => {
                println!("Received {} bytes", n);
                if n == 0 {
                    return;
                }

                let response_result = match Request::try_from(&buf as &[u8]) {
                    Err(e) => {
                        println!("Failed to parse request: {}", e);
                        let bad_req = Response::new(
                            StatusCode::BadRequest,
                            Some("Failed to parse request".to_string()),
                        );
                        bad_req.send(&mut stream)
                    }

                    Ok(request) => Server::process_request(request, stream, addr)
                };

                if let Err(e) = response_result {
                    println!("Failed to send response: {}", e);
                }
            }
        };
    }

    fn process_request(request: Request, mut stream: TcpStream, _addr: SocketAddr) -> IoResult<()> {
        println!("{}", request);
        let response = Response::new(StatusCode::Ok, Some("<h1>IT WORKS!!!</h1>".to_string()));
        response.send(&mut stream)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("Shutting down server");
    }
}
