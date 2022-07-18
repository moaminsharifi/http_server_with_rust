
#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;
use std::env;
use http::Method;
use http::request;
mod server;
mod http;
mod website_handler;
fn main() {
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path:String = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server_address = String::from("127.0.0.1:8080".to_string());
    let server:Server = Server::new(server_address);
    server.run(WebsiteHandler::new(public_path));
}



