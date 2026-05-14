use std::net::TcpStream;

pub fn run(addr: &str) {
    let stream = TcpStream::connect(addr).unwrap_or_else(|e| {
        eprintln!("[client] Cannot connect to {}: {}", addr, e);
        std::process::exit(1);
    });
    println!("[client] Connected to {}", addr);
}