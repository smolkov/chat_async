
use std::collections::HashMap;

use tokio::sync::broadcast;

use crate::message::Message;


pub struct Room {
	pub sender: broadcast::Sender<Message>,
	pub receiver: broadcast::Receiver<Message>
}



impl Room {
	pub fn new() -> Room {
		let (sender,receiver) = broadcast::channel(100);
		Room{
			sender,
			receiver
		}
	}
	pub fn subscribe(&self) -> Room {
		Room{
			sender: self.sender.clone(),
			receiver: self.receiver.resubscribe(),
		}
	}
}
