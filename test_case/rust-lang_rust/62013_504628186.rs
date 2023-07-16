rust
pub struct Demo {
    foo: Option<Box<Demo>>
}

pub fn simplified (mut a: &mut Demo){
    match &mut a.foo {
        Some(a_foo) => a = a_foo,
        None => ()
    }
    a.foo = None
}
