use futures::stream::SplitSink;
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
    input: mpsc::UnboundedReceiver<ConnectionEvent>,
    sink: SplitSink<WebSocketStream, Message>,
    connected: tokio::sync::watch::Sender<bool>,
}

impl ConnectionActor {
    fn new(
        input: mpsc::UnboundedReceiver<ConnectionEvent>,
        stream: WebSocketStream,
        connected: tokio::sync::watch::Sender<bool>,
    ) -> Self {
        let (sink, _) = stream.split();
        let _ = connected.send(true);

        Self { input, sink, connected }
    }

    async fn run(mut self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            tokio::select! {
                maybe_event = self.input.recv() => {
                    let Some(event) = maybe_event else { break; };
                    if !self.send_event(&event).await { break; }
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

        self.sink.send(Message::Text(msg)).await.is_ok()
    }

    async fn send_ws_ping(&mut self) -> bool {
        self.sink.send(Message::Ping(vec![])).await.is_ok()
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    send: mpsc::UnboundedSender<ConnectionEvent>,
    connected: tokio::sync::watch::Receiver<bool>,
}

impl ConnectionHandle {
    pub fn new(stream: WebSocketStream) -> Self {
        let (send, recv) = mpsc::unbounded_channel();
        let (connected_tx, connected_rx) = tokio::sync::watch::channel(false);

        let actor = ConnectionActor::new(recv, stream, connected_tx);
        tokio::spawn(async move { actor.run().await });

        Self { send, connected: connected_rx }
    }

    pub fn send_room_details(&self, payload: Value) {
        let _ = self.send.send(ConnectionEvent::RoomDetails(EventPayload { payload }));
    }

    pub fn is_connected(&self) -> bool {
        *self.connected.borrow()
    }
}
