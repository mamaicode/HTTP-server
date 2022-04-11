use super::method::Method;
use std::convert::TryFrom;

// Modeling the data we work with, handling HTTP requests and returning HTTP responces 
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub struct Request
{   path: String,
    // Absence of a value in a type safe way without a fear of no pointer exceptions <String>
    query_string: Option<String>,                   
    method: Method,
}

impl Request
{   
    // https://doc.rust-lang.org/std/convert/trait.TryInto.html
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}

}

impl TryFrom<&[u8]> for Request
{
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> 
    {
        unimplemented!()
    }

}