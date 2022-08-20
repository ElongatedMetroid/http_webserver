use std::{net::TcpStream, io::{BufReader, Read}};

#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    http_ver: String,

    file: String,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Request {
        let mut buf_reader = BufReader::new(stream);
        let mut request = String::new();

        buf_reader.read_to_string(&mut request).unwrap();

        // Contains method, URI, and HTTP version
        let mut request_line = request
            .lines()
            .next()
            .unwrap()
            .split_whitespace();

        let mut request = Request {
            method: request_line.next().unwrap().into(),
            uri: request_line.next().unwrap().into(),
            http_ver: request_line.next().unwrap().into(),

            file: String::new(),
        };

        if request.uri == *"/" {
            request.file = String::from("index.html");
        } else {
            request.file = request.uri.clone();
            request.file.remove(0);
        }

        request
    }
    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn uri(&self) -> &str {
        &self.uri
    }
    pub fn http_ver(&self) -> &str {
        &self.http_ver
    }
    pub fn file(&self) -> &str {
        &self.file
    }
}