
#![feature(use_extern_macros)]
trait Foo {}
#[derive(Foo::Anything)]
struct S;
