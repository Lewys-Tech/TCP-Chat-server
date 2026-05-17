use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use crate::message::Message;

pub fn run(addr: &str) {
    let stream = TcpStream::connect(addr).unwrap_or_else(|e| {
        eprintln!("[client] Cannot connect to {}: {}", addr, e);
        std::process::exit(1);


    });
    println!("[client] Connected to {}", addr);

    let read_stream = stream.try_clone().unwrap();
    let mut write_stream = stream;

    //spawned thread - reads from server
    thread::spawn(move || {
        use crate::message::Message;
        let reader = BufReader::new(read_stream);
        for line in reader.lines() {
            let line = line.unwrap();
            match crate::message::Message::decode(&line){
            Some(Message::Say { from, text })  => println!("<{}> {}", from, text),     // <bob> Hello!
            Some(Message::Join(name))          => println!("*** {} joined", name),      // *** bob joined
            Some(Message::Leave(name))         => println!("*** {} left", name),        // *** bob left
            Some(Message::Server(text))        => println!("[server] {}", text),        // [server] welcome
            Some(Message::Error(text))         => println!("[error] {}", text),         // [error] bad input
            None                               => println!("{}", line),

            
            
        }  
            
        }


    });
    let stdin = io::stdin();
    for line in stdin.lock().lines(){
        let line = line.unwrap();

        let encoded = if line.starts_with("JOIN:") {
            format!("{}\n", line)
        } else {
            let msg = Message::Say { from:  String::new(),  text:  line};
            format!("{}\n", msg.encode())
        };
       
        write_stream.write_all(encoded.as_bytes()).unwrap();
    }
}