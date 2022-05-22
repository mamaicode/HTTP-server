#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() 
{   
    // Using the code on someone elses computer & passing format macro
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // Read the public path variable from the environment when the process starts up 
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);  
    println!("public path: {}", public_path);
    // Passing IP and Port by String, server listening localhost
    let server = Server::new("127.0.0.1:8080".to_string());       
    server.run(WebsiteHandler::new(public_path)); 
}

