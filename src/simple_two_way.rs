extern crate actress;

use std::sync::mpsc::{Sender, SendError, channel};

enum Message { Ping(Sender<Message>), Pong }

struct NullActor;

impl actress::Actor for NullActor {
    type MessageType = Message;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Message::Ping(c) => { println!("Ping!"); c.send(Message::Pong).unwrap(); true },
            _ => { false }
        }
    }
}

fn act() -> Result<(), SendError<Message>> {
    let null_actor = NullActor;

    let (response_tx, response_rx) = channel();

    let actor = actress::spawn(null_actor);
    try!(actor.send(Message::Ping(response_tx)));

    match response_rx.recv() {
        Ok(Message::Pong) => println!("Pong!"),
        _ => println!("Error receiving response from actor")
    }

    Ok(())
}

fn main() {
    match act() {
        Ok(_) => println!("Message sends complete"),
        _ => println!("Message sends had errors")
    }
}
