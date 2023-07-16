 rust
trait T<'a> {
    fn foo<'b, A>(&'a self) -> A;
}

impl<'a> T<'a> for () {
    fn foo<'b, A>(&'a self) -> A where 'a : 'b { panic!("bleh") }
}

fn main() {}
