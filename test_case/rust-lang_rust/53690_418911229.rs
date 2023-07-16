rust
async fn foo(x: bool) -> u32 {
    if x {
        await!(foo(false)) + 1
    }
    else {
        4
    }
}
