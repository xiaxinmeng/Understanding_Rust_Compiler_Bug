rust
fn foo() -> f32 {
    let a = if true {
        return 3.0; // `<float>`
    } else {
        3 // `<integer>`
    };
    0.0
}
