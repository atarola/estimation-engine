use mustache::MapBuilder;
use poem::handler;
use poem::session::Session;
use poem::web::websocket::WebSocket;
use poem::web::Data;
use poem::web::Json;
use poem::web::Path;
use poem::IntoResponse;
use poem::Response;
use uuid::Uuid;

use crate::routes::types::RegisterRequest;
use crate::routes::types::VoteRequest;
use crate::state::AppState;
use crate::util::generate_id;
use crate::util::render_template;

/// POST /api/register
/// add ourselves to a session
#[handler]
pub async fn register_handler(
    session: &Session,
    state: Data<&AppState>,
    req: Json<RegisterRequest>,
) -> Response {
    let id = req.id.clone();
    let session_id = fetch_uuid(session);
    let name = req.name.clone();

    let room_id = match id {
        Some(room_id) => room_id,
        None => generate_id(8),
    };

    let room_handle = state.room_registry.get_or_create(room_id);
    let room = room_handle.register(session_id, name).await;

    let body = serde_json::to_value(room).unwrap();
    Response::builder().content_type("application/json").body(body.to_string())
}

/// GET /ws/:room_id
/// event stream
#[handler]
pub async fn websocket_handler(
    Path(room_id): Path<String>,
    session: &Session,
    state: Data<&AppState>,
    ws: WebSocket,
) -> impl IntoResponse {
    let session_id = fetch_uuid(session);
    let room = state.room_registry.get_or_create(room_id);

    ws.on_upgrade(move |socket| async move {
        room.add_websocket(session_id, socket).await;
    })
}

/// POST /api/:room_id/vote
/// send an event to the app
#[handler]
pub async fn vote_handler(
    session: &Session,
    state: Data<&AppState>,
    Path(room_id): Path<String>,
    req: Json<VoteRequest>,
) -> () {
    let session_id = fetch_uuid(session);
    let room = state.room_registry.get_or_create(room_id);
    room.vote(session_id, req.vote).await;
}

/// POST /api/:room_id/reveal
/// reveal all of the votes
#[handler]
pub async fn reveal_handler(state: Data<&AppState>, Path(room_id): Path<String>) -> () {
    let room = state.room_registry.get_or_create(room_id);
    room.reveal().await;
}

/// POST /api/:room_id/reset
/// reset all of the votes
#[handler]
pub async fn reset_handler(state: Data<&AppState>, Path(room_id): Path<String>) -> () {
    let room = state.room_registry.get_or_create(room_id);
    room.reset().await;
}

/// GET /*path
/// root handler, passes route off to javascript to handle
#[handler]
pub fn root_handler(session: &Session) -> Response {
    let uuid = fetch_uuid(session);
    let data = MapBuilder::new().insert_str("uuid", uuid).build();
    let template = render_template("index.html", &data);
    Response::builder().body(template)
}

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
