rust
use std::{sync::Arc, task::Waker};

let waker = Waker::from(Arc::new(|| {
    println!("Woken !");
}));
