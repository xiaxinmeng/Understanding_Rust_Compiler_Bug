rust
trait Foo<T> {
    fn foo();
}

trait Bound {}

impl<T: Bound> Foo<u32> for T {
    fn foo() {
        println!("u32 foo");
    }
}

impl<T: Bound> Foo<u64> for T {
    fn foo() {
        println!("u64 foo");
    }
}

fn main() {
    <() as Foo<_>>::foo();
}
