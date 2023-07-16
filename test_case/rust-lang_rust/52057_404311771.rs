rust
// Delayed ICE:
// error: internal compiler error: unresolved inference variable in outlives: _#0t
#![feature(nll)]

pub trait Parser {
    type Input;

    #[inline(always)]
    fn parse_first(input: &mut Self::Input);
}

impl<'a, I, P: ?Sized> Parser for &'a mut P
where
    P: Parser<Input = I>,
{
    type Input = I;

    fn parse_first(_: &mut Self::Input) {}
}

fn main() {}
