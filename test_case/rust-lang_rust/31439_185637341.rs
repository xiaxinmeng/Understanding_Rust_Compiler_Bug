 rust
struct S{ n: u8 }
struct T<'a>{ s: &'a S }
impl<'a> Drop for T<'a> { fn drop(&mut self){} }

fn test_good() -> u8 { // this is fine
    let s = S{ n: 5 };
    return T{ s: &s }.s.n;
}

fn test_bad() -> u8 { // this is not
    let s = S{ n: 5 };
    T{ s: &s }.s.n
}

fn main(){}
