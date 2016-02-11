# actress
A simple actor library in Rust.  Still pre-1.0 aka "not really ready for primetime"

# Intro

Actress leverages much of what already comes with Rust and is based around the idea that your data and your structures are central.

To this end, Acress supports one trait, called ```Actor``` that represents the minimal requirements on your data types to be used in the actor system.  Once you pass a value that meets these requirements into spawn, the system effectively turns your data structure into an actor/active object.  After doing so, it hands you the channel end-point that you can communicate with the actor.

Once an actor is spawned, you can't have any more direct interaction with it.  In essence, it becomes isolated thanks to Rust's type system, effectively making all communication with it to asynchronous.

Another key feature is that communicating with the actors is all-in-one via the message channels.  That is, the end-point you get when you spawn an actor serves as a way of also detecting the health of an actor.  If an actor has closed its side of the channel, then this will be detectable as you drain the messages.

# Usage

Include the actress library

```Rust
use actress
```

Then, implement the Actor trait on your struct

```Rust
struct SimpleActor {
  state: i32
}

impl actress::Actor for SimpleActor {
  type MessageType = i32;
  fn recv(&mut self, msg: i32) -> bool { self.state = msg; msg == 42 }
}
```

Finally, spawn and use your new actor

```Rust
let simple_actor = SimpleActor { state: 1 };
let actor = actress::spawn(simple_actor);
actor.send(10);
```

# Examples

## One-way echo 

```Rust
use actress;

use std::sync::mpsc::{SendError};

enum Message { Ping }

struct EchoActor;

impl actress::Actor for EchoActor {
    type MessageType = Message;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Message::Ping => { println!("Pong!"); true }
        }
    }
}

fn act() -> Result<(), SendError<Message>> {
    let echo_actor = EchoActor;

    let (actor, handle) = actress::spawn_with_handle(echo_actor);

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
```

## Two-way echo
```rust
use actress;

use std::sync::mpsc::{Sender, SendError, channel};

enum Message { Ping(Sender<Message>), Pong }

struct EchoActor;

impl actress::Actor for EchoActor {
    type MessageType = Message;

    fn recv(&mut self, msg: Self::MessageType) -> bool {
        match msg {
            Message::Ping(c) => { println!("Ping!"); c.send(Message::Pong).unwrap(); true },
            _ => { false }
        }
    }
}

fn act() -> Result<(), SendError<Message>> {
    let echo_actor = EchoActor;

    let (response_tx, response_rx) = channel();

    let actor = actress::spawn(echo_actor);
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
```
