#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() 
{      
    // Passing IP and Port by String, server listening localhost
    let server = Server::new("127.0.0.1:8080".to_string());       
    server.run(WebsiteHandler);
}

