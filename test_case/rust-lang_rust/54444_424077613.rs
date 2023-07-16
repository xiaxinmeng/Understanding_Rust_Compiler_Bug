
pub fn foo(x: bool) { // Ret
    if x {
        foo(x);  // Ret
    } else {
        loop {}   // `!`, can't return
    }
}
