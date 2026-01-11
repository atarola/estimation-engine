use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Room {
    pub id: String,
    pub state: RoomState,
    pub participants: HashMap<String, Client>,
    pub votes: Vec<Vote>,
}

impl Room {
    pub fn new(room_id: String) -> Self {
        Self {
            id: room_id,
            state: RoomState::Vote,
            participants: HashMap::new(),
            votes: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, uuid: String, name: String) {
        self.participants
            .insert(uuid.clone(), Client { uuid: uuid.clone(), name: name });
    }

    pub fn remove_participant(&mut self, uuid: String) {
        self.participants.remove_entry(&uuid);
        self.votes.retain(|v| v.uuid != uuid);
    }

    pub fn vote(&mut self, uuid: String, vote: VoteSizes) {
        self.votes.retain(|v| v.uuid != uuid);
        self.votes.push(Vote { uuid: uuid.clone(), vote: vote });
    }

    pub fn reveal(&mut self) {
        self.state = RoomState::Reveal;
    }

    pub fn reset(&mut self) {
        self.votes.clear();
        self.state = RoomState::Vote;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomState {
    Vote,
    Reveal,
}

#[derive(Clone, Debug, Serialize)]
pub struct Client {
    pub uuid: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
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
    Shrug,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub vote: VoteSizes,
}
