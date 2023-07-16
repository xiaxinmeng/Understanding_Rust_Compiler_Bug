
type foo<A> = { foo: int };
fn mk_foo<A>() -> foo<A> { {foo: 42} }
fn main() { 
    let bar: foo<int> = mk_foo();
}
