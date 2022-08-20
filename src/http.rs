use std::{net::TcpStream, io::{BufReader, BufRead}};

#[derive(Debug)]
pub struct Request {
    method: Option<String>,
    uri: Option<String>,
    http_ver: Option<String>,

    file: Option<String>,
}

impl Request {
    /// Parse a TcpStream containing an HTTP request into a Request type
    pub fn parse(stream: &mut TcpStream) -> Request {
        // ----- Store the http request into a vector -----

        let buf_reader = BufReader::new(stream);

        let http_request: Vec<_> = buf_reader
            // Get an iterator over the liens
            .lines()
            // Unwrap each line (lines returns an iterator of Result<String, Error>)
            .map(|result| result.unwrap_or_default())
            // The end of an HTTP request is signaled by two newline characters
            // in a row, so we take lines until there is a line that is an empty
            // String.
            .take_while(|line| !line.is_empty())
            // Collect this into a vector
            .collect();

        if http_request.is_empty() {
            return Request {
                method: None,
                uri: None,
                http_ver: None,
                file: None,
            }
        }

        // ----- Set all fields relating to the request_line -----

        // Contains an iterator to the method, URI, and HTTP version
        let mut request_line = http_request[0].split_whitespace();

        let mut request = Request {
            method: request_line.next().map(|s| s.to_string()),
            uri: request_line.next().map(|s| s.to_string()),
            http_ver: request_line.next().map(|s| s.to_string()),

            file: None,
        };

        // ----- Setup the file field -----
        if request.uri().is_some_and(|s| s == "/") {
            // If the uri is / set the file to index.html
            request.file = Some(String::from("index.html"));
        } else if !request.uri().is_none() {
            // If it is not / and the uri is not empty set the file to the uri 
            // without the starting /
            request.file = request.uri.clone();
            request.file.as_mut().unwrap().remove(0);
        }

        request
    }
    /// Returns the method containtained inside an &Option<String>, if the method is
    /// empty None is returned
    pub fn method(&self) -> &Option<String> {
        &self.method
    }
    /// Returns the uri containtained inside an &Option<String>, if the uri is
    /// empty None is returned
    pub fn uri(&self) -> &Option<String> {
        &self.uri
    }
    /// Returns the method containtained inside an &Option<String>, if the http_ver
    /// is empty None is returned
    pub fn http_ver(&self) -> &Option<String> {
        &self.http_ver
    }
    /// Returns the file containtained inside an &Option<String>, if the file is
    /// empty None is returned
    pub fn file(&self) -> &Option<String> {
        &self.file
    }
}