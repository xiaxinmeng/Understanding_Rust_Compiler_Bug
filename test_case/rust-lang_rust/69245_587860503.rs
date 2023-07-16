rust
unsafe fn bar1() {
    something_safe();
    unsafe { // OK: remove the lint as we want to allow this in the future
       something_unsafe();
    }
}

unsafe fn bar2() {
    something_unsafe(); // No lint there yet
    unsafe { // LINT: Keep the `unused_unsafe` if there is something unsafe outside 
       something_unsafe();
    }
}

unsafe fn bar3() {
    something_unsafe(); // No lint there yet, this would be the future `unsafe_in_unsafe_fn`
}
