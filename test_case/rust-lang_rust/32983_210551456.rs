 rust
trait A {
    type B;
}

impl A for () {
    type B = u32;
}

struct D<B>(::std::marker::PhantomData<B>);

impl<B> D<B> {
    fn new() -> D<B> {
        D(::std::marker::PhantomData)
    }
}

type U = D<<() as A>::B>;

fn main() {
    let _ = U::new();
}
