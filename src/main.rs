mod server;
mod message;
mod client;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()){
        Some("server") => server::run("127.0.0.1:8080"),
        Some("client") => client::run("127.0.0.1:8080"),
        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run -- server");
            eprintln!("  cargo run -- client");
            std::process::exit(1);
        }
    }
    client::run("127.0.0.1:8080");
    server::run("127.0.0.1:8080");
    let msg = message::Message::Join("alice".to_string());
    println!("{}", msg.encode());

    let decoded = message::Message::decode("JOIN:alice");
    println!("{:?}", decoded);
}