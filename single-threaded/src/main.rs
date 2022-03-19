/**
 * Building a Single-Threaded Web Server.
 * For more info: https://doc.rust-lang.org/book/ch20-01-single-threaded.html#validating-the-request-and-selectively-responding
 */
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Listening to the TCP Connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        // match stream {
        //     Ok(stream) => {
        //         println!("New connection established.")
        //     }
        //     Err(e) => {
        //         println!("Failed to establish connection.")
        //     }
        // }
    }
}

// Validations the Request and Selectively Responding
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     // Reading the Request
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

//     let get = b"GET / HTTP/1.1\r\n";

//     if buffer.starts_with(get) {
//         let contents = fs::read_to_string("public/index.html").unwrap();

//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//             contents.len(),
//             contents
//         );

//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//         let contents = fs::read_to_string("public/404.html").unwrap();

//         let response = format!("{}{}", status_line, contents);

//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

// A Touch of Refactoring
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
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
