extern crate actress;

use std::sync::mpsc::{SendError};

enum Message { Ping }

struct NullActor;

impl actress::Recv for NullActor {
    type MessageType = Message;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Message::Ping => { println!("Pong!"); true }
        }
    }
}

fn act() -> Result<(), SendError<Message>> {
    let null_actor = NullActor;

    let (actor, handle) = actress::spawn_with_handle(null_actor);

    println!("Ping!");
    try!(actor.send(Message::Ping));

    handle.join().unwrap();

    Ok(())
}

fn main() {
    match act() {
        Ok(_) => println!("Message sends complete"),
        _ => println!("Message sends had errors")
    }
}
