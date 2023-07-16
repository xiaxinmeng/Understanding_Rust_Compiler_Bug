rust
// With the Wake trait
use std::task::{Waker, Wake};
use crossbeam::sync::{Unparker};

struct BlockOnWaker {
    unparker: Unparker,
}

impl Wake for BlockOnWaker {
    fn wake(self: Arc<Self>) {
        self.unparker.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let unparker = ...;
    let waker = Waker::from(Arc::new(BlockOnWaker { unparker }));
    ...
}
