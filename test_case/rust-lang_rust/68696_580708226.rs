rust
use std::panic::catch_unwind;

struct CatchPanic;

impl Drop for CatchPanic {
    fn drop(&mut self) {
        let _ = catch_unwind(|| {
            panic!("1");
        });
    }
}

fn main() {
    let _ = catch_unwind(|| {
        let _catch = CatchPanic;
        panic!("2");
    });
}
