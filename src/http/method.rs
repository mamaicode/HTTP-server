use std::str::FromStr;

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

impl FromStr for Method
{
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

// Creating a custom error type so that in our request we can convert error into a ParseError
pub struct MethodError;