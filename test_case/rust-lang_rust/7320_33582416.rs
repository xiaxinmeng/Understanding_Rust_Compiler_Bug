
trait Foo: Send {
    fn foo(~self) { bar(self as ~Foo); }
}

fn bar(_b: ~Foo) { }

fn main() {}
