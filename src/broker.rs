use crate::message::Message;
use actix::prelude::*;
use std::collections::HashMap;

pub struct Broker {
    subscribers: HashMap<String, Vec<Recipient<Message>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            subscribers: HashMap::new(),
        }
    }
}

impl Actor for Broker {
    type Context = Context<Self>;
}

pub struct Publish {
    pub queue_name: String,
    pub message: Message,
}

impl actix::Message for Publish {
    type Result = ();
}

impl Handler<Publish> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) {
        if let Some(subscribers) = self.subscribers.get_mut(&msg.queue_name) {
            for subscriber in subscribers.iter_mut() {
                let _ = subscriber.do_send(msg.message.clone());
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub queue_name: String,
    pub subscriber: Recipient<Message>,
}

impl Handler<Subscribe> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        self.subscribers
            .entry(msg.queue_name)
            .or_insert_with(Vec::new)
            .push(msg.subscriber);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Message;
    use tokio::sync::mpsc;

    struct MockSubscriber {
        tx: mpsc::Sender<Message>,
    }

    impl Actor for MockSubscriber {
        type Context = Context<Self>;
    }

    impl Handler<Message> for MockSubscriber {
        type Result = ();

        fn handle(&mut self, msg: Message, _: &mut Self::Context) {
            let _ = self.tx.try_send(msg);
        }
    }

    #[actix::test]
    async fn test_broker_subscription_and_publish() {
        let broker = Broker::new().start();

        let (tx, mut rx) = mpsc::channel(1);
        let mock_subscriber = MockSubscriber { tx }.start();

        broker
            .send(Subscribe {
                queue_name: "test_queue".to_string(),
                subscriber: mock_subscriber.recipient(),
            })
            .await
            .unwrap();

        let test_message = Message {
            message_id: "1".to_string(),
            data: "Avada Kedavra".to_string(),
            publish_time: "2021-07-01T00:00:00Z".to_string(),
        };

        broker
            .send(Publish {
                message: test_message.clone(),
                queue_name: "test_queue".to_string(),
            })
            .await
            .unwrap();

        let received_message = rx.recv().await.expect("Expected a message to be received");
        assert_eq!(received_message.data, "Avada Kedavra")
    }
}
