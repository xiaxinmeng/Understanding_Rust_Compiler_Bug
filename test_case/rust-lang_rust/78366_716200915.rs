rust
async fn bar() -> usize {
    unimplemented!()
}

async fn foo(status: usize) {
    match status {
        x if bar().await == (x + 1) => {}
        _ => {}
    }
}
