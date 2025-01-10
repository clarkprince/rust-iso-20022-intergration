use async_trait::async_trait;
use serde_json::Value;

use crate::error::MessagingError;

#[async_trait]
pub trait MessagePublisher: Send + Sync {
    async fn publish_message(&self, routing_key: &str, message: Value) -> Result<(), MessagingError>;
}

pub struct RabbitMQPublisher {
    connection: lapin::Connection,
    channel: lapin::Channel,
}
