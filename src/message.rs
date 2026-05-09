/// A chat message sent over the wire.
/// Wire format (newline-delimited):
///   JOIN:<username>\n
///   SAY:<username>:<text>\n
///   LEAVE:<username>\n
///   SERVER:<text>\n
///   ERROR:<text>\n
#[derive(Debug, Clone)]
pub enum Message {
    Join(String),
    Say { from: String, text: String },
    Leave(String),
    Server(String),
    Error(String),
}

impl Message {
    /// Serialize to the wire format string (without trailing newline).
    pub fn encode(&self) -> String {
        match self {
            Message::Join(name)           => format!("JOIN:{}", name),
            Message::Say { from, text }   => format!("SAY:{}:{}", from, text),
            Message::Leave(name)          => format!("LEAVE:{}", name),
            Message::Server(text)         => format!("SERVER:{}", text),
            Message::Error(text)          => format!("ERROR:{}", text),
        }
    }

    /// Parse a wire format line back into a Message.
    pub fn decode(line: &str) -> Option<Message> {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("JOIN:") {
            return Some(Message::Join(name.to_string()));
        }
        if let Some(rest) = line.strip_prefix("SAY:") {
            // SAY:<from>:<text> — text may itself contain colons
            if let Some((from, text)) = rest.split_once(':') {
                return Some(Message::Say {
                    from: from.to_string(),
                    text: text.to_string(),
                });
            }
        }
        if let Some(name) = line.strip_prefix("LEAVE:") {
            return Some(Message::Leave(name.to_string()));
        }
        if let Some(text) = line.strip_prefix("SERVER:") {
            return Some(Message::Server(text.to_string()));
        }
        if let Some(text) = line.strip_prefix("ERROR:") {
            return Some(Message::Error(text.to_string()));
        }
        None
    }
}