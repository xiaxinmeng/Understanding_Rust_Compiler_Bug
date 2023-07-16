 rust
#[deriving(PartialEq, Show)]
struct Foo<T>(T);

fn hello() -> &'static str { "hello" }
fn foo() -> Foo<&'static str> { Foo(hello()) }
fn tup() -> (&'static str,) { (hello(),) }

fn main() {
    let a = "hello".to_string();
    // Works
    assert_eq!(Foo(a.as_slice()), Foo(hello()));
    // Works
    assert_eq!((a.as_slice(),), tup());
    // Fails to compile
    //assert_eq!(Foo(a.as_slice()), foo());
}
