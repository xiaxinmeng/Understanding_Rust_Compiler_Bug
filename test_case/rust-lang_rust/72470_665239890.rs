rust
// src/main.rs
use tokio;
use std::error::Error;
use std::sync::Mutex;

#[allow(dead_code)]
enum Msg {
    A(Vec<()>),
    B,
}

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_, mut rx) = tokio::sync::mpsc::unbounded_channel::<Msg>();
    let entity = Box::new(Mutex::new(()));

    tokio::spawn(async move {
        tokio::select! {
            Some(msg) = rx.recv() => {
                entity.lock();
                drop(msg);
            }
        }
        entity.lock();
    });

    return Ok(());
}
