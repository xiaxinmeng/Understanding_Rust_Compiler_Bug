rust
trait Parser2 {
    type Input;
    type PartialState;
}

struct Test<I>(::std::marker::PhantomData<fn(I)>);

impl<I> Parser2 for Test<I> {
    type Input = I;
    type PartialState = ();
}

fn line<'a, I>() -> impl Parser2<Input = I, PartialState = impl Send + 'static> {
    Test(::std::marker::PhantomData)
}

fn status<'a, I>() -> impl Parser2<Input = I, PartialState = impl Send + 'static> {
    line()
}

fn main() {
}
