 rust
struct Chan<P> = (Sender<Box<u8>>, Receiver<Box<u8>>, PhantomData<P>)

fn request<P, Q>(rx: Receiver<Chan<P>) -> Chan<Q> {
    unsafe { transmute(rx.recv().unwrap()) }
}
