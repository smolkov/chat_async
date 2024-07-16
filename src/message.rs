use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use crate::error::ChatError;

#[derive(Debug,Clone)]
pub struct TextMessage {
    user: String,
    content: String,
}
#[derive(Clone,Debug)]
pub enum Message {
    Text(TextMessage),
    ChangeName(String),
    ChangeRoom(String),
}

impl FromStr for Message {
    type Err = ChatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((id, data)) = s.split_once(':') {
            let id: usize = id.parse()?;
            match id {
                1 => Ok(Message::Text(TextMessage::from_str(data)?)),
                2 => Ok(Message::ChangeName(data.to_owned())),
                3 => Ok(Message::ChangeRoom(data.to_owned())),
                id => Err(ChatError::WrongIdError(id)),
            }
        } else {
            Err(ChatError::DecodeMessageError)
        }
    }
}

impl FromStr for TextMessage {
    type Err = ChatError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((user, content)) = s.split_once(':') {
            Ok(TextMessage {
                user: user.to_owned(),
                content: content.to_owned(),
            })
        } else {
            Err(ChatError::DecodeMessageError)
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Text(m)  => write!(f, "{}", m),
            Message::ChangeName(s) => write!(f, "{}", s),
            Message::ChangeRoom(s) => write!(f, "{}", s),
        }
    }
}

impl Display for TextMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.user, self.content)
    }
}
