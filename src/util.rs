use mustache;
use mustache::Data;
use poem::web::websocket::Message;
use rand::Rng;
use std::iter;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::CLIENTS;
use crate::model::Event;
use crate::model::PingEvent;
use crate::model::StatusEvent;
use crate::model::Topic;
use crate::model::TopicState;
use crate::model::Vote;
use crate::model::VoteSizes;
use crate::StaticFiles;

// render a template from the static files using moustache
pub fn render_template(name: &str, data: &Data) -> String {
    let index = StaticFiles::get(name).unwrap();
    let template_string = std::str::from_utf8(index.data.as_ref()).unwrap();
    let template = mustache::compile_str(template_string).unwrap();
    template.render_data_to_string(&data).unwrap()
}

// get a list of topics
pub async fn topics() -> Vec<String> {
   let clients = CLIENTS.read().await;
    clients.keys().cloned().collect()
}

// ping all clients on all topics
pub async fn ping_all(topics: Vec<String>) {
    for topic_id in topics {
        let event = Event::Ping(PingEvent {
            topic_id: topic_id.clone()
        });

        publish(&topic_id, &event).await;
    } 
}

// setup a client and a topic
pub async fn register_client(uuid: String, name: String, id: Option<String>) -> String {
    // generate a topic id if one isn't passed in
    let topic_id = match id {
        Some(topic_id) => topic_id,
        None => { generate_id(8) }
    };

    let mut clients = CLIENTS.write().await;

    // find or create a topic by the id
    let mut topic = match clients.remove(&topic_id) {
        Some(topic) => topic,
        None => Topic::new(topic_id.clone())
    };

    // add ourselves to the client
    topic.add_client(uuid, name);

    // stash the json of the topic before we give it away
    let json = topic.to_json();

    clients.insert(topic_id, topic);

    json.to_string()
}

// open a channel for the client on this topic
pub async fn open_channel(uuid: &String, id: &String) -> Option<UnboundedReceiver<Message>> {
    let (tx, rx) = unbounded_channel::<Message>();

    let mut clients = CLIENTS.write().await;
    
    let topic = match clients.get_mut(id) {
        Some(topic) => topic,
        None => return None
    };

    match topic.participants.get_mut(uuid) {
        Some(client) => {
            client.sender = Some(tx);
            Some(rx)
        },
        None => None
    }
}

// 
pub async fn close_channel(uuid: &String, id: &String) {
    let mut clients = CLIENTS.write().await;

    let topic = match clients.get_mut(id) {
        Some(topic) => topic,
        None => return
    };

    // clear any votes for this uuid
    topic.votes.retain(|v| !v.uuid.eq(uuid));
    topic.participants.remove(uuid);
}

// 
pub async fn vote(uuid: &String, id: &String, vote: VoteSizes) {
    let mut clients = CLIENTS.write().await;

    let topic = match clients.get_mut(id) {
        Some(topic) => topic,
        None => return
    };

    // remove any old votes for this uuid
    topic.votes.retain(|v| !v.uuid.eq(uuid));

    // add the vote to the votes
    topic.votes.push(Vote { uuid: uuid.clone(), vote });

    // if everyone has voted, reveal the votes
    if topic.votes.len() == topic.participants.len() {
        topic.state = TopicState::Reveal;
    }
}

//
pub async fn reveal_votes(id: &String) {
    let mut clients = CLIENTS.write().await;

    let topic = match clients.get_mut(id) {
        Some(topic) => topic,
        None => return
    };

    topic.state = TopicState::Reveal;
}

//
pub async fn reset_votes(id: &String) {
    let mut clients = CLIENTS.write().await;

    let topic = match clients.get_mut(id) {
        Some(topic) => topic,
        None => return
    };

    topic.state = TopicState::Vote;
    topic.votes.clear();
}

//
pub async fn publish_topic(id: &String) {
    let clients = CLIENTS.read().await;

    let topic = match clients.get(id) {
        Some(topic) => topic,
        None => return
    };

    let event = Event::Status(StatusEvent {
        topic_id: id.to_string(),
        payload: topic.to_json()
    });

    publish(id, &event).await;
}

// send a message to all clients on the topic
pub async fn publish(id: &String, event: &Event) {
    let clients = CLIENTS.read().await;

    let topic = match clients.get(id) {
        Some(topic) => topic,
        None => return
    };

    topic.participants
        .iter()
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
              let msg = serde_json::to_string(&event).unwrap();
              sender.send(Message::Text(msg)).unwrap();
            }
        });
}

// generate a random id
pub fn generate_id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}