rust
use std::time::Duration;
struct A {
    b: u64,
}

fn main() {
    let a = A { b: 1 };
    bar(|| {}, a);
}

fn bar<F>(predicate: F, a: A)
where
    F: FnMut(),
{
    if Duration::new(0, 0) == Duration::from_millis(a.b) {}
}
