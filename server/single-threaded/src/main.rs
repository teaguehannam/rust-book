use std::io::prelude::*; // read/write to stream
use std::net::TcpListener; // listen for requests
use std::net::TcpStream; // single stream format
use std::fs; // get html file for response
use std::str; // we are the parser now

// Single open connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 32]; // stack 32 bytes
    stream.read(&mut buffer).unwrap(); // write to buffer

    let get = b"GET / HTTP/1.1\r\n";
    let nutrients = b"GET /nutrients HTTP/1.1\r\n";

    println!("{:?}", str::from_utf8(&buffer));

    let (status_line, filename) = if buffer.starts_with(get) {
        // root directory
        ("HTTP/1.1 200 OK", "./public/index.html")
    } else if buffer.starts_with(nutrients) {
        // nutrients directory
        ("HTTP/1.1 200 OK", "./public/nutrients.json")
    } else {
        // no match
        ("HTTP/1.1 404 NOT FOUND", "./public/404.html")
    };
    // println!("{}, {}", status_line, filename);

    // HTML file for response
    let contents = fs::read_to_string(filename).unwrap();

    // Formatting for response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // send it
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // end stream
}

fn main() {
    // Result<T, E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // single client server connection attempt
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
