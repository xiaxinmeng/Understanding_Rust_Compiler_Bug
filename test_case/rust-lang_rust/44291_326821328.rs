rust
fn joshtriplett(hi: bool) {
    let closure = || ();
    if hi {
        let _rust_fn: fn() = closure;
    } else {
        let _c_fn: extern fn() = closure;
    }
}
