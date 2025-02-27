use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming(){
        let  stream= stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let block = b"GET /block HTTP/1.1\r\n";

    let (status, filename)= 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(block){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let content = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            content.len(),
            content
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    
}
