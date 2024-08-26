# 🦀 CrabMQ Rust Client Library

This library provides a simple and efficient client for interacting with the [CrabMQ](https://github.com/CrabMQ/crab-mq) queue server. With this library, you can easily publish and subscribe to messages over TCP.

## ✨ Features

- 📤 **Publish messages:** Send messages to the queue server.
- 📥 **Subscribe to messages:** Receive messages from the queue server.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
crabmq = "1.0.0"
```

## 🚀 Usage

### 🔌 Connecting to the Queue Server

You can connect to the CrabMQ server as a publisher or a subscriber.

#### 📤 Publisher

```rust
use crabmq::Client;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut publisher = Client::publisher("127.0.0.1:8080").await?;

    publisher.publish(b"Hello, world!").await?;

    Ok(())
}
```

#### 📥 Subscriber

```rust
use crabmq::Client;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut subscriber = Client::subscriber("127.0.0.1:8080").await?;

    loop {
        let message = subscriber.listen().await?;
        println!("Received: {:?}", message);
    }
}
```

## 🤝 Contributing

Feel free to open issues or submit pull requests if you find bugs or want to contribute improvements.
