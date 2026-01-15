use std::collections::HashMap;
use std::fmt;

use poem::web::websocket::WebSocketStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_stream::adapters::Filter;
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt;
use tokio_stream::StreamMap;

use crate::actors::connection::ConnectionHandle;
use crate::routes::types::Room;
use crate::routes::types::VoteSizes;

#[derive(Debug)]
struct VoteEvent {
    respond_to: oneshot::Sender<()>,
    uuid: String,
    vote: VoteSizes,
}

#[derive(Debug)]
struct RegisterEvent {
    respond_to: oneshot::Sender<Room>,
    uuid: String,
    name: String,
}

#[derive(Debug)]
struct RevealEvent {
    respond_to: oneshot::Sender<()>,
}

#[derive(Debug)]
struct ResetEvent {
    respond_to: oneshot::Sender<()>,
}

struct AddWebsocketEvent {
    respond_to: oneshot::Sender<()>,
    uuid: String,
    websocket: WebSocketStream,
}

impl fmt::Debug for AddWebsocketEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddWebsocketEvent")
            .field("uuid", &self.uuid)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
enum RoomEvent {
    Register(RegisterEvent),
    AddWebsocket(AddWebsocketEvent),
    Vote(VoteEvent),
    Reveal(RevealEvent),
    Reset(ResetEvent),
}

type DisconnectStream = Filter<WatchStream<bool>, fn(&bool) -> bool>;

struct RoomActor {
    recv: mpsc::Receiver<RoomEvent>,
    connections: HashMap<String, ConnectionHandle>,
    watch_streams: StreamMap<String, DisconnectStream>,
    room: Room,
}

impl RoomActor {
    fn new(room_id: String, recv: mpsc::Receiver<RoomEvent>) -> Self {
        Self {
            recv,
            connections: HashMap::new(),
            watch_streams: StreamMap::new(),
            room: Room::new(room_id),
        }
    }

    async fn run(mut self) {
        loop {
            tokio::select! {
                Some(event) = self.recv.recv() => {
                    tracing::debug!(?event, "room event");

                    match event {
                        RoomEvent::Register(event) => self.on_register(event).await,
                        RoomEvent::AddWebsocket(event) => self.on_add_websocket(event).await,
                        RoomEvent::Vote(event) => self.on_vote(event).await,
                        RoomEvent::Reveal(event) => self.on_reveal(event).await,
                        RoomEvent::Reset(event) => self.on_reset(event).await,
                    }
                }
                Some((uuid, _)) = self.watch_streams.next() => {
                    self.connections.remove(&uuid);
                    self.watch_streams.remove(&uuid);
                    self.room.remove_participant(uuid);
                    self.update_status().await;
                }
                else => break,
            }
        }
    }

    async fn on_register(&mut self, event: RegisterEvent) {
        self.room.add_participant(event.uuid, event.name);
        self.update_status().await;
        let _ = event.respond_to.send(self.room.clone());
    }

    async fn on_add_websocket(&mut self, event: AddWebsocketEvent) {
        let connection = ConnectionHandle::new(event.websocket);
        let watch = WatchStream::new(connection.connection_watch())
            .filter(is_disconnected as fn(&bool) -> bool);

        self.connections.insert(event.uuid.clone(), connection);
        self.watch_streams.insert(event.uuid.clone(), watch);
        self.update_status().await;
        let _ = event.respond_to.send(());
    }

    async fn on_vote(&mut self, event: VoteEvent) {
        self.room.vote(event.uuid, event.vote);
        self.update_status().await;
        let _ = event.respond_to.send(());
    }

    async fn on_reveal(&mut self, event: RevealEvent) {
        self.room.reveal();
        self.update_status().await;
        let _ = event.respond_to.send(());
    }

    async fn on_reset(&mut self, event: ResetEvent) {
        self.room.reset();
        self.update_status().await;
        let _ = event.respond_to.send(());
    }

    async fn update_status(&mut self) {
        let payload = serde_json::to_value(&self.room).unwrap();

        for handle in self.connections.values() {
            handle.send_room_details(payload.clone());
        }
    }
}

fn is_disconnected(conn: &bool) -> bool {
    !*conn
}

#[derive(Clone, Debug)]
pub struct RoomHandle {
    send: mpsc::Sender<RoomEvent>,
}

impl RoomHandle {
    pub fn new(room_id: String) -> Self {
        let (send, recv) = mpsc::channel(8);
        let actor = RoomActor::new(room_id.clone(), recv);
        tokio::spawn(async move { actor.run().await });

        Self { send }
    }

    pub async fn register(&self, uuid: String, name: String) -> Room {
        self.request(|respond_to| RoomEvent::Register(RegisterEvent { uuid, name, respond_to }))
            .await
    }

    pub async fn add_websocket(&self, uuid: String, websocket: WebSocketStream) {
        self.request(|respond_to| {
            RoomEvent::AddWebsocket(AddWebsocketEvent { uuid, websocket, respond_to })
        })
        .await
    }

    pub async fn vote(&self, uuid: String, vote: VoteSizes) {
        self.request(|respond_to| RoomEvent::Vote(VoteEvent { uuid, vote, respond_to }))
            .await
    }

    pub async fn reveal(&self) -> () {
        self.request(|respond_to| RoomEvent::Reveal(RevealEvent { respond_to })).await
    }

    pub async fn reset(&self) -> () {
        self.request(|respond_to| RoomEvent::Reset(ResetEvent { respond_to })).await
    }

    async fn request<R, F>(&self, build: F) -> R
    where
        F: FnOnce(oneshot::Sender<R>) -> RoomEvent,
    {
        let (tx, rx) = oneshot::channel::<R>();
        let event = build(tx);
        let _ = self.send.send(event).await;
        rx.await.expect("room actor dropped response")
    }
}
