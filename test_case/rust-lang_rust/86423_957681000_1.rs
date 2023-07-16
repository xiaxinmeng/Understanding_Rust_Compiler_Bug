rust
trait BufReadExt: BufRead2 {
    fn read_until(&mut self, delimiter: u8, buf: &mut Vec<u8>) -> IoResult<Option<NonZeroUsize>>;
    fn read_line(&mut self, buf: &mut String) -> IoResult<Option<NonZeroUsize>>;
    fn split(self, delimiter: u8) -> Split<Self>
    where
        Self: Sized;
    fn lines(self) -> Lines<Self>
    where
        Self: Sized;
