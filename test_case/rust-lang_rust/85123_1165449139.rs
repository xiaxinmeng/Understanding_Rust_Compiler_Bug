rust
impl<'a> Line<'a> {
    pub(crate) fn parse_css<'b>(input: &mut Parser<'a, 'b>) -> Result<Line<'a>, ParseError<'a, ()>> {
        let first = input.next()?;
        let second = input.try_parse(|input| input.next());
        match (first, second) {
            // ... omitted for brevity ...
            (Token::Ident(id), Err(_)) => Ok(Line::Named(&**id)),  // *** lifetime error is here
            _ => Err(input.new_custom_error(())),
        }
    }
}
