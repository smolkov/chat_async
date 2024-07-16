use std::{any, str::FromStr};

use anyhow::Result;
use chat::message::Message;
use chat::{room::Room, state::State};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    select, signal, spawn,
    sync::{broadcast, mpsc},
};

struct Session {
    user: String,
    stream: TcpStream,
    room: Room,
    state: State,
}

impl Session {
    pub async fn read_message(&mut self) -> Result<Message> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut buffer = String::new();
        let len = reader.read_line(&mut buffer).await?;
        if len == 0 {
            return Err(anyhow::anyhow!("Client close connection"));
        }
        Ok(Message::from_str(buffer.as_str())?)
    }

    pub async fn write_message(&mut self, message: Message) -> Result<()> {
        self.stream.write_all(format!("{message}").as_bytes()).await?;
        Ok(())
    }

    pub async fn process_message(&mut self, message: Message) -> Result<()> {
        match message {
            Message::Text(msg) => {
                println!("send message");
            }
            Message::ChangeName(name) => self.user = name,
            Message::ChangeRoom(room) => {
                println!("change room");
            }
        }
        Ok(())
    }
}

async fn process(mut session: Session) -> Result<()> {
    let mut rx = session.room.receiver.resubscribe();
    loop {
        select! {
            message = session.read_message() => {
                match message? {
                    Message::Text(msg)  => {session.room.sender.send(Message::Text(msg))?;},
                    Message::ChangeName(_name) => { println!("Change name")},
                    Message::ChangeRoom(room) => {
                        println!("change room {room}");
                        session.room = session.state.subscribe_room(&room).await?;
                        rx = session.room.receiver.resubscribe();
                    }
                }
            },
            message = rx.recv() => {
                if message.is_err()  {
                    break;
                }
                if let  Message::Text(msg) =  message? {
                    session.write_message(Message::Text(msg)).await?;
                }
            }
        }
    }
    Ok(())
}

async fn server() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:3000").await?;
    let mut state = State::new();
    let mut users: usize = 1;
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            println!("Connect new user {}", addr);
            let session = Session {
                stream: socket,
                room: state.subscribe_room("default").await?,
                user: format!("User{users}"),
                state: state.clone(),
            };
            spawn(async {
                let err = process(session).await;
                println!("Connection closet");
            });
            users += 1;
        }
    }
    Ok(())
}

async fn shutdown() -> Result<()> {
    signal::ctrl_c().await?;
    println!("shutdown..");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    select! {
        _ = server() => {

        },
        _ = shutdown() => {

        }
    }
    Ok(())
}
