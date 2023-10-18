mod api;
mod model;
mod util;

use poem::endpoint::EmbeddedFilesEndpoint;
use poem::EndpointExt;
use poem::get;
use poem::middleware::Tracing;
use poem::post;
use poem::Route;
use poem::session::CookieConfig;
use poem::session::CookieSession;
use rust_embed::RustEmbed;
use shuttle_poem::ShuttlePoem;
use util::ping_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tokio::time::Instant;
use tokio::time::sleep_until;

use crate::api::register_handler;
use crate::api::reset_handler;
use crate::api::reveal_handler;
use crate::api::root_handler;
use crate::api::vote_handler;
use crate::api::websocket_handler;
use crate::util::topics;
use crate::model::Topic;

#[macro_use]
extern crate lazy_static;

const PING_TIME: u64 = 5000;

type Clients = Arc<RwLock<HashMap<String, Topic>>>;

lazy_static! {
    pub static ref CLIENTS: Clients = Arc::new(RwLock::new(HashMap::new()));
}

// struct to handle our static files
#[derive(RustEmbed)]
#[folder = "public"]
pub struct StaticFiles;

// main app startup
#[shuttle_runtime::main]
async fn main() -> ShuttlePoem<impl poem::Endpoint>  {
    tokio::spawn(async {
        loop {
            let keys = topics().await;
            ping_all(keys).await;
            sleep_until(Instant::now() + Duration::from_millis(PING_TIME)).await;
        }
    });

    let app = Route::new()
        .nest("/public", EmbeddedFilesEndpoint::<StaticFiles>::new())
        .at("/api/register", post(register_handler))
        .at("/api/:topic_id/reveal", post(reveal_handler))
        .at("/api/:topic_id/reset", post(reset_handler))
        .at("/api/:topic_id/vote", post(vote_handler))
        .at("/ws/:topic_id", get(websocket_handler))
        .at("/*path", get(root_handler))
        .with(CookieSession::new(CookieConfig::default().secure(false)))
        .with(Tracing);
    
    Ok(app.into())
}
