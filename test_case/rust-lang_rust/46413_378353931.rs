rust
// #![feature(nll)]

struct Foo<'a>(&'a i32);

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {}
}

impl<'a> Foo<'a> {
    fn foo(&self) {}
}

fn bar() {
    let i = 3;
    Foo(&i).foo()
}

fn main() {
    bar();
}
