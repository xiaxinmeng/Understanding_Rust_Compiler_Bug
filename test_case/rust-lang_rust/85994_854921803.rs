rust
#![feature(const_generics)]

struct Value<const B: bool> {}
impl Value<true> { fn is_true() {} }

fn f<T>() {
    Value::<{std::mem::needs_drop::<T>()}>::is_true();
}
fn main() {
    f::<u32>();
}
