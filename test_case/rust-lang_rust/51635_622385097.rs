rust
fn gen<T>() -> Option<T> { None }

#[quoter]
fn beta() {
    gen::< Result<(),()> >();

    let _ = 1 + 1.0;
}
