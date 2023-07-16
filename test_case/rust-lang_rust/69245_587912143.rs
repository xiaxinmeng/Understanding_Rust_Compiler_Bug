rust
unsafe fn parent() {
    ...
    unsafe { foo(); } // copied from some other function
    bar(); // pre-existing
    ...
}
