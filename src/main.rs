mod actors;
mod routes;
mod state;
mod util;

use poem::endpoint::EmbeddedFilesEndpoint;
use poem::get;
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::post;
use poem::session::CookieConfig;
use poem::session::CookieSession;
use poem::EndpointExt;
use poem::Route;
use poem::Server;
use rust_embed::RustEmbed;
use tracing_subscriber::EnvFilter;

use crate::routes::handlers::register_handler;
use crate::routes::handlers::reset_handler;
use crate::routes::handlers::reveal_handler;
use crate::routes::handlers::root_handler;
use crate::routes::handlers::vote_handler;
use crate::routes::handlers::websocket_handler;
use crate::state::AppState;

// struct to handle our static files
#[derive(RustEmbed)]
#[folder = "public"]
pub struct StaticFiles;

// main app startup
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let state = AppState::new();

    let app = Route::new()
        .nest("/public", EmbeddedFilesEndpoint::<StaticFiles>::new())
        .at("/api/register", post(register_handler))
        .at("/api/:room_id/reveal", post(reveal_handler))
        .at("/api/:room_id/reset", post(reset_handler))
        .at("/api/:room_id/vote", post(vote_handler))
        .at("/ws/:room_id", get(websocket_handler))
        .at("/*path", get(root_handler))
        .with(CookieSession::new(CookieConfig::default().secure(false)))
        .with(Tracing)
        .data(state);

    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}
