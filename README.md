# tcp-chat 💬

A multi-client TCP chat server built from scratch in Rust — no frameworks, no async runtimes, no external crates. Just the standard library.

Built as a learning project to understand systems programming concepts: raw TCP sockets, OS threads, shared state across threads, and wire protocols.

---

## Demo

```
[server] Listening on 127.0.0.1:8080
[server] New connection from 127.0.0.1:52056
[server] Peer 0 registered
[server] Peer 0 is now known as alice
[server] Peer 1 registered
[server] Peer 1 is now known as bob
```

```
# alice's terminal          # bob's terminal
JOIN:alice                  JOIN:bob
                            Hello alice!
<bob> Hello alice!
Hey bob, welcome!
                            <alice> Hey bob, welcome!
```

---

## Features

- Multiple clients connected simultaneously, each on its own OS thread
- Usernames — set yours with `JOIN:<name>` when you connect
- Real-time broadcast — every message goes to all connected clients
- Custom wire protocol — human-readable, newline-delimited messages
- Single binary — run as server or client with a flag

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

### Build

```bash
git clone https://github.com/Lewys-Tech/tcp-chat
cd tcp-chat
cargo build --release
```

### Run the server

```bash
cargo run -- server
# or on a custom address:
cargo run -- server 0.0.0.0:8080
```

### Connect as a client

```bash
# open a new terminal
cargo run -- client

# or connect to a remote server:
cargo run -- client 192.168.1.10:8080
```

Open as many client terminals as you like — they all chat with each other in real time.

### First steps once connected

```
JOIN:alice        ← set your username (do this first)
Hello everyone!   ← start chatting
/quit             ← disconnect
```

---

## Project Structure

```
src/
├── main.rs       Entry point — parses CLI args, runs server or client
├── message.rs    Wire protocol — Message enum, encode() and decode()
├── server.rs     TCP server — accepts connections, manages peers, broadcasts
└── client.rs     TCP client — reads stdin, prints incoming messages
```

Each file has one job. No file knows more than it needs to.

---

## Wire Protocol

Messages are newline-delimited UTF-8 strings. Each line on the network is one of:

| Direction       | Format              | Meaning                     |
|-----------------|---------------------|-----------------------------|
| client → server | `JOIN:<name>`       | Set your username           |
| client → server | `SAY::<text>`       | Send a chat message         |
| client → server | `LEAVE:<reason>`    | Disconnect gracefully       |
| server → client | `SAY:<from>:<text>` | Message from another user   |
| server → client | `JOIN:<name>`       | Someone joined              |
| server → client | `LEAVE:<name>`      | Someone left                |
| server → client | `SERVER:<text>`     | Server announcement         |
| server → client | `ERROR:<text>`      | Error message               |

You can speak the protocol directly with `netcat`:

```bash
nc 127.0.0.1 8080
JOIN:alice
SAY::Hello from netcat!
LEAVE:bye
```

---

## Key Rust Concepts

This project was built specifically to learn these concepts by doing:

### `TcpListener` and `TcpStream`
Raw TCP sockets from the standard library. No HTTP, no WebSockets — just bytes over a network connection.

### `thread::spawn`
Every client gets its own OS thread. The main thread only accepts new connections and immediately hands them off — so no client ever blocks another.

```rust
thread::spawn(move || {
    // this runs concurrently for every connected client
});
```

### `Arc<Mutex<T>>`
The hardest concept — sharing state safely across threads.

- `Mutex<T>` — wraps data so only one thread can access it at a time (like a lock on a door)
- `Arc<T>` — lets multiple threads hold a pointer to the same data (atomic reference counting)

```rust
let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));

// give each thread its own pointer to the same state
let state = Arc::clone(&state);

// lock before accessing, automatically unlocks when `s` goes out of scope
let mut s = state.lock().unwrap();
```

### `BufReader` + `.lines()`
Efficient line-by-line reading from a stream. Waits for a `\n` and yields one line at a time — perfect for a line-based protocol.

### `stream.try_clone()`
A `TcpStream` can't be used in two places at once. `try_clone()` creates a second handle to the same socket — one for reading, one for writing.

### RAII — automatic cleanup
Rust releases resources automatically when they go out of scope. The `Mutex` lock is released the moment `s` goes out of scope — no manual unlocking needed.

```rust
{
    let mut s = state.lock().unwrap();
    // do work
} // ← lock automatically released here
```

---

## Architecture

```
main thread
│
├── TcpListener::bind()
│
└── loop: listener.incoming()
        │
        ├── client connects
        │       │
        │       └── thread::spawn ──► handle_client thread
        │                               │
        │                               ├── register in Arc<Mutex<State>>
        │                               ├── read lines from TcpStream
        │                               ├── if JOIN: → update username
        │                               └── if SAY:  → broadcast to all peers
        │
        ├── client connects
        │       └── thread::spawn ──► handle_client thread
        │
        └── ...
```

---

## Ideas for Extending This Project

These are natural next steps — each one is a real Rust skill:

- **Rooms / channels** — `JOIN:#rust` to enter a named room, only broadcast within it
- **Private messages** — `/msg alice Hello` sends to one person only
- **Leave notifications** — broadcast `*** alice left` when a client disconnects
- **Dead peer cleanup** — remove disconnected clients from the HashMap when a write fails
- **Async rewrite** — replace threads with `tokio` and `tokio::io::split`
- **TUI client** — use `ratatui` for a split input/output terminal UI
- **Persistence** — log all messages to a file or SQLite database
- **Encryption** — wrap streams with `rustls` for TLS
- **Auth** — password-protected server or rooms

---

## What I Learned

This project was built incrementally, understanding every line before writing the next:

- How TCP connections actually work at the socket level
- Why `Arc<Mutex<T>>` exists and when you need it
- The difference between `String` and `&str`, `self` and `&self`
- How Rust prevents data races at compile time
- RAII and why Rust doesn't need a garbage collector
- How to design a simple binary wire protocol

---

## License

MIT
