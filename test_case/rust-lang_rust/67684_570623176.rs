rust
use std::marker::PhantomData;
trait StreamError<Item, Range>: Sized {}
trait ParseError<Item, Range>: Sized {
    type StreamError: StreamError<Item, Range>;
}
struct FastResult<T, E> {
    _marker: PhantomData<(T, E)>,
}
type ConsumedResult<O, I> = FastResult<O, <I as Stream>::Error>;
trait Stream {
    type Item;
    type Range;
    type Error: ParseError<Self::Item, Self::Range>;
}

enum Error<T, R> {
    Expected(PhantomData<(T, R)>),
}
impl<Item, Range> StreamError<Item, Range> for Error<Item, Range> {}
impl<Item, Range> ParseError<Item, Range> for Errors<Item, Range> {
    type StreamError = Error<Item, Range>;
}
type EasyParseError<S> = Errors<<S as Stream>::Item, <S as Stream>::Range>;
struct Errors<I, R> {
    _marker: PhantomData<(I, R)>,
}
fn token<I>(_c: I::Item) -> Token<I>
where
    I: Stream,
    I::Item: PartialEq,
{
    unimplemented!()
}
struct Token<I>
where
    I: Stream,
{
    _marker: PhantomData<I>,
}
impl<I> Parser for Token<I>
where
    I: Stream,
{
    type Input = I;
    type Output = I::Item;
    type PartialState = ();
}
struct AndThen<P, F>(P, F);
impl<P, F, O, E, I> Parser for AndThen<P, F>
where
    I: Stream,
    P: Parser<Input = I>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<<I::Error as ParseError<I::Item, I::Range>>::StreamError>,
    I::Error: ParseError<I::Item, I::Range>,
{
    type Input = P::Input;
    type Output = O;
    type PartialState = P::PartialState;
}
trait Parser {
    type Input: Stream;
    type Output;
    type PartialState: Default;
    fn parse_mode(
        &mut self,
        _input: &mut Self::Input,
        _state: &mut Self::PartialState,
    ) -> ConsumedResult<Self::Output, Self::Input>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn and_then<F, O, E, I>(self, _f: F) -> AndThen<Self, F>
    where
        Self: Parser<Input = I> + Sized,
        F: FnMut(Self::Output) -> Result<O, E>,
        I: Stream,
        E: Into<<I::Error as ParseError<I::Item, I::Range>>::StreamError>,
    {
        unimplemented!()
    }
}
fn expr<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char, Error = EasyParseError<I>>,
{
    let int2 = token('-').and_then(|_| Err(Error::Expected(PhantomData)));
    int2
}
struct Expr<I>
where
    <I as Stream>::Error: ParseError<<I as Stream>::Item, <I as Stream>::Range>,
    I: Stream<Item = char, Error = EasyParseError<I>>,
{
    _marker: std::marker::PhantomData<fn(I) -> ()>,
}
impl<I> Expr<I>
where
    <I as Stream>::Error: ParseError<<I as Stream>::Item, <I as Stream>::Range>,
    I: Stream<Item = char, Error = EasyParseError<I>>,
{
    #[allow(dead_code)]
    fn parse_mode_impl(&mut self, input: &mut I) -> ConsumedResult<(), I> {
        let Expr { .. } = *self;
        {
            let mut state = Default::default();
            expr().parse_mode(input, &mut state)
        }
    }
}

