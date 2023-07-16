 rust
trait T<'a> {
    fn foo<'b : 'a, A>(&'a self) -> A;
}

impl<'a> T<'a> for () {
    fn foo<'b : 'a, A>(&'a self) -> A where 'a : 'b { panic!("bleh") }
}

fn main() {}
