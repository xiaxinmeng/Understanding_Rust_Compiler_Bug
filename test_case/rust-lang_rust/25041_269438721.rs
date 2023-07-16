rust
impl<I, O, U> Add<Parser<I, U>> for Parser<I, O> {
    type Output = Parser<I, (O, U)>;

    fn add(self, other: Parser<I, U>) -> Self::Output
        where I: 'static,
              O: 'static,
              U: 'static
    {
        Parser::new(move |input: &mut Input<I>| {
            self.parse(input).and_then(|out1| other.parse(input).map(|out2| (out1, out2)))
        })
    }
}
