rust
async fn foo_block() {
    let a = #[track_caller] async {
        panic!();
    };
    a.await // <- panic location points to the `await`
}
