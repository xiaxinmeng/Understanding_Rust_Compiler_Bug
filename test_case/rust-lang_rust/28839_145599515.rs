 Rust
pub fn get_foo2<'a>(foo: &'a mut Option<&'a mut Foo>) -> &'a mut Foo {
    match foo {
        &mut Some(ref mut x) => match x { x => *x },
        &mut None => panic!(),
    }
}
