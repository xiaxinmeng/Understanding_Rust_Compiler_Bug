rust
fn foo() -> i32 {
    return 1;
    let tmp: f32 = 0.0;
    tmp // expected `i32`, found `f32`
}
