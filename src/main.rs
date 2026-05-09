mod message;

fn main() {
    let msg = message::Message::Join("alice".to_string());
    println!("{}", msg.encode());

    let decoded = message::Message::decode("JOIN:alice");
    println!("{:?}", decoded);
}