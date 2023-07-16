 rust
pub struct RefReader<'a, R>(&'a mut R);

impl <'a, R: Reader> Reader for RefReader<'a, R> { ... }
