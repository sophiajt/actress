use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender};

pub trait Recv {
    type MessageType : Send + 'static;
    fn recv(&mut self, msg: Self::MessageType) -> bool;
}

pub fn spawn_with_handle<T: Recv + Send + 'static>(mut contained: T) -> (Sender<T::MessageType>, JoinHandle<()>) {
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

pub fn spawn<T: Recv + Send + 'static>(contained: T) -> Sender<T::MessageType> {
    let (tx, _) = spawn_with_handle(contained);
    tx
}
