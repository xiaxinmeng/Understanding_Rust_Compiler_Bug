rust
 fn elided<'a>(x: &'a i32) -> impl Copy + 'a { x }
