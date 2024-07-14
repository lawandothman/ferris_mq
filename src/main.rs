use actix::prelude::*;
use anyhow::Result;
use ferris_mq::{Broker, Publisher, Subscriber};

#[actix::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🚀 Starting the ferris_mq server");

    let broker = Broker::new().start();

    let publisher = Publisher::new(broker.clone());
    let subscriber = Subscriber::new(broker.clone());

    subscriber.subscribe("queue_1".to_string()).await?;

    publisher
        .publish("queue_1".to_string(), "Hello, World!".to_string())
        .await?;

    Ok(())
}
