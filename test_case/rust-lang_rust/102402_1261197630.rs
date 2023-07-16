rust
pub fn foo(n: usize) -> bool {
    let black_box_zero = (n - 1) as f32;
    
    let result = 1.0 / ((-0.0) * black_box_zero);

    result == std::f32::NEG_INFINITY
}
