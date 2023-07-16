rust
struct Foo(u32);

pub trait Bar {
    fn bar(self);
}

impl Foo {
    pub fn map<F>(f: F) where F: Bar {
        f.bar();
    }
}
