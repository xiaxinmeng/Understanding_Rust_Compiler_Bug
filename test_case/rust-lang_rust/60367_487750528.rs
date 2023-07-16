rust
#![feature(impl_trait_in_bindings)]

static mut TEST: Option<impl core::fmt::Debug> = None;

fn main() {
    unsafe {
        TEST = Some(0)
    }
}
