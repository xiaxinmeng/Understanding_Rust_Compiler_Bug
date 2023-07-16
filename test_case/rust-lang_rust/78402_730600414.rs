
fn bar<T>(_: T) -> impl Stream<Item = Result<Vec<u8>, io::Error>> + 'static {
