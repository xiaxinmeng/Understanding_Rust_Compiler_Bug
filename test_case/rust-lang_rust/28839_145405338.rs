 rust
pub struct Foo;
// Accepted on stable and nightly
pub fn get_foo<'a, 'b>(foo: &'a mut Option<&'b mut Foo>) -> &'a mut Foo {
    match foo {
        &mut Some(ref mut x) => *x,
        &mut None => panic!(),
    }
}
// Rejected on nightly, accepted on stable
pub fn get_foo2<'a>(foo: &'a mut Option<&'a mut Foo>) -> &'a mut Foo {
    match foo {
        &mut Some(ref mut x) => *x,
        &mut None => panic!(),
    }
}
