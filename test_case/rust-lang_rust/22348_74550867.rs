 rust
#![deny(unused_must_use)]

fn f() -> Result<(), ()> {
    Ok(())
}

fn main() {
    f().ok();
}
