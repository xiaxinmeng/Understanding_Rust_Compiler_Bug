rust
#![allow(unused)]
#![warn(elided_lifetimes_in_paths)]

struct DoubleLifetime<'a, 'b> {
    one: &'a str,
    another: &'b str,
}

fn accepts_double_lifetime(dl: DoubleLifetime) -> String {
    format!("{} {}", dl.one, dl.another)
}

fn main() {}
