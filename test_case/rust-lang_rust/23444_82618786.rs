 rust
fn f(x: Option<bool>, y: bool) -> i32 {
    if let Some(x) = x {
        // lots of code here
        1
    } else if y {
        // lots of code here
        1
    }
}
