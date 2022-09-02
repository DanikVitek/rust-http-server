use std::{net::{TcpListener, SocketAddr, TcpStream}, io::Read};

use crate::http::Request;

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
                
                Ok((stream, addr)) => Server::process_connection(stream, addr)
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
                if n == 0 { return; }

                match Request::try_from(&buf as &[u8]) {
                    Err(e) => println!("Failed to parse request: {}", e),

                    Ok(request) => Server::process_request(request, addr),
                }
            },
        };
    }

    fn process_request(request: Request, _addr: SocketAddr) {
        println!("{}", request);
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("Shutting down server");
    }
}
