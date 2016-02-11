extern crate actress;

use std::sync::mpsc::{Sender, SendError, channel};

enum Response { Ack(i32) }
enum Command { Update(i32), Tell(Sender<Response>) }

struct Counter {
    state: i32
}

impl actress::Actor for Counter {
    type MessageType = Command;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Command::Update(x) => { self.state = x; false },
            Command::Tell(c) => { c.send(Response::Ack(self.state)).unwrap(); true },
        }
    }
}

fn act() -> Result<(), SendError<Command>> {
    let counter = Counter { state: 0 };

    let (response_tx, response_rx) = channel();

    let actor = actress::spawn(counter);
    try!(actor.send(Command::Update(30)));
    try!(actor.send(Command::Tell(response_tx)));

    match response_rx.recv() {
        Ok(Response::Ack(v)) => println!("Response: {}", v),
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
