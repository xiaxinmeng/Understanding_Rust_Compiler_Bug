rust
async fn a() -> Result<(), ()> {
    Ok(())
}

fn main() {
    match a() {
        Ok(()) => (),
        Err(()) => (),
    }
}
