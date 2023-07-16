 rust
trait Foo {
    fn foo() -> Self;
}

impl Foo for f32 {
    fn foo() -> f32 {
        42.0
    }
}

impl Foo for f64 {
    fn foo() -> f64 {
        42.0
    }
}

fn bar<T: Foo=f64>() -> T { // “=f64” has no effect.
    T::foo()
}

fn main() {
    let _: f64 = bar(); // The type annotation is still needed.
}
