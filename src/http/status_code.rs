// Check more info for status codes: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// Implementing most useful status codes

//Implementing Display
use std::fmt::{Display, Formatter, Result as FmtResult};

/* When we want to implement Copy we need to implement Clone trait as well because if a type implements Copy
It can have trivial implementation for Clone that would perform the same task as Copy */
#[derive(Copy, Clone, Debug)]
// If we will want to generate a response we can just cast StatusCode into an integer and send it down the socket
pub enum StatusCode
{
    // Mapping the variantes of enum with the actual status codes
        Ok = 200,
        BadRequest = 400, 
        NotFound = 404,
        BadGateaway = 502,
}
 
// Method to map the variants to the correct reason-phrase 
impl StatusCode
{   // Takes reference to self and returns a string slice because we want to return static strings
    pub fn reason_phrase(&self) -> &str{
                                            match self{
                                                        Self::Ok => "Ok",
                                                        Self::BadRequest => "Bad Rrequest",
                                                        Self::NotFound => "Not Found",
                                                        Self::BadGateaway => "Bad Gateaway",
                                                      }
                                       }
}

impl Display for StatusCode
{
    // Takes reference for self and formatter 
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
                                                    // Writing to the Formatter
                                                    write!(f, "{}", *self as u16 )
                                                 }
}

