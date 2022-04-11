use super::method::Method;
// Modeling the data we work with, handling HTTP requests and returning HTTP responces 
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub struct Request
{   path: String,
    // Absence of a value in a type safe way without a fear of no pointer exceptions <String>
    query_string: Option<String>,                   
    method: Method,
}