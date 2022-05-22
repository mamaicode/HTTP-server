use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use super::{QueryString};

// Syntax for attribute -> #[derive(Debug)]
// Modeling the data we work with, handling HTTP requests and returning HTTP responces 
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug)]
pub struct Request<'buf> // 'buf is the lifetime to our buffer
{   path: &'buf str,
    // Absence of a value in a typesafe way without a fear of no pointer exceptions <String>
    query_string: Option<QueryString<'buf>>,                   
    method: Method,
}

// Implementing Getters for the request
impl<'buf> Request<'buf>
{
    // Naming the Getter after the field without prefixing it with the Get word
    pub fn ath(&self) -> &str{
                                &self.path
                             }
    // Another one for the method
    pub fn method(&self) -> &Method{
                                      &self.method
                                   }
    // Query string
    pub fn query_string(&self) -> Option<&QueryString>{
                                                         &self.query_string.as_ref()
                                                      }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>
{
    type Error = ParseError;

    // Parsing the request
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> 
    {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        // Supporting HTTP 1.1 only!
        if protocol != "HTTP/1.1"
        {
            return Err(ParseError::InvalidProtocol);
        }

        // Converting it into enum
        let method: Method = method.parse()?;

        let mut query_string = None;
        // Clean code to avoid empty match variables, if let syntax is used
        if let Some(i) = path.find('?')
        {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        // Creating a new request
        Ok(Self
        {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>
{   // Looping through string slice to find whitespace
    for (i, c) in request.chars().enumerate()
    {
        if c == ' ' || c == '\r'
        {   // Adding one byte to i to skip whitespace, as whitespace takes up 1 byte
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
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

impl From<MethodError> for ParseError
{
    fn from(_: MethodError) -> Self
    {
        Self::InvalidMethod
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

