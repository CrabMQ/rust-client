use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const PUBLISH_BYTE: &[u8] = &[0];
const SUBSCRIBE_BYTE: &[u8] = &[1];

pub struct QueueClient;

pub struct Publisher {
    stream: TcpStream,
}

pub struct Subscriber {
    stream: TcpStream,
}

impl Publisher {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn publish(&mut self, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.write(PUBLISH_BYTE).await?;
        self.stream.write(&message).await?;
        Ok(())
    }
}

impl Subscriber {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

impl Subscriber {
    pub async fn listen(&mut self) -> Option<Vec<u8>> {
        let mut buffer = vec![0; 1024];

        let n = self.stream.read(&mut buffer).await.unwrap();

        println!("Total Of Bytes Received: {}", n);

        if n == 0 {
            return None;
        }

        Some(buffer[..n].to_vec())
    }
}

impl QueueClient {
    pub async fn connect(addr: &str) -> Result<Publisher, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Publisher::new(stream))
    }

    pub async fn subscribe(addr: &str) -> Result<Subscriber, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(addr).await?;
        stream.write(SUBSCRIBE_BYTE).await?;
        Ok(Subscriber::new(stream))
    }
}
