rust
async fn foo() {
    let x = CreateMyNonSendCopyType();
    let y = &x as *const _;
    leak_ptr_to_somewhere(y);
    some_future.await();
    ... // some other code that doesn't reference `x` or `y`
}
