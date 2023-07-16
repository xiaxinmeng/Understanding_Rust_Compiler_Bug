rust
pub fn foo(x: &mut i32, y: &mut i32) -> i32 {
    *x = 23;
    *y = 19;
    *x // optimize to 23
}
