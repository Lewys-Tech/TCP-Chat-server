use std::net::TcpListener;
use std::io::{BufRead, BufReader};
pub fn run(addr: &str) {

    let listener = TcpListener::bind(addr).unwrap();


    println!("[server] Listening on {}", addr);


    for stream in listener.incoming() {
    let stream = stream.unwrap();
    println!("[server] New connection from {}", stream.peer_addr().unwrap());
    let reader = BufReader::new(&stream);
    
    for line in reader.lines() {
    let line = line.unwrap();
    println!("[server] Received: {}", line);
}
    }
}