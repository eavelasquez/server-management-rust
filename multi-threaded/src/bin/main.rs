/**
 * Building a Multi-Threaded Web Server.
 * For more info: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
 */
use multithreadedrust::ThreadPool;
use std::fs;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Listening to the TCP Connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("200 OK", "public/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "public/index.html")
    } else {
        ("404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
