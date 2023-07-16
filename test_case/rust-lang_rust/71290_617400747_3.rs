rust
pub fn foo(a: bool, b: bool) -> u8 {
    (if a { 1 } else { 0 }) + (if b { 1 } else { 0 })
}
