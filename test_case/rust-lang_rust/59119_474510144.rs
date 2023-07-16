rust
fn foo(waker: &Waker) {
    invoke_one_or_the_other(|| use(waker), || use(waker))
}
