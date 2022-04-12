// Exposing Request and Method types directly out of http module and not through submodules
pub use request::Request;
pub use method::Method;
// Exposing parse error
pub use request::ParseError;

pub mod request;
pub mod method;