rust
struct DropBomb;

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("double panicking to abort");
    }
}

fn panic_abort() {
    let _guard = DropBomb;
    panic!("attempted to leave ! uninitialized")
}
