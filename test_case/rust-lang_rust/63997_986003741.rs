rust
struct Handlers([Option<fn()>; _]);

impl Handlers {
    const fn new() -> Self {
        Self([None; _])
    }
}
