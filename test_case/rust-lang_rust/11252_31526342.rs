 rust
// Because ~[u8], it's represented as a pointer to len, capacity and data,
// so it would be passed ByValue (datum-wise).
fn foo(x: ~[u8]) -> ~str {
    unsafe {
        // This call would move out of x, so the original cleanup
        // has to be revoked. but this isn't in the same scope as
        // the original, so only ByRef(ZeroMem) can be revoked
        // and that produces bloated code (which I wanted to avoid).
        ::std::unstable::intrinsics::transmute(x)
    }
}
fn main() {
    foo(~[]);
}
