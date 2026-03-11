use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
mod thread_pool;
use thread_pool::ThreadPool;

fn main() {
    test_thread_pool();
}

fn test_tcp() {
    let address = "0.0.0.0:7878";
    let mut listener = TcpListener::bind(address).unwrap();
    println!("连接服务器成功，等待连接... 在 {} 上监听", address);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        thread::spawn(|| handle_connection(stream));
    }

    println!("服务器关闭, 不再监听连接");
}

fn test_thread_pool() {
    let address = "0.0.0.0:7878";
    let mut listener = TcpListener::bind(address).unwrap();
    println!("连接服务器成功，等待连接... 在 {} 上监听", address);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("服务器关闭, 不再监听连接, 这里不可能执行到");
}

fn handle_connection(mut stream: TcpStream) {
    println!("处理连接...");

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("HTTP 请求: {:#?}", http_request);

    let request_line = &http_request[0];

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK\r\n", "examples/statics/hello.html")
    // }else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n","examples/statics/404.html")
    // };

    let (status_line, filename) = match &request_line[..] {
        // 这里为什么要用 引用  [..] , 而上面不用,   request_line 是 &String, 但下面的常量   "GET / HTTP/1.1" 则是一个  &static str， 不同的类型，
        //let (status_line, filename) = match request_line {  // 这里为什么要用 引用  [..] , 而上面不用
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "examples/statics/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "examples/statics/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND\r\n", "examples/statics/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}Content-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
