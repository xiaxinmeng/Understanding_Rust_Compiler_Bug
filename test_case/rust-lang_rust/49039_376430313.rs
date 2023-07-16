rust
#![deny(warnings)]
fn foo() -> Result<(), !> {
    Ok(())
}
fn main() {
    let bar = foo() .map_err(|e| e.to_string());
    println!("{:?}", bar);
}
