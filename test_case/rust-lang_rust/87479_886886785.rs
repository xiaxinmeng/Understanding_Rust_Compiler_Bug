rust
trait Parser {
    type Output<'a>;
    fn parse<'a>(&mut self, data: &'a [u8]) -> Self::Output<'a>;
}
