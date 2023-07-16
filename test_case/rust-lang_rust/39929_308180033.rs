rust
#![feature(conservative_impl_trait)]
fn impl_trait_with_ref<T: ?Sized>(v: &T) -> impl Iterator<Item=&T> {
    std::iter::once(v)
}
fn main() {
    for s in impl_trait_with_ref("foobar") {
        println!("{}", s);
    }
}
