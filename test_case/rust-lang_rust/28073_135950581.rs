 rust
pub struct Bytes<R> {
    inner: BytesInner<R>,
}

enum BytesInner<R> {
    Raw(R),
    Buffered(BufReader<R>),
}
