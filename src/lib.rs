//! Actress - a simple actor library for Rust
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender};


/// Trait required to be compatible as an actor
///
/// # Examples
///
/// ```
/// use actress;
///
/// struct NullActor;
///
/// impl actress::Actor for NullActor {
///     type MessageType = i32;
/// 
///     fn recv(&mut self, msg: i32) -> bool { msg == 42 }
/// }
/// ```
pub trait Actor {
    /// The type of message the actor can receive.
    type MessageType : Send + 'static;

    /// A handler to receive messages from the channel.  Returning 'true' will signal that this actor has 
    /// completed.
    fn recv(&mut self, msg: Self::MessageType) -> bool;
}

/// Spawns an actor with the given struct instance.  Returns a tuple of a channel ```Sender``` to communicate
/// with the newly-spawned actor as well as a ```JoinHandle``` to later join the actor.
///
/// # Examples
///
/// ```
/// struct NullActor;
///
/// impl actress::Actor for NullActor {
///     type MessageType = i32;
/// 
///     fn recv(&mut self, msg: i32) -> bool { msg == 42 }
/// }
///
/// fn main() {
///    let null_actor = NullActor;
///    let (actor, handle) = actress::spawn_with_handle(null_actor);
/// }
/// ```
pub fn spawn_with_handle<T: Actor + Send + 'static>(mut contained: T) -> (Sender<T::MessageType>, JoinHandle<()>) {
    let (tx, rx) = channel();
    let handle = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(result) => if contained.recv(result) == true { break },
                Err(_) => break
            }
        }
    });

    (tx, handle)
}

/// Spawns an actor with the given struct instance.  Returns a channel ```Sender``` to communicate
/// with the newly-spawned actor.
///
/// # Examples
///
/// ```
/// struct NullActor;
///
/// impl actress::Actor for NullActor {
///     type MessageType = i32;
/// 
///     fn recv(&mut self, msg: i32) -> bool { msg == 42 }
/// }
///
/// fn main() {
///    let null_actor = NullActor;
///    let actor = actress::spawn(null_actor);
/// }
/// ```
pub fn spawn<T: Actor + Send + 'static>(contained: T) -> Sender<T::MessageType> {
    let (tx, _) = spawn_with_handle(contained);
    tx
}
