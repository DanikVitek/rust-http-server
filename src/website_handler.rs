use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};
use std::{fs, path::MAIN_SEPARATOR};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!(
            "{public_path}{MAIN_SEPARATOR}{file_path}",
            public_path = &self.public_path,
            file_path = Self::to_file_path(file_path)
        );

        self.filter_valid_path(path)
    }

    fn filter_valid_path(&self, path: String) -> Option<String> {
        match fs::canonicalize(path) {
            Err(_) => None,
            Ok(canon_path) => {
                let public_path = fs::canonicalize(&self.public_path).unwrap();
                if !canon_path.starts_with(public_path) {
                    println!(
                        "Directory Traversal Attack Attempted: {}",
                        canon_path.display()
                    );
                    return None;
                }

                fs::read_to_string(canon_path).ok()
            }
        }
    }

    fn to_file_path(path: &str) -> String {
        path.replace('/', &MAIN_SEPARATOR.to_string() as &str)
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: Request) -> Response {
        dbg!(&request);
        println!("Request:\n{}", &request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, None, self.read_file("/index.html")),
                "/hello" => Response::new(StatusCode::Ok, None, self.read_file("/hello.html")),
                path => match self.read_file(path) {
                    Some(file_contents) => Response::new(StatusCode::Ok, None, Some(file_contents)),
                    None => Response::new(StatusCode::NotFound, None, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None, None),
        }
    }
}
