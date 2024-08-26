use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub const SUBSCRIBE_BYTE: u8 = 1;

pub struct Subscriber {
    stream: TcpStream,
}

impl Subscriber {
    pub async fn connect(addr: &str) -> Result<Self, std::io::Error> {
        TcpStream::connect(addr).await.map(Self::new)
    }

    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn send_subscribe_message(&mut self) -> Result<usize, std::io::Error> {
        self.stream.write(&[SUBSCRIBE_BYTE]).await
    }

    pub async fn listen(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = [0; 1024];

        self.stream
            .read(&mut buffer)
            .await
            .map(|n| buffer[..n].to_vec())
    }
}
