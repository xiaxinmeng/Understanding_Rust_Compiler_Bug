 rust
struct X {
    x: ~uint,
}

#[inline(never)]
fn function(mut slot: Option<X>) {
    let a = std::util::replace(&mut slot, None);
    let _breaker = match a {
        Some(_) => ~1,
        None => ~1,
    };
}

#[start]
fn main(_: int, _: **u8) -> int {
    function(None);
    0
}
