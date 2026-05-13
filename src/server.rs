use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::Write;

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
            let id;
            {
            let mut s = state.lock().unwrap();
            id = s.next_id;
            s.next_id += 1;
            s.peers.insert(id, Peer {
                username: format!("anon#{}", id),
                stream: stream.try_clone().unwrap(),
            });
            println!("[server] Peer {} registered", id);
        }
            let reader = BufReader::new(&stream);
            for line in reader.lines() {
                let line = line.unwrap();

                if line.starts_with("JOIN:"){
                    let username = line.strip_prefix("JOIN:").unwrap().to_string();
                    let mut s = state.lock().unwrap();
                    if let Some(peer) = s.peers.get_mut(&id){
                        peer.username = username.clone();
                    }
                    println!("[server] Peer {} is now known as {}", id, username);
                    continue;
                }


                println!("[server] Received: {}", line);
            let mut s = state.lock().unwrap();
            for (_, peer) in s.peers.iter_mut() {
                let msg = format!("{}\n", line);
                let _ =peer.stream.write_all(msg.as_bytes());
            }

            }
        });
    }
}