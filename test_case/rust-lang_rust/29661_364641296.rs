rust
trait Future {
    type Item;
    type Error;
                          // |  This is long and annoying to write and read
                          // V
    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
}
