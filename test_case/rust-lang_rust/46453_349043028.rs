
struct Foo(u64);

impl From<()> for Foo {
     fn from(_: ()) -> Self {
       Foo (0xdeadbeef)
     }
}
