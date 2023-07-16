rust
use std::sync::Once;

static INIT: Once = Once::new();

fn main() {
    INIT.call_once(|| unsafe {
        *(0xaabbccdd as *mut usize) = 1234;
    });
}
