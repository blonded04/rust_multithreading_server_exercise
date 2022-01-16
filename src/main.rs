use cppanywhere::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2004").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        run_binary("main");
        ("HTTP/1.1 200 OK", "html/markup.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn run_binary(filename: &str) {
    if cfg!(target_os = "windows") {
        Command::new(format!(".\\cpp\\{}", filename)).spawn().expect("No file.");
    } else {
        Command::new(format!("./cpp/{}", filename)).spawn().expect("No file.");
    }
}
