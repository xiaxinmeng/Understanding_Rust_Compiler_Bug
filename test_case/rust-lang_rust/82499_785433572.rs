rust
use core::sync::atomic::{AtomicUsize, Ordering};

#[link_section = ".rodata"]
static BAD: AtomicUsize = AtomicUsize::new(0);

fn main() {
    BAD.store(42, Ordering::SeqCst);
}
