
impl<R: Reader> Reader for LimitReader<R> {
    type Error = Option<R::Error>;
    ...
}
