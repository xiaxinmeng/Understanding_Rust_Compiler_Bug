rust
trait Foo {
    fn bar(self);
}

impl Foo for *const [u8] {
    fn bar(self) {}
}

pub struct Test {
    data: [u8],
}

pub fn test_len(t: *const Test) -> usize {
    unsafe { (*t).data.bar() }
}
