rust
async fn some_future() -> u8 { 1 }

pub async fn bad_mir(x: u8) {
    match x {
        y if some_future().await == y => (),
        _ => (),
    }
}
