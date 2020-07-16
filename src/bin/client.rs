use std::env;
use std::net::TcpStream;
use std::io::Write;

fn main(){
    //const USAGE:&str = "Usage: ./client ADDR";
    let args:Vec<String> = env::args().collect();
    //let _addr:String = args[1].trim().to_string();
    let mes:String = args[1].to_string();
    let mut stream = TcpStream::connect(
        //addr.as_str()
        "127.0.0.1:5000"
    ).unwrap();
    
    stream.write(mes.as_bytes()).expect("Writing error.");
    stream.flush().unwrap();
}