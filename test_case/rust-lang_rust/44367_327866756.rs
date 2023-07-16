
pub fn bar() {
    foo(0.0);
}

#[target_feature = "-sse,-sse2"]
pub fn foo(x: f32) -> f32 {
    x + x
}
