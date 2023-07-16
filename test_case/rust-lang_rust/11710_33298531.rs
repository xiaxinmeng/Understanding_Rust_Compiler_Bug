 rust
use std::util;
enum Option<T> {
    None,
    Some(T),
}

#[inline(never)]
fn function<T>(slot: &mut Option<T>, f: T) {
    let a = util::replace(slot, None);
    let _breaker = match a {
        Some(_) => bar(f),
        None => f,
    };
}

#[inline(never)]
fn bar<T>(g: T) -> T {
    g
}

#[start]
fn main(_: int, _: **u8) -> int {
    let mut slot = None;
    function(&mut slot, proc() {});
    0
}
