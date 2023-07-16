 rust
struct Unused<T>(T);

fn return_ok() -> Result<(), ()> {
    Ok(())
}

fn main() {
    Unused(return_ok()); // no warning
}
