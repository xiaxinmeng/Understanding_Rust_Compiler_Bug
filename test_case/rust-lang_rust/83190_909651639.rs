rust
use std::marker::PhantomData;
pub trait StreamError: Sized {}
pub trait ParseError: Sized + PartialEq {
    type StreamError: StreamError;
}
pub trait StreamOnce {
    type Token: Clone;
    type Range: Clone;
    type Error: ParseError;
}
pub trait Stream: StreamOnce {}
pub struct Map<P, F>(P, F);
impl<Input, A, B, P, F> Parser<Input> for Map<P, F>
where
    Input: Stream,
    P: Parser<Input, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;
    type PartialState = P::PartialState;
}
struct TakeWhile1<Input, F>(F, PhantomData<fn(Input) -> Input>);
impl<Input, F> Parser<Input> for TakeWhile1<Input, F>
where
    Input: Stream,
    F: FnMut(Input::Token) -> bool,
{
    type Output = Input::Range;
    type PartialState = usize;
}
fn take_while1<Input, F>(_f: F) -> TakeWhile1<Input, F>
where
    Input: Stream,
    F: FnMut(Input::Token) -> bool,
{
    unimplemented!()
}
pub struct SequenceState<T, U>(PhantomData<(T, U)>);
impl<T, U: Default> Default for SequenceState<T, U> {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(Default)]
pub struct PartialState1<A> {
    _marker: PhantomData<(A,)>,
}
impl<Input: Stream, A> Parser<Input> for (A,)
where
    Input: Stream,
    Input::Error: ParseError,
    A: Parser<Input>,
{
    type Output = (A::Output,);
    type PartialState = PartialState1<SequenceState<A::Output, A::PartialState>>;
}
pub trait Parser<Input: Stream> {
    type Output;
    type PartialState: Default;
    fn map<F, B>(self, _f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        unimplemented!()
    }
}
pub fn metric_stream_parser<'a, I>() -> impl Parser<I, Output = (), PartialState = impl Default + 'a>
where
    I: 'a + StreamOnce<Token = u8, Range = &'a [u8]> + Stream,
{
    let val = take_while1(|_: u8| false);
    let metric = (val,).map(|_| todo!());
    metric
}
