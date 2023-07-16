rust
pub struct Foo<'a, 'b> {
    r1: &'a mut u32,
    r2: &'b mut u32,
}

pub fn use_foo(foo: &mut Foo) -> u32 {
    let r1 = &mut *foo.r1;
    *r1 = 1;
    *foo.r2 = 2;
    return *r1;
}
