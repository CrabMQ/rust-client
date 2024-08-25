# 🦀 CrabMQ Rust Client Library

This library provides a simple and efficient client for interacting with the [CrabMQ](https://github.com/CrabMQ/crab-mq) queue server. With this library, you can easily publish and subscribe to messages over TCP.

## ✨ Features

- 📤 **Publish messages:** Send messages to the queue server.
- 📥 **Subscribe to messages:** Receive messages from the queue server.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
crabmq_client = "1.0.0"
```

## 🚀 Usage

### 🔌 Connecting to the Queue Server

You can connect to the CrabMQ server as a publisher or a subscriber.

#### 📤 Publisher

```rust
use crabmq_client::{QueueClient, Publisher};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut publisher = QueueClient::connect("127.0.0.1:8080").await?;
    
    let message = b"Hello, CrabMQ!";
    publisher.publish(message).await?;

    Ok(())
}
```

#### 📥 Subscriber

```rust
use crabmq_client::{QueueClient, Subscriber};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut subscriber = QueueClient::subscribe("127.0.0.1:8080").await?;

    loop {
        let message = subscriber.listen().await?;
        println!("Received: {:?}", message);
    }
}
```

## 🤝 Contributing

Feel free to open issues or submit pull requests if you find bugs or want to contribute improvements.
