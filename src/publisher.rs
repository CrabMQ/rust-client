use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub const PUBLISH_BYTE: u8 = 0;

pub struct Publisher {
    stream: TcpStream,
}

pub enum PublishError {
    Connect(std::io::Error),
    Write(std::io::Error),
}

impl Publisher {
    pub async fn connect(addr: &str) -> Result<Self, PublishError> {
        TcpStream::connect(addr)
            .await
            .map(Self::new)
            .map_err(PublishError::Connect)
    }

    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn publish(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut buf: Vec<u8> = Vec::with_capacity(message.len() + 1);

        buf.push(PUBLISH_BYTE);
        buf.extend_from_slice(message);

        self.stream.write_all(&buf).await?;
        Ok(())
    }
}
