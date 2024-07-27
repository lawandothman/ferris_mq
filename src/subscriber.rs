use actix::prelude::*;
use anyhow::{anyhow, Result};
use log::{error, info};

use crate::{broker::Subscribe, Broker, Message};

#[derive(Clone)]
pub struct Subscriber {
    broker: Addr<Broker>,
}

impl Subscriber {
    pub fn new(broker: Addr<Broker>) -> Self {
        Subscriber { broker }
    }

    pub async fn subscribe(&self, queue_name: String) -> Result<()> {
        let addr = self.clone().start();
        self.broker
            .send(Subscribe {
                queue_name,
                subscriber: addr.recipient(),
            })
            .await
            .map_err(|e| {
                error!("Failed to subscribe to queue: {}", e);
                anyhow!(e)
            })?;
        Ok(())
    }
}

impl Actor for Subscriber {
    type Context = Context<Self>;
}

impl Handler<Message> for Subscriber {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        info!("Received message: {:?}", msg);
    }
}
