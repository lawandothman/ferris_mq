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
