use std::io::prelude::*; // read/write to stream
use std::net::TcpListener; // listen for requests
use std::net::TcpStream; // single stream format
use std::fs; // get html file for response

fn main() {
    // .bind() returns Result<T, E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // stream = single open client server conneciton attempt
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Single open connection between client & server
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1024 bytes on stack
    stream.read(&mut buffer).unwrap(); // read bytes from stream

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("{}, {}", status_line, filename);

    // HTML file for response
    let contents = fs::read_to_string(filename).unwrap();

    // Formatting for response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();  // send it
    stream.flush().unwrap(); // end single connection
}
