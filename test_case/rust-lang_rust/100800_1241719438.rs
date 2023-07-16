rust
#![feature(type_alias_impl_trait)]

trait Foo {
    fn foo(&self) {println!("A");}
}
impl<T> Foo for T {}

struct B;
impl B {
    fn foo(&self) {println!("B");}
}

type Input = impl Foo;
fn run1<F: FnOnce(Input)>(f: F, i: Input) {f(i)}
fn run2<F: FnOnce(B)>(f: F, i: B) {f(i)}

fn main() {
    run1(|x: B| {x.foo()}, B);
    run2(|x: B| {x.foo()}, B);
}
