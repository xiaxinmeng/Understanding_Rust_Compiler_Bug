rust
fn delimited_by<U, V, L, R>(
    self,
    start: L,
    end: R
) -> DelimitedBy<Self, L, R, U, V>
where
    Self: Sized,
    L: Parser<I, U, Error = Self::Error>,
    R: Parser<I, V, Error = Self::Error>, 
