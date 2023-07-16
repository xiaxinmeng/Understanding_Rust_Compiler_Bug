rust
async fn composed() {
    let inner = i_am_1kb();
    { foo(); fn foo() { } }
    await!(inner);
}
