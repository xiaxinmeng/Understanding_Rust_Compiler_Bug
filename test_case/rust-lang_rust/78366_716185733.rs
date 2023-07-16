rust
async fn bar() -> &'static [bool] {
    unimplemented!()
}

async fn foo(status: usize) {
    match status {
        x if bar().await[x] => {}
        _ => {}
    }
}
