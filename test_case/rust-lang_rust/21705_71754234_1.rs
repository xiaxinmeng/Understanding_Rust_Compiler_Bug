
impl <I, O, P> Parser for Box<P>
  where I: Stream, P: Parser<Input=I, Output=O> {
  type Input = I;
  type Output = O;
  fn parse_state(&mut self, input: State<I>) -> ParseResult<O, I> {
    (*self).parse_state(input)
  }
}
