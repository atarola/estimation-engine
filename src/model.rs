use poem::web::websocket::Message;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;


#[derive(Debug, Serialize)]
pub struct Topic {
    pub id: String,
    pub state: TopicState,
    pub participants: HashMap<String, Client>,
    pub votes: Vec<Vote>
}

impl Topic {
    // add a client to the topic
    pub fn add_client(&mut self, uuid: String, name: String) {
        self.participants.insert(
            uuid.clone(),
            Client {
                uuid: uuid.clone(),
                name: name.clone(),
                sender: None
            }
        );
    }

    // return the json representation of this topic
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    // create a new topic
    pub fn new(id: String) -> Topic{
        Topic {
            id: id.clone(),
            state: TopicState::Vote,
            votes: Vec::new(),
            participants: HashMap::new()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Client {
    pub uuid: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub sender: Option<UnboundedSender<Message>>,
}

#[derive(Debug, Serialize)]
pub struct Vote {
    pub uuid: String,
    pub vote: VoteSizes,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum VoteSizes {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    Coffee,
    Shrug
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TopicState {
    Vote,
    Reveal
}

//
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub id: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub vote: VoteSizes
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Status(StatusEvent),
    Ping(PingEvent)
}

#[derive(Debug, Serialize)]
pub struct StatusEvent {
    pub topic_id: String,
    pub payload: Value,
}

#[derive(Debug, Serialize)]
pub struct PingEvent {
    pub topic_id: String
}