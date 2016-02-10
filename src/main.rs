use std::sync::mpsc::{SendError};

mod actress {
    use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::{Sender};

    pub trait Recv {
        type MessageType : Send + 'static;
        fn recv(&mut self, msg: Self::MessageType) -> bool;
    }

    pub fn spawn<T: Recv + Send + 'static>(mut contained: T) -> Sender<T::MessageType> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(result) => if contained.recv(result) == true { break },
                    Err(_) => break
                }
            }
        });

        tx
    }
}

enum Command { Update(i32), Show, Quit }

struct Counter {
    state: i32
}

impl actress::Recv for Counter {
    type MessageType = Command;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Command::Update(x) => { self.state = x; false }
            Command::Show => { println!("Current value is: {}", self.state); false },
            Command::Quit => true
        }
    }
}

fn act() -> Result<(), SendError<Command>> {
    use std::io;

    let counter = Counter { state: 0 };

    let actor = actress::spawn(counter);
    try!(actor.send(Command::Update(30)));
    try!(actor.send(Command::Show));
    try!(actor.send(Command::Update(10)));
    try!(actor.send(Command::Show));
    try!(actor.send(Command::Quit));

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    Ok(())
}

fn main() {

    match act() {
        Ok(_) => println!("Message sends complete"),
        _ => println!("Message sends had errors")
    }
}
