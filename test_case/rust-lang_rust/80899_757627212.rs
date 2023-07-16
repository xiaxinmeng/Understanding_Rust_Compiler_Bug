rust
fn transmute_to_self<T>(t: T) -> T {
    unsafe { core::mem::transmute(t) }
}
