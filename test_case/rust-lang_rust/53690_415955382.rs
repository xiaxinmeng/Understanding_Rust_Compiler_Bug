rust
async fn foo() -> u32 {
    let x = Box::new(foo());
    await!(x) + 1
}
