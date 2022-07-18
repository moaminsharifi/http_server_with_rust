use super::server::Handler;
use crate::http::Method;
use crate::http::Response;
use crate::http::StatusCode;
use std::fs;
pub struct WebsiteHandler {
    public_path: String,
}
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::request::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/test" => Response::new(StatusCode::Ok, self.read_file("test.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(
                        StatusCode::NotFound,
                        Some("<h1>Page not found</h1>".to_string()),
                    ),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
