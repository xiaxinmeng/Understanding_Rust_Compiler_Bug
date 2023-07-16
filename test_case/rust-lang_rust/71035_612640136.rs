
fn create_stream() -> impl futures::TryStream<Ok=(), Error=mpsc::SendError,
Item=Result<(), mpsc::SendError>> {
    let (_, rx) = mpsc::unbounded();
    rx
}
