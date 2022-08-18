use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    {process, fs, thread},
    time::Duration,
};

use colored::Colorize;

mod threadpool;
mod config;

use threadpool::{ThreadPool, PoolCreationError};
use config::{Config};

fn main() {
    let config = match Config::new("Config.toml") {
        Ok(c) => c,

        Err(e) => {
            eprintln!("{}", format!("Configuration file had an error: {e}, Attempting to fall back on `Default.toml`").red());
            
            match Config::new("Default.toml") {
                Ok(c) => {
                    eprintln!("{}", "Successfully fell back on `Default.toml`".green());
                    c
                },
                Err(e) => {
                    eprintln!("{}", format!("Fallback config `Default.toml` failed: {e}, Exiting").red());
                    process::exit(1);
                }
            }
        }
    };

    // Create a new TcpListener, the bind method will return a new TcpListener 
    // instance that will be bound to the port 7878
    let listener = TcpListener::bind(config.ip()).unwrap_or_else(|err| {
        eprintln!("Could not bind port: {err}");
        process::exit(1);
    });

    let pool = match ThreadPool::build(config.thread_count()) {
        Ok(p) => p,
        Err(e) => {
            match e {
                PoolCreationError::ZeroThreads => {
                    eprintln!(
                        "{}", 
                        "***** Attempted to create a thread pool with 0 threads. Upping thread count to 1 *****".red()
                    );

                    ThreadPool::build(1).unwrap()
                }
            }
        }
    };

    // Iterate through each each stream between the client and the server, this
    // could also be seen as iterating between each connection attempt.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Parse HTTP request and send back a response
fn handle_connection(mut stream: TcpStream) {
    // Create a new BufRead instance that wraps a mutable reference to the stream
    let buf_reader = BufReader::new(&mut stream);
    // Read the first line of the HTTP request
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (request_file, _) = request_line.split_at(request_line.len() - 8);

    println!("{}", request_file);



    // Check if the request
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            // Status line part of a response that uses HTTP version 1.1, has a 
            // status code of 200, and an OK reason phrase, no headers, and no body
            ("HTTP/1.1 200 OK", "index.html")
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Read the HTML document into a String
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    // Format the files contents as the body of the success response
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // Send the response data as bytes directly down the connection.
    stream.write_all(response.as_bytes()).unwrap();
}
