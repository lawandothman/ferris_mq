use actix::Message as ActixMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: String,
    pub data: String,
    pub publish_time: String,
}

impl ActixMessage for Message {
    type Result = ();
}
