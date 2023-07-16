rust
use std::marker::PhantomData;

trait ParseError<Item, Range> {
    type StreamError;
}

trait Stream {
    type Item;
    type Range;
    type Error: ParseError<Self::Item, Self::Range>;
}

impl<Item, Range> ParseError<Item, Range> for Errors<Item, Range> {
    type StreamError = ();
}

type EasyParseError<S> = Errors<<S as Stream>::Item, <S as Stream>::Range>;

struct Errors<I, R> {
    _marker: PhantomData<(I, R)>,
}

impl<I> Parser for PhantomData<I> {
    type Input = I;
    type Output = I;
    type PartialState = ();
}

struct AndThen<P, I, O, E>(P, I, O, E);

impl<P, O, E, I> Parser for AndThen<P, I, O, E>
where
    I: Stream,
    P: Parser,
    E: Into<<I::Error as ParseError<I::Item, I::Range>>::StreamError>,
{
    type Input = P::Input;
    type Output = ();
    type PartialState = ();
}

trait Parser {
    type Input;
    type Output;
    type PartialState: Default;
    fn parse_mode(self, _: Self::Input, _: Self::PartialState)
    where
        Self: Sized,
    {
        loop {}
    }
}

struct Expr<I>
where
    I: Stream<Error = EasyParseError<I>>,
{
    _marker: std::marker::PhantomData<fn(I) -> ()>,
}

impl<I> Expr<I>
where
    <I as Stream>::Error: ParseError<<I as Stream>::Item, <I as Stream>::Range>,
    I: Stream<Error = EasyParseError<I>>,
{
    #[allow(dead_code)]
    fn parse_mode_impl(self, input: I) {
        fn expr<I>() -> impl Parser<Input = I, Output = ()>
        where
            I: Stream<Error = EasyParseError<I>>,
        {
            (loop {}) as AndThen<PhantomData<I>, I, (), ()>
        }

        expr().parse_mode(input, Default::default())
    }
}
