use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

// Representing HTTP response 
// This struct stores the status code and the body
#[derive(Debug)]
pub struct Response
{ 
     // Stores the HTTP status code and the body
    status_code: StatusCode,
    // In case the body doesn't have a response we wrap an option around the string
    body: Option<String>,

}

// Constructor for the response
impl Response
{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
                                                                    // Creating a new response
                                                                    Response{status_code,body}
                                                                       }
    // Dynamically generating HTTP response from the response struct
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
                                                                  let body = match &self.body{
                                                                  
                                                                    Some(b) => b,
                                                                    None => " "
                                                                                             };
                                                                  // Writing to TCP stream directly                                                 
                                                                  write!(stream, 
                                                                    "HTTP/1.1 {} {}\r\n\r\n{}",
                                                                    self.status_code, 
                                                                    self.status_code.reason_phrase(),
                                                                    body)
                                                               }
}

