Rust
fn main() {
    Ok(4)?; //~ ERROR the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
}

fn foo() -> Result<(), ()> {
    ()?; //~ ERROR <want something else>
    Ok(())
}
