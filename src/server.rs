use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// one connected client

struct Peer {
    username: String,
    stream: TcpStream,
}

//shared server state
struct State {
    peers: HashMap<u64, Peer>,
    next_id: u64,
}

impl State {
    fn new() -> Self {
        State {
            peers: HashMap::new(),
            next_id: 0,
        }
    }
}


pub fn run(addr: &str) {
    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    let listener = TcpListener::bind(addr).unwrap();
    println!("[server] Listening on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[server] New connection from {}", stream.peer_addr().unwrap());

        let state = Arc::clone(&state);

        thread::spawn(move || {
            let reader = BufReader::new(&stream);
            for line in reader.lines() {
                let line = line.unwrap();
                println!("[server] Received: {}", line);
            }
        });
    }
}