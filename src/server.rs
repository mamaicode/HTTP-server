use std::io::Read;
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

        //https://doc.rust-lang.org/std/#modules
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
                                        stream.read(&mut buffer);
                                       },

                Err(e) => println!("Failed to establich a connection: {}", e),
            }
        }
    }
}



