rust
async fn foo(x: Result<(), ()>) {
    match x {
        Ok(()) | Err(_e) => (),
    }
}
