rust
use std::task::Waker;

let waker = Waker::from_fn(|| {
    println!("Woken !");
});
