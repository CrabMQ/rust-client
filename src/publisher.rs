use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub const PUBLISH_BYTE: u8 = 0;

pub struct Publisher {
    stream: TcpStream,
}

impl Publisher {
    pub async fn connect(addr: &str) -> Result<Self, std::io::Error> {
        TcpStream::connect(addr).await.map(Self::new)
    }

    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn publish(&mut self, message: &[u8]) -> Result<(), std::io::Error> {
        let mut buf: Vec<u8> = Vec::with_capacity(message.len() + 1);

        buf.push(PUBLISH_BYTE);
        buf.extend_from_slice(message);

        self.stream.write_all(&buf).await?;
        Ok(())
    }
}
