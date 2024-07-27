use actix::prelude::*;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::error;
use uuid::Uuid;

use crate::{
    broker::{Broker, Publish},
    Message,
};

pub struct Publisher {
    broker: Addr<Broker>,
}

impl Publisher {
    pub fn new(broker: Addr<Broker>) -> Self {
        Publisher { broker }
    }

    pub async fn publish(&self, queue_name: String, data: String) -> Result<()> {
        let message_id = Uuid::new_v4().to_string();
        let publish_time = Utc::now().to_rfc3339();
        let message = Message {
            message_id,
            data,
            publish_time,
        };
        let publish = Publish {
            queue_name,
            message,
        };

        self.broker.send(publish).await.map_err(|e| {
            error!("Failed to send message to broker: {}", e);
            anyhow!(e)
        })?;
        Ok(())
    }
}
