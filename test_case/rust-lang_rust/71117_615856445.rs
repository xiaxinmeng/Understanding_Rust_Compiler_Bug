rust
fn foo(x: &mut i32) {
    // We are allowed to move the write down below the call, because if `bar`
    // reads or writes `x`, it causes immediate UB.
    // Without protectors this transformation would be illegal as `bar` could just pop `x`'s
    // tag off the stack without any bad consequences, since `x` does not get used again.
    *x = 13;
    bar();
}
