rust
struct S{ n: u8 }
struct T<'a>{ s: &'a S }
impl<'a> Drop for T<'a> { fn drop(&mut self){} }

// This is fine
fn test_good() -> u8 {
    let s = S{ n: 5 };
    return T{ s: &s }.s.n;
}

// This isn't.
fn test_bad() -> u8 {
    let s = S{ n: 5 };
    T{ s: &s }.s.n
}
