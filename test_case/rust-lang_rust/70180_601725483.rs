rust
#![feature(const_generics)]

fn works() {
    let array/*: [u8; _]*/ = default_byte_array();
    let _: [_; 4] = array;
    Foo::foo(&array);
}

fn doesnt_work() {
    let array/*: [u8; _]*/ = default_byte_array();
    Foo::foo(&array);
    // ^ cannot infer type for type parameter `T` declared on the trait `Foo`
    let _: [_; 4] = array;
}

trait Foo<T> {
    fn foo(&self) {}
}

impl Foo<i32> for [u8; 4] {}
impl Foo<i64> for [u8; 8] {}

// Only needed because `[u8; _]` is not valid type syntax.
fn default_byte_array<const N: usize>() -> [u8; N]
    where [u8; N]: Default,
{
    Default::default()
}
