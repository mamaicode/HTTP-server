use super::StatusCode;

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
}