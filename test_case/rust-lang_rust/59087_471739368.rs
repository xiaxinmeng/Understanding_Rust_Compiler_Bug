rust
async fn composed() {
    let inner = i_am_1kb();
    { let _foo = &inner; }
    await!(inner);
}
