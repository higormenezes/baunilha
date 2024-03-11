use std::{
    io::{BufRead, Error, ErrorKind, Read},
    net::TcpStream,
    str::FromStr,
};

#[derive(Debug)]
pub enum Method {
    GET,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMethodError;

impl FromStr for Method {
    type Err = ParseMethodError;

    fn from_str(method_str: &str) -> Result<Self, Self::Err> {
        match method_str.to_lowercase().as_str() {
            "get" => Ok(Method::GET),
            _ => Err(ParseMethodError),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol_version: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Request, Error> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let first_line = match buffer.lines().next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "No content found in the stream",
                ))
            }
        };

        let words: Vec<&str> = first_line.split_whitespace().collect();

        let method = match Method::from_str(words[0]) {
            Ok(m) => m,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Method not found in the stream",
                ))
            }
        };
        let path = String::from(words[1]);
        let protocol_version = String::from(words[2]);

        Ok(Request {
            method,
            path,
            protocol_version,
        })
    }
}
