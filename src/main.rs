use http::request::Request;
use server::Server;

fn main() 
{      
    // Passing IP and Port by String, server listening localhost
    let server = Server::new("127.0.0.1:8080".to_string());       
    server.run();
}

mod server
{
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
            println!("Listening on {}", self.addr)
        }

    }

}


mod http
{   
    pub mod request
    {
        use super::method:Method;
        // Modeling the data we work with, handling HTTP requests and returning HTTP responces 
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
        pub struct Request
        {   path: String,
            // Absence of a value in a type safe way without a fear of no pointer exceptions <String>
            query_string: Option<String>,                   
            method: Method,
        }
    }

    pub mod method
    {
        pub enum Method
        {
            GET,
            HEAD,
            POST,
            PUT,
            DELETE,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }    
}
