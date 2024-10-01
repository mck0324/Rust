use crate::http::{method, request};

use super::method::Method;
use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::path;
use std::str::Utf8Error; 

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf : &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        match get_next_word(reqeust) {
            Some((method, request)) => {},
            None => return  Err(ParseError::InvalidRequest),
        }
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        unimplemented!()
    }
}

fn get_next_word(reqeust: &str) -> Option<(&str, &str)> {
    for (i,c) in reqeust.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&reqeust[..i], &reqeust[i+1..]));
        }
    }
    None 

}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",    
        }

    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {
    
}
