rust
use combine::parser::range::take_while1;
use combine::stream::RangeStream;
use combine::{choice, Parser, StreamOnce};

pub fn metric_stream_parser<'a, I, F>(
) -> impl Parser<I, Output = (), PartialState = impl Default + 'a>
where
    I: 'a + StreamOnce<Token = u8, Range = &'a [u8], Position = usize> + RangeStream,
{
    let val = take_while1(|_: u8| false);
    let sampling = take_while1(|_: u8| false);
    let metric = (val, choice((sampling,))).map(|_| todo!());

    choice(((metric).map(|_: F| ()),))
}
