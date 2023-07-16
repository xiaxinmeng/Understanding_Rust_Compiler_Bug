rust
use std::ops::Add;

pub struct Input<'a, I: 'a> {
  pub data: &'a [I],
  pub position: usize,
}

impl<'a, I: 'a> Input<'a, I> {
  pub fn new(input: &'a [I]) -> Input<I> {
    Input {
      data: input,
      position: 0,
    }
  }

  pub fn current(&self) -> Option<I>
    where I: Copy + Clone + 'static
  {
    if self.position < self.data.len() {
      Some(self.data[self.position])
    } else {
      None
    }
  }

  pub fn advance(&mut self) {
    self.position += 1;
  }
}

#[derive(Debug, PartialEq)]
pub enum Error {
  Incomplete,
  Mismatch { message: String, position: usize },
}

pub type Result<O> = ::std::result::Result<O, Error>;

pub struct Parser<I, O> {
  method: Box<Fn(&mut Input<I>) -> Result<O>>,
}

impl<I, O> Parser<I, O> {
  pub fn new<P>(parse: P) -> Parser<I, O>
    where P: Fn(&mut Input<I>) -> Result<O> + 'static
  {
    Parser { method: Box::new(parse) }
  }

  pub fn parse(&self, input: &mut Input<I>) -> Result<O> {
    (self.method)(input)
  }
}

impl<I, O1, O2> Add for Parser<I, O1> {
  type Output = Parser<I, (O1, O2)>;

  fn add(self, other: Parser<I, O2>) -> Self::Output
    where I: 'static,
          O1: 'static,
          O2: 'static
  {
    Parser::new(move |input: &mut Input<I>| {
      self.parse(input).and_then(|out1| other.parse(input).map(|out2| (out1, out2)))
    })
  }
}
