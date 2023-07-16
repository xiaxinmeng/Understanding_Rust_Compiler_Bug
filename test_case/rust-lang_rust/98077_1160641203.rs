rust
async fn foo() {
    let mut f = None;
    let value = 0;
    f = Some(async { value }.await);
    core::mem::drop(f);
}
