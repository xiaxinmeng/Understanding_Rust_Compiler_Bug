 rust
fn test() -> Box<std::any::Any + 'static> { box 1i }
println!("{}", test())
