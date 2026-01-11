use std::sync::Arc;

use dashmap::DashMap;

use crate::actors::room::RoomHandle;

#[derive(Clone, Debug)]
pub struct AppState {
    pub room_registry: RoomRegistry,
}

impl AppState {
    pub fn new() -> Self {
        Self { room_registry: RoomRegistry::new() }
    }
}

#[derive(Clone, Debug)]
pub struct RoomRegistry {
    pub rooms: Arc<DashMap<String, RoomHandle>>,
}

impl RoomRegistry {
    pub fn new() -> Self {
        Self { rooms: Arc::new(DashMap::new()) }
    }

    pub fn get_or_create(&self, room_id: String) -> RoomHandle {
        self.rooms
            .entry(room_id.clone())
            .or_insert_with(|| RoomHandle::new(room_id.clone()))
            .clone()
    }
}
