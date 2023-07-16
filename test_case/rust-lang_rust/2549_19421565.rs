
fn foo(arg: fn(callback: once fn())) {
    let x = NonCopyable;
    do arg { move_out_of(x); }; // requires arg's argument to be 1-shot, to move out
}
