
13 | impl<'a, I, O1, O2, P1: Parser<'a, I, O1>, P2: Parser<'a, I, O2>> Parser<'a, I, O1> for Left<P1, P2> {
   |                 ^^ unconstrained type parameter
