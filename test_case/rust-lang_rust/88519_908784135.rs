rust
pub fn foo_bar(val: i32) -> i32 {
    (if val == 1 { 0 } else { val }) as i32
}
