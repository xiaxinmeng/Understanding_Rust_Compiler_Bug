rust
fn assert_stream_send<'u, R>(
	strm: impl 'u + Send + Stream<Item = R>,
) -> impl 'u + Send + Stream<Item = R> {
	strm
}
