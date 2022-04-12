use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;

// Modeling the data we work with, handling HTTP requests and returning HTTP responces 
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub struct Request
{   path: String,
    // Absence of a value in a type safe way without a fear of no pointer exceptions <String>
    query_string: Option<String>,                   
    method: Method,
}

impl TryFrom<&[u8]> for Request
{
    type Error = ParseError;

    // Parsing the request
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> 
    {
        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}

// Custom error type
pub enum ParseError
{
    InvalidRequest, 
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError
{
    fn message(&self) -> &str
    {
        match self
        {
            Self::InvalidRequest => "Invalid Request", 
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError
{
    fn from(_: Utf8Error) -> Self
    {
        Self::InvalidEncoding
    }
}

impl Display for ParseError
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult
    {
        // Writing to formatter
        write!(f, "{}", self.message())
    }

}

impl Debug for ParseError
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult
    {
        // Writing to formatter
        write!(f, "{}", self.message())
    }

}

impl Error for ParseError
{

}