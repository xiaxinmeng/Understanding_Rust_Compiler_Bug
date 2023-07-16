rust
use std::sync::Once;
use core::panic::AssertUnwindSafe;

fn main() {
    let o = Once::new();
    let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
        o.call_once(|| panic!("foo"));
    }));
    o.call_once(|| {});
}
