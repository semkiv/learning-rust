use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // in fact we're iterating over connection attempts (not connections), which might be unsuccessful, hence `unwrap`

        handle_connection(stream);
    }
}

// note that `stream` parameter is mutable; the reason is that `TcpStream` instance keeps track of what data it returns to us internally, it might read more data than we asked for and save that data for the next time we ask for data - it therefore needs to be `mut` because its internal state might change
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "resources/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "resources/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // `flush` will wait and prevent the program from continuing until all the bytes are written to the connection - `TcpStream` contains an internal buffer to minimize calls to the underlying operating system
}
