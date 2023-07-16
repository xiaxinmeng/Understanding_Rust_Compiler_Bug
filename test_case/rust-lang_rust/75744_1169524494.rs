rust
fn g() -> std::ops::ControlFlow<u8> {
    std::ops::ControlFlow::Break(1)
} 

fn f() -> u8 {
    g()?; // does not compile in 1.61: E0277  the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    0
}

// f() returns 1.
