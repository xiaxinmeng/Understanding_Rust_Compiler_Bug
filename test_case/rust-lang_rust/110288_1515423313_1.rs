rust
pub const fn f() {
    // OK:
    let _a: [Option<String>; 2] = [None, None];
    let _b: &'static [Option<String>; 2] = &[None, None];
}
