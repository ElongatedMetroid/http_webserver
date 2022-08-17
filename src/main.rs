use std::{net::TcpListener, process};

fn main() {
    // Create a new TcpListener
    let listener = 
        // Returns a new TcpListener instance that will be bound to the port 7878
        match TcpListener::bind("127.0.0.1:7878") {
            Ok(v) => v,
            Err(e) => {
                println!("Port could not be bound: {e}");
                process::exit(1);
            }
        };

    // Iterate through each each stream between the client and the server, this
    // could also be seen as iterating between each connection attempt.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
