rust
#![feature(inline_const)]

#[inline(always)]
fn falser() -> bool {
    false
}

fn has_late_error<T>() {
    const {
        panic!()
    };
}

fn main() {
    if falser() {
        has_late_error::<()>();
    }
}
