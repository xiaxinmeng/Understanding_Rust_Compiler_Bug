
use futures::channel::mpsc;
use futures::stream::StreamExt;

//fn create_stream() -> impl futures::Stream<Item=Result<(), mpsc::SendError>> { // WORKS
fn create_stream() -> impl futures::TryStream<Ok=(), Error=mpsc::SendError> { // ERROR
    let (_, rx) = mpsc::unbounded();
    rx
}

fn main() {
    let rx = create_stream();
    let (tx, _) = mpsc::unbounded();
    let foo = rx.forward(tx);
    let _ = futures::executor::block_on(foo);
}
