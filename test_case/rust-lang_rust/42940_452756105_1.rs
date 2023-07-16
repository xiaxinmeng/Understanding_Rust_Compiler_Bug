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

trait Static: Send + 'static {}
impl<T> Static for T where T: Send + 'static {}

fn line<'a, I>() -> impl Parser2<Input = I, PartialState = impl Static> {
    Test(::std::marker::PhantomData)
}

fn status<'a, I>() -> impl Parser2<Input = I, PartialState = impl Static> {
    line()
}
