rust
fn qux() -> dyn std::fmt::Display {
    if false {
        0i32
    } else {
        1u32 //~ ERROR `if` and `else` have incompatible types
    }
}
