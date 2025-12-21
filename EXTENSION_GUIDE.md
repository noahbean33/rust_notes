# Extension Guide for Multithreaded Web Server

This guide provides detailed implementation suggestions for extending the basic multithreaded web server. These ideas are based on the suggestions from the Rust Book's final project chapter.

## Table of Contents

1. [Enhanced Documentation](#1-enhanced-documentation)
2. [Testing Infrastructure](#2-testing-infrastructure)
3. [Robust Error Handling](#3-robust-error-handling)
4. [Alternative Use Cases](#4-alternative-use-cases)
5. [Using Production Thread Pool Crates](#5-using-production-thread-pool-crates)
6. [Advanced Features](#6-advanced-features)

---

## 1. Enhanced Documentation

### Current State
The ThreadPool has basic doc comments, but they could be more comprehensive.

### Implementation Steps

#### Add Module-Level Documentation

Create detailed module documentation in `src/lib.rs`:

```rust
//! # Thread Pool Library
//!
//! A simple thread pool implementation for executing closures concurrently.
//!
//! ## Features
//!
//! - Fixed-size thread pool
//! - Channel-based job distribution
//! - Graceful shutdown via Drop trait
//!
//! ## Example
//!
//! ```rust
//! use hello::ThreadPool;
//!
//! let pool = ThreadPool::new(4);
//! 
//! for i in 0..8 {
//!     pool.execute(move || {
//!         println!("Job {}", i);
//!     });
//! }
//! ```
```

#### Enhance Method Documentation

Add examples, edge cases, and thread safety notes:

```rust
impl ThreadPool {
    /// Creates a new ThreadPool with the specified number of workers.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads to spawn
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// ```
    ///
    /// # Thread Safety
    ///
    /// All workers share a single receiver wrapped in Arc<Mutex<_>>,
    /// ensuring thread-safe job distribution.
    pub fn new(size: usize) -> ThreadPool {
        // implementation
    }

    /// Executes a closure on an available worker thread.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that implements FnOnce() + Send + 'static
    ///
    /// # Panics
    ///
    /// Panics if the channel's receiver has been dropped (shouldn't
    /// happen in normal operation).
    ///
    /// # Examples
    ///
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// pool.execute(|| {
    ///     println!("Hello from worker thread!");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // implementation
    }
}
```

#### Generate and Review Documentation

```bash
cargo doc --open
```

---

## 2. Testing Infrastructure

### Unit Tests for ThreadPool

Add tests in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_threadpool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_threadpool_zero_size_panics() {
        let _pool = ThreadPool::new(0);
    }

    #[test]
    fn test_execute_single_job() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });

        thread::sleep(Duration::from_millis(100));
        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn test_execute_multiple_jobs() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
        }

        thread::sleep(Duration::from_millis(500));
        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_graceful_shutdown() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(50));
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
        }

        drop(pool); // Should wait for all jobs to complete
        assert_eq!(*counter.lock().unwrap(), 5);
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use hello::ThreadPool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_concurrent_execution() {
    let pool = ThreadPool::new(4);
    let results = Arc::new(Mutex::new(Vec::new()));

    for i in 0..20 {
        let results_clone = Arc::clone(&results);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(10));
            results_clone.lock().unwrap().push(i);
        });
    }

    drop(pool);
    assert_eq!(results.lock().unwrap().len(), 20);
}
```

### HTTP Server Tests

Create `tests/http_server_test.rs`:

```rust
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[test]
fn test_http_root_endpoint() {
    // Start server in background thread
    thread::spawn(|| {
        // Your main function here or extracted server logic
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();

    assert!(buffer.contains("200 OK"));
    assert!(buffer.contains("Hello!"));
}
```

---

## 3. Robust Error Handling

### Replace unwrap() with Proper Error Handling

#### Create Custom Error Types

Add to `src/lib.rs`:

```rust
use std::fmt;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
    SystemResourceError(String),
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolCreationError::ZeroSize => {
                write!(f, "Thread pool size must be greater than 0")
            }
            PoolCreationError::SystemResourceError(msg) => {
                write!(f, "Failed to create thread pool: {}", msg)
            }
        }
    }
}

impl std::error::Error for PoolCreationError {}
```

#### Implement build() Method

```rust
impl ThreadPool {
    /// Creates a new ThreadPool, returning Result for better error handling.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads
    ///
    /// # Errors
    ///
    /// Returns `PoolCreationError::ZeroSize` if size is 0.
    /// Returns `PoolCreationError::SystemResourceError` if thread creation fails.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            match Worker::build(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(e) => {
                    return Err(PoolCreationError::SystemResourceError(
                        format!("Failed to create worker {}: {}", id, e)
                    ));
                }
            }
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
}
```

#### Update Worker Creation

```rust
use std::io;

impl Worker {
    fn build(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, io::Error> {
        let builder = thread::Builder::new().name(format!("worker-{}", id));

        let thread = builder.spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        })?;

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}
```

#### Update Main with Error Handling

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .map_err(|e| format!("Failed to bind to address: {}", e))?;
    
    let pool = ThreadPool::build(4)?;

    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    if let Err(e) = handle_connection_safe(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    println!("Shutting down.");
    Ok(())
}

fn handle_connection_safe(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or("No request line")?
        .map_err(|e| format!("Failed to read request: {}", e))?;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read {}: {}", filename, e))?;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes())
        .map_err(|e| format!("Failed to write response: {}", e))?;

    Ok(())
}
```

---

## 4. Alternative Use Cases

### Image Processing Pipeline

```rust
use hello::ThreadPool;
use std::path::PathBuf;

fn main() {
    let pool = ThreadPool::new(4);
    let images: Vec<PathBuf> = get_image_files();

    for image_path in images {
        pool.execute(move || {
            process_image(&image_path);
        });
    }
}

fn process_image(path: &PathBuf) {
    println!("Processing image: {:?}", path);
    // Resize, apply filters, etc.
}

fn get_image_files() -> Vec<PathBuf> {
    // Read from directory
    vec![]
}
```

### Parallel Data Processing

```rust
use hello::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    let pool = ThreadPool::new(8);
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = Arc::new(Mutex::new(Vec::new()));

    for chunk in data.chunks(2) {
        let chunk = chunk.to_vec();
        let results_clone = Arc::clone(&results);
        
        pool.execute(move || {
            let sum: i32 = chunk.iter().sum();
            results_clone.lock().unwrap().push(sum);
        });
    }

    drop(pool); // Wait for completion
    println!("Results: {:?}", results.lock().unwrap());
}
```

### File System Operations

```rust
use hello::ThreadPool;
use std::fs;

fn main() {
    let pool = ThreadPool::new(4);
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];

    for file in files {
        pool.execute(move || {
            if let Ok(content) = fs::read_to_string(file) {
                let word_count = content.split_whitespace().count();
                println!("{}: {} words", file, word_count);
            }
        });
    }
}
```

---

## 5. Using Production Thread Pool Crates

### Option 1: threadpool Crate

Add to `Cargo.toml`:
```toml
[dependencies]
threadpool = "1.8"
```

Update `src/main.rs`:
```rust
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let pool = ThreadPool::new(4);
    let (tx, rx) = channel();

    for i in 0..8 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(i).unwrap();
        });
    }

    drop(tx);
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Option 2: rayon Crate

Add to `Cargo.toml`:
```toml
[dependencies]
rayon = "1.8"
```

Update with parallel iterators:
```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (0..100).collect();
    
    let sum: i32 = numbers.par_iter().sum();
    println!("Sum: {}", sum);
}
```

### Option 3: tokio Runtime

For async/await patterns:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                println!("Task {}", i);
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }
}
```

### Comparison

| Feature | Custom Pool | threadpool | rayon | tokio |
|---------|-------------|-----------|-------|-------|
| Complexity | Simple | Simple | Simple | Complex |
| Control | Full | Medium | Low | High |
| Performance | Good | Good | Excellent | Excellent |
| Async Support | No | No | No | Yes |
| Best For | Learning | General threading | Data parallelism | Async I/O |

---

## 6. Advanced Features

### Request Logging

```rust
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

fn log_request(request_line: &str, status: u16) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {} - {}\n", timestamp, request_line, status);
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("server.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}
```

### Configuration File Support

Create `config.toml`:
```toml
[server]
host = "127.0.0.1"
port = 7878
workers = 4

[paths]
html_dir = "public"
log_file = "server.log"
```

Parse with `serde`:
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ServerConfig {
    server: Server,
    paths: Paths,
}

#[derive(Deserialize)]
struct Server {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Deserialize)]
struct Paths {
    html_dir: String,
    log_file: String,
}
```

### Metrics and Monitoring

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct Metrics {
    requests_served: AtomicUsize,
    active_connections: AtomicUsize,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            requests_served: AtomicUsize::new(0),
            active_connections: AtomicUsize::new(0),
        }
    }

    pub fn increment_requests(&self) {
        self.requests_served.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (
            self.requests_served.load(Ordering::SeqCst),
            self.active_connections.load(Ordering::SeqCst),
        )
    }
}
```

---

## Next Steps

1. **Start Small**: Pick one extension that interests you
2. **Test Thoroughly**: Add tests before implementing features
3. **Iterate**: Make small changes and validate
4. **Document**: Keep documentation updated
5. **Benchmark**: Measure performance improvements

## Resources

- [Rust Book Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [threadpool crate](https://docs.rs/threadpool/)
- [rayon crate](https://docs.rs/rayon/)
- [tokio crate](https://tokio.rs/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
