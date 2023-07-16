rust
async fn foo() {
    let mut f = None;
    let value = 0;
    f = Some(async move { value });
    core::mem::drop(f);
}
