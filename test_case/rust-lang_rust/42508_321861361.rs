rust
pub fn parse_nested_block<'i: 't, 't, F, T, E>(parser: &mut Parser<'i, 't>, parse: F)
                                               -> Result <T, ParseError<'i, E>>
    where F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>> {
