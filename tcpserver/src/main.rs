use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listner = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listner.incoming() {
        let mut stream = stream.unwrap();
        println!("Connention established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
