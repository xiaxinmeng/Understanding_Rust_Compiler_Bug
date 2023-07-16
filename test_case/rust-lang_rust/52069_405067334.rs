rust
#![allow(unused)]
#![warn(elided_lifetimes_in_paths)]

struct DoubleLifetime<'a, 'b> {
    one: &'a str,
    another: &'b str,
}

fn returns_double_lifetime(one: &'a str, another: &'b str) -> DoubleLifetime {
    DoubleLifetime { one, another }
}

fn main() {}
