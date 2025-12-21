# Multithreaded Web Server in Rust

A simple, educational HTTP server implementation in Rust that demonstrates concurrent request handling using a custom thread pool. This project is based on the final project from the Rust Programming Language book.

## Features

- **TCP Connection Handling**: Listens for incoming TCP connections on `127.0.0.1:7878`
- **HTTP Request Parsing**: Parses basic HTTP/1.1 GET requests
- **Multithreaded Architecture**: Custom thread pool implementation for concurrent request handling
- **Graceful Shutdown**: Implements proper cleanup and shutdown procedures
- **Multiple Routes**:
  - `/` - Returns a welcome HTML page
  - `/sleep` - Simulates slow request processing (5-second delay)
  - All other routes return a 404 error page

## Project Structure

```
hello/
├── src/
│   ├── main.rs          # HTTP server implementation
│   └── lib.rs           # ThreadPool and Worker implementations
├── hello.html           # Main page HTML
├── 404.html             # Error page HTML
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Architecture

### ThreadPool

The `ThreadPool` manages a fixed number of worker threads that process incoming connections concurrently:

- **Channel-based job distribution**: Uses `mpsc::channel()` for job queuing
- **Arc<Mutex<>>**: Enables safe sharing of the receiver across multiple threads
- **Worker pattern**: Each worker runs in its own thread, waiting for jobs

### Request Flow

1. TCP listener accepts incoming connections
2. Connection is passed to the thread pool via `execute()`
3. An available worker picks up the job from the channel
4. Worker processes the HTTP request and sends response
5. Worker returns to the pool, ready for the next job

## Building and Running

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build

```bash
cd hello
cargo build
```

### Run

```bash
cargo run
```

The server will start on `http://127.0.0.1:7878`

### Test the Server

Open your web browser and visit:

- `http://127.0.0.1:7878/` - Main page
- `http://127.0.0.1:7878/sleep` - Slow request (5-second delay)
- `http://127.0.0.1:7878/anything` - 404 error page

**Note**: The current implementation shuts down after serving 2 requests to demonstrate graceful shutdown. Remove `.take(2)` in `main.rs` for continuous operation.

## Key Concepts Demonstrated

### 1. TCP and HTTP Protocols
- Raw TCP socket handling
- HTTP request/response format
- Status codes (200 OK, 404 NOT FOUND)

### 2. Concurrency
- Thread spawning and management
- Channel-based communication (`mpsc`)
- Synchronization primitives (`Arc`, `Mutex`)

### 3. Ownership and Lifetimes
- Moving closures into threads
- Trait bounds (`FnOnce`, `Send`, `'static`)

### 4. Resource Management
- Custom `Drop` implementation for cleanup
- Graceful shutdown pattern

## Configuration

### Change Port

Edit `main.rs`:
```rust
let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
```

### Change Thread Pool Size

Edit `main.rs`:
```rust
let pool = ThreadPool::new(8); // Increase to 8 workers
```

### Remove Request Limit

Edit `main.rs`:
```rust
for stream in listener.incoming() { // Remove .take(2)
    // ...
}
```

## Limitations

This is an educational project and **not production-ready**:

- No HTTPS/TLS support
- Limited HTTP method support (GET only)
- No query parameter parsing
- Basic error handling (uses `unwrap()`)
- No request routing framework
- No static file serving
- No logging framework

## Extension Ideas

See `EXTENSION_GUIDE.md` for detailed implementation suggestions including:

- Enhanced documentation and testing
- Robust error handling
- Additional use cases beyond web serving
- Integration with production-ready thread pool crates

## Learning Resources

- [The Rust Programming Language Book - Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [std::net module documentation](https://doc.rust-lang.org/std/net/)
- [std::sync module documentation](https://doc.rust-lang.org/std/sync/)

## License

This is an educational project based on examples from The Rust Programming Language book.

## Acknowledgments

Based on the final project from "The Rust Programming Language" book by Steve Klabnik and Carol Nichols.
