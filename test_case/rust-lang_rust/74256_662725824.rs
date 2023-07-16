rust
async fn async_fn(x: &i32, y: &mut &i32) {
    *y = x;
}

fn sync_fn(x: &i32, y: &mut &i32) {
    *y = x;
}
