 rust
trait IoResultIterator {
    fn fail_on_error(self) -> FailOnErrorIterator<Self> {
       FailOnErrorIterator {
           wrapped: self
       }
    }
}

impl <T, I: Iterator<IoResult<T>>> IoResultIterator for I {}

struct FailOnErrorIterator<I> {
    wrapped: I
}

impl <T, I: Iterator<IoResult<T>>> Iterator<T> for FailOnErrorIterator<I> {
    fn next(&mut self) -> Option<T> {
       match self.wrapped.next() {
           Some(x) => match x {
               Ok(y) => Some(y),
               Err(io::IoError { kind: io::EndOfFile, .. }) => None,
               _ => fail!()
           },
           None => None
       }
    }
}
