rust
pub struct BufferedStream<S> {
    inner: BufferedReader<InternalBufferedWriter<S>>
}
