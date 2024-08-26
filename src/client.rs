use crate::publisher::Publisher;
use crate::subscriber::Subscriber;

pub struct Client;

impl Client {
    pub async fn publisher(addr: &str) -> Result<Publisher, std::io::Error> {
        Publisher::connect(addr).await
    }

    pub async fn subscriber(addr: &str) -> Result<Subscriber, std::io::Error> {
        match Subscriber::connect(addr).await {
            Err(e) => Err(e),
            Ok(mut subscriber) => {
                // We need to send a subscribe message to the server
                // so it knows we want to receive messages
                subscriber.send_subscribe_message().await?;
                Ok(subscriber)
            }
        }
    }
}
