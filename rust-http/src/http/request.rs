use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf : &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {
    
}