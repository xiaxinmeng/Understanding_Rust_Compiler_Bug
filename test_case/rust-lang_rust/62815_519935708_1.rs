rust
impl Closure for /*typeof(|| x + y)*/ {
    type Captures = (Capture<"x", i32>, Capture<"y", i32>);
    fn captures_ref(&self) -> &Self::Captures {
        // Note that this requires the closure to be represented as its Captures:
        unsafe { &*(self as *const _ as *const _) }
    }
}
