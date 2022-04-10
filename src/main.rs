fn main() 
{    // Passing IP and Port by String, server listening localhost
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];

    dbg!(&string);
    dbg!(string_slice);

    let server = Server::new("127.0.0.1:8080");       
    server.run();
}


struct Server
{
    addr: String, 
}

// Implementation block for struct
impl Server
{
    // Writing associative function
    fn new(addr: String) -> Self                           
    {
        Self
        {
            addr
        }
    }   
    
    // Implementing the run method
    fn run(self)
    {

    }

}