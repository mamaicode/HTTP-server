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