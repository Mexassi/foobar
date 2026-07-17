use std::{fs, io::{BufRead, BufReader, Write}, net::TcpListener};
use foobar::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let _stream = stream.unwrap();
        pool.execute(|| handle_stream(_stream));
    }
}

fn handle_stream(mut stream: std::net::TcpStream) {
  let buffer_reader = BufReader::new(&stream);
  let http_request: Vec<_> = buffer_reader
    .lines()
    .map(|line| line.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
  println!("Handling stream");

  let method = &http_request[0];

  println!("found {}", method);

  if method.contains("GET") {
    println!("should respond with html");
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
  } else {
    let status_line = "HTTP/1.1 404 Not Found";
    let response = format!("{status_line}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
  }

}
