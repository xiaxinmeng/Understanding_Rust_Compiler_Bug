rust
struct Test;

enum Foo {
    #[inline]
   //~^ ERROR attribute should be applied
    Variant,
}

fn main() {}
