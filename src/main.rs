mod server;
mod message;

fn main() {
    server::run("127.0.0.1:8080");
    let msg = message::Message::Join("alice".to_string());
    println!("{}", msg.encode());

    let decoded = message::Message::decode("JOIN:alice");
    println!("{:?}", decoded);
}