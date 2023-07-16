
    fn starts_with<I>(mut self, other: I) -> bool
    where I: Iterator,
    Self::Item: PartialEq<I::Item>,
    Self: Sized
