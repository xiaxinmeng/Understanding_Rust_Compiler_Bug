rust
struct Test;

enum Foo {
    #[repr(u8)]
    //[before]~^ WARN unused attribute
    //[after]~^^ ERROR attribute should be applied
    Variant,
}

#[repr(u8)] //~ ERROR attribute should be applied
fn main() {}
