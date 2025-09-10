use std::{
    collections::HashMap,
    io::{Read, Cursor},
};

#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a str,
    pub uri: &'a str,
    pub protocol: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub content: Vec<u8>,
}

impl<'a> Request<'a> {
    pub fn parse(request: &'a str) -> Result<Request<'a>, Error> {
        let mut lines = request.split("\n");
        let request_line = lines
            .next()
            .ok_or(Error::EmptyRequest)?;

        let request_line = request_line.strip_suffix("\r").unwrap_or(request_line);

        let mut request_parts= request_line.split_whitespace();
        let method = request_parts.next().ok_or(Error::InvalidStatusLine)?;
        let uri = request_parts.next().ok_or(Error::InvalidStatusLine)?;
        let protocol = request_parts.next().ok_or(Error::InvalidStatusLine)?;

        let mut headers = HashMap::new();

        for line in &mut lines {
            if line == "\r" || line.is_empty() {
                break;
            }

            let line = line.strip_suffix("\r").unwrap_or(line);

            if let Some((k, v)) = line.split_once(": ") {
                headers.insert(k, v);
            }
        }

        let length = match headers.get("Content-Length") {
            Some(v) => v.parse::<usize>().map_err(|_| Error::InvalidHeader)?,
            None => 0,
        };

        let mut content = vec![0u8; length];

        let joined = lines.collect::<Vec<&str>>().join("\n");
        let mut reader = Cursor::new(joined.as_bytes());

        let n = reader.read(&mut content).map_err(|_| Error::ContentReadError)?;
        content.truncate(n);

        Ok(Request {
            method,
            uri,
            protocol,
            headers,
            content
        })
    }
}

pub struct Response {
    pub protocol: &'static str,
    pub status_code: &'static str,
    pub reason_phrase: &'static str,
    pub headers: HashMap<&'static str, &'static str>,
    pub content: Vec<u8>,
    _response: String
}

#[derive(Debug)]
pub enum Error {
    ParseError,
    EmptyRequest,
    InvalidHeader,
    InvalidStatusLine,
    ContentReadError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError => write!(f, "There was an error parsing the request"),
            Error::EmptyRequest => write!(f, "The provided request was empty"),
            Error::InvalidHeader => write!(f, "One or more headers was invalid"),
            Error::InvalidStatusLine => write!(f, "The status line of the request was invalid"),
            Error::ContentReadError => write!(f, "Could not read content from request")
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests;