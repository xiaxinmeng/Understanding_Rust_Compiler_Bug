 rust
fn foo<'a>(x: &'a int) -> int { *x }
fn bar(x: &int) -> int { *x }

fn unwrap<T>(f: fn(&T) -> T, v: &T) -> T {
    f(v)
}

fn main() {
    let x = unwrap(foo, &42);
    let y = unwrap(bar, &32);
    assert!(x == 42);
    assert!(y == 32);
}
