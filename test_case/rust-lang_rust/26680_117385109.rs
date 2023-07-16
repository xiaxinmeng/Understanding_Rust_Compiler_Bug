 rust
pub trait Thing<'a> {
    type X = &'a str;

    fn foo(x: Self::X) -> String {
        x.to_string()
    }
}

struct Impl;
impl<'a> Thing<'a> for Impl {}

fn main() {
    println!("{}", Impl::foo("wow"));
}
