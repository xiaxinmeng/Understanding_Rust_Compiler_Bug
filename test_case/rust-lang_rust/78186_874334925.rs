rust
static MUTEX: MutexThatGetsVeryAngryIfItIsMoved = ...;

impl MutexThatGetsVeryAngryIfItIsMoved {
    pub fn lock(self: Pin<&Self>) -> ... { ... }
}

fn something() {
    let guard = Pin::static_ref(&MUTEX).lock();
    ...
}
