use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, Mutex, RwLock};
use anyhow::Result;

use crate::message::Message;

use super::room::Room;

pub struct StateInner {
    rooms: HashMap<String, Room>,
}
#[derive(Clone)]
pub struct State {
    pub inner: Arc<RwLock<StateInner>>,
}

impl State {
    pub fn new() -> State {
        let inner = StateInner {
            rooms: HashMap::new(),
        };
        State {
            inner: Arc::new(RwLock::new(inner)),
        }
    }
	pub async fn subscribe_room(&mut self, name:&str) -> Result<Room> {
		let mut inner = self.inner.write().await;
		if let Some(room) = inner.rooms.get(name) {
			Ok(room.subscribe())	
		}else {
			let room = Room::new();
			let subscribe = room.subscribe();
			inner.rooms.insert(name.to_owned(), room);	
			Ok(subscribe)
		}

	}
}
