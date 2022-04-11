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
    }
}



