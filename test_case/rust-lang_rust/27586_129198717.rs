 Rust
fn undefined<T>() -> T { unreachable!() }
fn main() {
    match undefined() {
        &ref x => {}
    }
}
