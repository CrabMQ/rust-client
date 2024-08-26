use crate::publisher::{PublishError, Publisher};
use crate::subscriber::{SubscribeError, Subscriber};

pub struct QueueClient;

impl QueueClient {
    pub async fn publisher(addr: &str) -> Result<Publisher, PublishError> {
        Publisher::connect(addr).await
    }

    pub async fn subscriber(addr: &str) -> Result<Subscriber, SubscribeError> {
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
