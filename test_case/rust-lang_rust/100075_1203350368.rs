rust
fn opaque<T>(t: T) -> impl Sized {
    opaque(Some(t))
}

#[allow(dead_code)]
fn main() {}
