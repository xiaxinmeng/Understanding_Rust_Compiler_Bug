rust
fn main() {
    fn assert_send(_: impl Send) {}
    assert_send(async { // error: not Send
        let rc = std::rc::Rc::new(0);
        drop(rc);
        // rc is dropped but is still part of the state machine at the await point 
        async {}.await
    })
}
