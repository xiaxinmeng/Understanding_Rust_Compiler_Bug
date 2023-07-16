rust
use combine::parser::char::spaces;
use combine::Parser;
use combine::{easy, error, stream};

fn mk_type_parser<'a>() -> impl Parser<easy::Stream<&'a str>, Output = ()> {
    mk_inner_type_parser().skip(spaces())
}

struct TypeParser<'a>(&'a ());
fn mk_inner_type_parser<'a>() -> TypeParser<'a> {
    unimplemented!()
}

impl<'a> Parser<easy::Stream<&'a str>> for TypeParser<'a>
where
    easy::Stream<&'a str>: stream::Stream,
{
    type Output = ();
    type PartialState = ();
    fn add_committed_expected_error(
        &mut self,
        _errors: &mut error::Tracked<<easy::Stream<&'a str> as stream::StreamOnce>::Error>,
    ) {
        let mut parser = mk_type_parser();
        let _: &mut dyn Parser<_, Output = _, PartialState = _> = &mut parser;
    }
}
