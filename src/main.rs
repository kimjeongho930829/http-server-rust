#![allow(dead_code)]

use server::Server;
use wensite_handler::WebsiteHandler;

mod http;
mod server;
mod wensite_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
