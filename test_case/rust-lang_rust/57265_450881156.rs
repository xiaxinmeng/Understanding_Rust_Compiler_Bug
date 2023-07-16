rust
fn call<'a, T>(x: &'a T, f: fn(&'a T) -> S<&'a T>) {
    let y = f(x);
}
