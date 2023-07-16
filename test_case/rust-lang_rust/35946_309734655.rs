rust
fn result() -> Result<i32, &'static str> {
    let x: u32 = Ok("")?;
    unimplemented!()
}
