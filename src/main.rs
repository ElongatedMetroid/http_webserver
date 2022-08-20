#![feature(is_some_with)]

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    {process, fs},
};

use colored::Colorize;

mod threadpool;
mod config;
mod http;

use threadpool::{ThreadPool, ZeroThreads};
use config::Config;
use http::{Request};

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
            eprintln!("{e}");

            eprintln!(
                "{}", 
                "***** Attempted to create a thread pool with 0 threads. Upping thread count to 1 *****".red()
            );

            ThreadPool::build(1).unwrap()
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
    let request = Request::parse(&mut stream);

    // TEMP EMPTY REQUEST CHECK
    if request.file().is_none() {
        return;
    }

    // Read the HTML document into a String
    let contents = fs::read_to_string(request.file().as_ref().unwrap()).unwrap();
    let length = contents.len();
    
    // Format the files contents as the body of the success response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // Send the response data as bytes directly down the connection.
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
