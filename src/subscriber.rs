use std::io;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub const SUBSCRIBE_BYTE: u8 = 1;

pub struct Subscriber {
    stream: TcpStream,
}

pub enum SubscribeError {
    Connect(io::Error),
    Write(io::Error),
    Read(io::Error),
    InvalidMessage,
}

impl Subscriber {
    pub async fn connect(addr: &str) -> Result<Self, SubscribeError> {
        TcpStream::connect(addr)
            .await
            .map(Self::new)
            .map_err(SubscribeError::Connect)
    }

    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn send_subscribe_message(&mut self) -> Result<usize, SubscribeError> {
        self.stream
            .write(&[SUBSCRIBE_BYTE])
            .await
            .map_err(SubscribeError::Write)
    }

    pub async fn listen(&mut self) -> Result<Vec<u8>, SubscribeError> {
        let mut buffer = [0; 1024];

        self.stream
            .read(&mut buffer)
            .await
            .map(|n| buffer[..n].to_vec())
            .map_err(SubscribeError::Read)
    }
}
