
fn foo(+blk: fn(+p: &a/fn() -> &a/fn())) {
    let mut state = 0;
    let statep = &mut state;
    do blk {
        || { *statep = 1; }
    }
}
fn main() {
    do foo |p| { p()() }
}


frames.rs:1:0: 1:0 error: Unconstrained region variable #8
frames.rs:1 fn foo(+blk: fn(+p: &a/fn() -> &a/fn())) {
            ^
frames.rs:1:0: 1:0 error: Unconstrained region variable #10
frames.rs:1 fn foo(+blk: fn(+p: &a/fn() -> &a/fn())) {
            ^
