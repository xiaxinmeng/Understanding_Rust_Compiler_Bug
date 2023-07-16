rust
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

struct NewT<T>(T);

trait Foo {
    const W: usize;
    fn bar(v: [(); Self::W]);
}

impl<X: Foo> Foo for NewT<X>
{
    const W: usize = X::W;
    fn bar(v: [(); Self::W]) {
        X::bar(v);
    }
}
