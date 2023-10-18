use futures_util::SinkExt;
use futures_util::StreamExt;
use mustache::MapBuilder;
use poem::handler;
use poem::http::StatusCode;
use poem::IntoResponse;
use poem::Response;
use poem::session::Session;
use poem::web::Json;
use poem::web::Path;
use poem::web::websocket::WebSocket;
use uuid::Uuid;

use crate::model::RegisterRequest;
use crate::model::VoteRequest;
use crate::util::close_channel;
use crate::util::open_channel;
use crate::util::publish_topic;
use crate::util::register_client;
use crate::util::render_template;
use crate::util::reset_votes;
use crate::util::reveal_votes;
use crate::util::vote;

// POST /api/register
// add ourselves to a session
#[handler]
pub async fn register_handler(session: &Session, req: Json<RegisterRequest>) -> Response {
    let topic = register_client(
            fetch_uuid(session), 
            req.name.clone(), 
            req.id.clone()
        )
        .await;

    Response::builder()
        .content_type("application/json")
        .body(topic)
}

// POST /api/:topic_id/vote
// send an event to the app
#[handler]
pub async fn vote_handler(Path(topic_id): Path<String>, req: Json<VoteRequest>, session: &Session) -> () {
    let uuid = fetch_uuid(session);
    vote(&uuid, &topic_id, req.vote.clone()).await;
    publish_topic(&topic_id).await;
}

// POST /api/:topic_id/reveal
// reveal all of the votes
#[handler]
pub async fn reveal_handler(Path(topic_id): Path<String>) -> () {
    reveal_votes(&topic_id).await;
    publish_topic(&topic_id).await;
}

// POST /api/:topic_id/reset
// reset all of the votes
#[handler]
pub async fn reset_handler(Path(topic_id): Path<String>) -> () {
    reset_votes(&topic_id).await;
    publish_topic(&topic_id).await;
}

// GET /ws/:topic_id
// event stream
#[handler]
pub async fn websocket_handler(Path(topic_id): Path<String>, session: &Session, ws: WebSocket) -> impl IntoResponse {
    let uuid = fetch_uuid(session);
    let channel = open_channel(&uuid, &topic_id).await;

    let mut rx = match channel {
        Some(item) => item,
        None => { return StatusCode::NOT_FOUND.into_response(); }
    };

    let id = topic_id.clone();

    let response = ws.on_upgrade(move |socket| async move {
            let (mut sink,  _) = socket.split();

            // sending data to the user
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if sink.send(msg).await.is_err() {
                        close_channel(&uuid, &id).await;
                        rx.close();
                        break;
                    }
                }

                publish_topic(&id).await;
            });
        })
        .into_response();

    publish_topic(&topic_id).await;
    response
}

// GET /*path
// root handler, passes route off to json to handle
#[handler]
pub fn root_handler(session: &Session) -> Response {
    let uuid = fetch_uuid(session);

    // pull the index file and return it
    let data = MapBuilder::new()
        .insert_str("uuid", uuid)
        .build();

    let template = render_template("index.html", &data);
    Response::builder().body(template)
}

// get the uuid from the session, or generate a new one if it doesn't exist
fn fetch_uuid(session: &Session) -> String {
    match session.get::<String>("uuid") {
        Some(uuid) => uuid,
        None => {
            let uuid = Uuid::new_v4();
            session.set("uuid", uuid);
            uuid.to_string()
        }
    }
}