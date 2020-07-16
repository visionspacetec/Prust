use std::io::prelude::*;
//use std::env;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //let args:Vec<String> = env::args().collect();
    //let _addr:String = args[1].trim().to_string();
    
    let listener = TcpListener::bind(
        //addr.as_str()
        "127.0.0.1:5000"
    ).unwrap();

    for stream in listener.incoming() {
        let stream :TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));
}