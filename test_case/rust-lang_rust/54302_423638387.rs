Rust
trait Foo {
    fn foo(self) -> &'static u32;
}

impl<'a> Foo for &'a u32 where 'a: 'static {
    fn foo(self) -> &'static u32 { self }
}

trait RefFoo {
    fn ref_foo(&self) -> &'static u32;
}

impl<T> RefFoo for T where for<'a> &'a T: Foo {
    fn ref_foo(&self) -> &'static u32 {
        self.foo()
    }
}

fn coerce_lifetime(a: &u32) -> &'static u32
{
    <u32 as RefFoo>::ref_foo(a)
}

fn main() {
}
