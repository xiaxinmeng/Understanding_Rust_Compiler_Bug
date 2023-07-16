rust
// type HelloWorldStream: Stream<Item = Response, Error = ()>;
type HelloWorldStream: Stream<Item = Response, Error = ()> + Send + Sync;
