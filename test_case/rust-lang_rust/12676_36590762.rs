 rust
fn f() -> Result<~str, Error> { ... }
fn g() -> Result<int, Error> { ... }

fn h() -> Result<f64, Error> {
    try!(f());
    try!(g());
    // ...
    Ok(3.14)
}
