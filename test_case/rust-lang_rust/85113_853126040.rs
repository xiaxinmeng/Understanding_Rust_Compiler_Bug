rust
#![feature(unboxed_closures)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

type Fun = impl Fn<()>;
static BBB: Fun = bbb;
type Out = <Fun as FnOnce<()>>::Output;

fn bbb() {}

fn main() {
    let bar: Out = BBB();
}
