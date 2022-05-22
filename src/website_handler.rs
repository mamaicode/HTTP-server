use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler
{
    fn handle_request(&mut self, request: &Request) -> Response{
                                                                  match request.method(){
                                                                                            // Handling GET requests
                                                                                            Method::GET => match request.path(){
                                                                                                                                  "/" => Response::new(StatusCode::Ok, Some("<h1>OK</h1>".to_string())),
                                                                                                                                  "/user" => Response::new(StatusCode::Ok, Some("<h1>Welcome User</h1>".to_string())),
                                                                                                                                  _ => Response::new(StatusCode::NotFound, None),      
                                                                                                                               }

                                                                                            _ => Response::new(StatusCode::NotFound, None),
                                                                                        }
                                                               }
}