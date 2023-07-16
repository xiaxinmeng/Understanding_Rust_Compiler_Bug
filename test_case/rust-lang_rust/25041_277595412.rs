rust
pub enum Error {
  Incomplete,
  Mismatch { message: String, position: usize },
  Conversion { message: String, position: usize },
  Custom { message: String, position: usize, inner: Option<Box<Error>> },
}
pub type Result<O> = ::std::result::Result<O, Error>;
pub trait Parser<'a, I, O> {
  fn parse(&self, input: &'a [I], start: usize) -> Result<(O, usize)>;
}

pub struct Left<P1, P2>(P1, P2);
impl<'a, I, O1, O2, P1: Parser<'a, I, O1>, P2: Parser<'a, I, O2>> Parser<'a, I, O1> for Left<P1, P2> {
  fn parse(&self, input: &'a [I], start: usize) -> Result<(O1, usize)> {
    self.0.parse(input, start).and_then(|(out1, pos1)|
      self.1.parse(input, pos1).map(|(_, pos2)|
        (out1, pos2)
      )
    )
  }
}
