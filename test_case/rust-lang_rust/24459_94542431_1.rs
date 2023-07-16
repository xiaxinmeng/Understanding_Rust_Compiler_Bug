 rust
fn request<P, Q>(rx: Receiver<Chan<P>>) -> Chan<Q> {
    let Chan(tx, rx, _) = rx.recv().unwrap();
    Chan(tx, rx, PhantomData)
}
