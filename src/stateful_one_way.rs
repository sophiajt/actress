extern crate actress;

use std::sync::mpsc::{SendError};

enum Command { Update(i32), Show, Quit }

struct Counter {
    state: i32
}

impl actress::Recv for Counter {
    type MessageType = Command;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Command::Update(x) => { self.state = x; false },
            Command::Show => { println!("Current value is: {}", self.state); false },
            Command::Quit => true
        }
    }
}

fn act() -> Result<(), SendError<Command>> {
    let counter = Counter { state: 0 };

    let (actor, handle) = actress::spawn_with_handle(counter);
    try!(actor.send(Command::Update(30)));
    try!(actor.send(Command::Show));
    try!(actor.send(Command::Update(10)));
    try!(actor.send(Command::Show));
    try!(actor.send(Command::Quit));

    handle.join().unwrap();

    Ok(())
}

fn main() {
    match act() {
        Ok(_) => println!("Message sends complete"),
        _ => println!("Message sends had errors")
    }
}
