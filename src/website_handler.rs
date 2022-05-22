use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler
{
    public_path: String
}

impl WebsiteHandler
{
    pub fn new(public_path: String) -> Self{
                                             Self { public_path }
                                           }
    // Method to route, returning an option in case file does not exist
    fn read_file(&self, file_path: &str) -> Option<String>{
                                                            // Reading the public path and append the file path to it
                                                            let path = format!("{}/{}", self.public_path, file_path);
                                                            // Read file to string
                                                            fs::read_to_string(path).ok()                                                                                                      
                                                          } 


}

impl Handler for WebsiteHandler
{
    fn handle_request(&mut self, request: &Request) -> Response{
                                                                  match request.method(){
                                                                                            // Handling GET requests
                                                                                            Method::GET => match request.path(){
                                                                                                                                  "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                                                                                                                                  "/user" => Response::new(StatusCode::Ok, self.read_file("user.html")),
                                                                                                                                  _ => Response::new(StatusCode::NotFound, None),      
                                                                                                                               }

                                                                                            _ => Response::new(StatusCode::NotFound, None),
                                                                                        }
                                                               }
}