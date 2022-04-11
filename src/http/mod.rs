// Exposing Request and Method types directly out of http module and not through submodules
pub use request::Request;
pub use method::Method;

pub mod request;
pub mod method;