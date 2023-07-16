 rust
trait T {
    fn foo<'a, 'b, A>(self) -> A;
}

impl T for () {
    fn foo<'a, 'b, A>(self) -> A where 'a : 'b {}
}

fn main() {}
