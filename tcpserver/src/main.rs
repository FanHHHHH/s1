use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("连接建立成功！");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
