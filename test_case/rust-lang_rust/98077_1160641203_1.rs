rust
async fn foo() {
    let value = 0;
    let mut f = None;
    f = Some(async { value });
    core::mem::drop(f);
}
