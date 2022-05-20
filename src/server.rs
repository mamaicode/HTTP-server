use std::io::{Read, Write};
// Pulling request in our scope
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::TcpListener;

pub struct Server
{
    addr: String, 
}

// Implementation block for struct
impl Server
{
// Writing associative function
    pub fn new(addr: String) -> Self                           
    {
        Self { addr } 
    }      

// Implementing the run method
    pub fn run(self)
    {   
        println!("Listening on {}", self.addr);

        //https://doc.rust-lang.org/std/#modulesD
        // Creating TCP socket and binding it to the address that we want to use
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // An infinite loop, on every iteration checking for new connections
        loop
        {   
            match listener.accept()
            {
                Ok((mut stream, _)) => {
                                        // Allocating a buffer
                                        let mut buffer = [0; 1024];

                                        // Reading the byte the client has sent
                                        match stream.read(&mut buffer)
                                        {   Ok(_) => {  // Converting the buffer into actual text 
                                                        println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                                        
                                                        let response = match Request::try_from(&buffer[..])
                                                        {
                                                            Ok(request) => {

                                                                                dbg!(request);
                                                                                // Importing response
                                                                                Response::new(
                                                                                                StatusCode::Ok, 
                                                                                                Some("<h1>Working<h/1>".to_string()),
                                                                                             )
                                                                           }

                                                            Err(e) => {
                                                                        println!("Failed to parse a request: {}", e);
                                                                        Response::new(StatusCode::BadRequest, None)
                                                                      }
                                                        };
                                                        
                                                        if let Err(e) = response.send(&mut stream)
                                                        {
                                                            println!("Failed to send response: {}", e);
                                                        }

                                                     }

                                            Err(e) => println!{"Failed to read from connection: {}", e},
                                        }
                                       }

                Err(e) => println!("Failed to establich a connection: {}", e),
            }
        }
    }
}




