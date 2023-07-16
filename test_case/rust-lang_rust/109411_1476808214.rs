Rust
#![feature(target_feature_11)]

use core::panic::PanicInfo;

#[target_feature(enable = "avx2")]
fn panic(_info: &PanicInfo) {
    loop {}
}

fn main() {
    std::panic::set_hook(Box::new(panic));
}
