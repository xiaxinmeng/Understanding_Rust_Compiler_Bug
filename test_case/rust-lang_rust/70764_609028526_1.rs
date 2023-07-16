rust
// With this API
use std::task::Waker;

fn block_on<F: Future>(future: F) -> F::Output {
    let unparker = ...;
    let waker = Waker::from(Arc::new(move || unparker.unpark()));
    ...
}
