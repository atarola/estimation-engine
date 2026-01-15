use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use poem::web::websocket::Message;
use poem::web::websocket::WebSocketStream;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc;

#[derive(Debug, Serialize)]
pub struct EventPayload {
    pub payload: Value,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ConnectionEvent {
    RoomDetails(EventPayload),
}

struct ConnectionActor {
    recv_app: mpsc::UnboundedReceiver<ConnectionEvent>,
    send_ws: SplitSink<WebSocketStream, Message>,
    recv_ws: SplitStream<WebSocketStream>,
    connected: tokio::sync::watch::Sender<bool>,
}

impl ConnectionActor {
    fn new(
        recv_app: mpsc::UnboundedReceiver<ConnectionEvent>,
        stream: WebSocketStream,
        connected: tokio::sync::watch::Sender<bool>,
    ) -> Self {
        let (send_ws, recv_ws) = stream.split();
        let _ = connected.send(true);
        Self { recv_app, send_ws, recv_ws, connected }
    }

    async fn run(mut self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

        loop {
            tokio::select! {
                app_event = self.recv_app.recv() => {
                    let Some(event) = app_event else { break; };
                    if !self.send_event(&event).await { break; }
                }

                ws_event = self.recv_ws.next() => {
                    match ws_event {
                        Some(Ok(Message::Ping(payload))) => {
                            if !self.send_ws_pong(payload).await { break; }
                        }
                        Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                        _ => {}
                    }
                }

                _ = interval.tick() => {
                    if !self.send_ws_ping().await { break; }
                }
            }
        }

        let _ = self.connected.send(false);
    }

    async fn send_event(&mut self, event: &ConnectionEvent) -> bool {
        let Ok(msg) = serde_json::to_string(event) else {
            return false;
        };

        self.send_ws.send(Message::Text(msg)).await.is_ok()
    }

    async fn send_ws_ping(&mut self) -> bool {
        self.send_ws.send(Message::Ping(vec![])).await.is_ok()
    }

    async fn send_ws_pong(&mut self, payload: Vec<u8>) -> bool {
        self.send_ws.send(Message::Pong(payload)).await.is_ok()
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    send_app: mpsc::UnboundedSender<ConnectionEvent>,
    recv_connected: tokio::sync::watch::Receiver<bool>,
}

impl ConnectionHandle {
    pub fn new(stream: WebSocketStream) -> Self {
        let (send_app, recv_app) = mpsc::unbounded_channel();
        let (send_connected, recv_connected) = tokio::sync::watch::channel(false);

        let actor = ConnectionActor::new(recv_app, stream, send_connected);
        tokio::spawn(async move { actor.run().await });

        Self { send_app, recv_connected }
    }

    pub fn send_room_details(&self, payload: Value) {
        let _ = self.send_app.send(ConnectionEvent::RoomDetails(EventPayload { payload }));
    }

    pub fn connection_watch(&self) -> tokio::sync::watch::Receiver<bool> {
        self.recv_connected.clone()
    }
}
