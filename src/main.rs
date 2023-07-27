use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, spawn},
    time::Duration,
};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9900").unwrap();
    println!("Server UP and running on port : 9900 ");

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("connection established");
        pool.execute(|| {
            handleconnection(stream);
        });
    }
}

fn handleconnection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("request : {}", String::from_utf8_lossy(&buffer[..]));
    //

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // println!("{:#?}", get.chars());
    let (statusline, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let html_content = fs::read_to_string(filename).unwrap();

    let responce = format!(
        "{}\r\n{}\r\n\r\n{}",
        statusline,
        html_content.len(),
        html_content
    );

    stream.write(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}
