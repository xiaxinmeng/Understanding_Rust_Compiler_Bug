rust
struct S{ n: u8 }
struct T<'a>{ s: &'a S }
impl<'a> Drop for T<'a> { fn drop(&mut self){} }

// The `if-else` makes it work...
fn test_if_else() -> u8 {
    let s = S{ n: 5 };
    if true {
        T{ s: &s }.s.n
    } else {
        T{ s: &s }.s.n
    }
}
