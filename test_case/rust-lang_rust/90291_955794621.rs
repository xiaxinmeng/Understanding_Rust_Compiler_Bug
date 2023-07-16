rust
struct Shared {
    ptr: AtomicPtr<_>,
    index: AtomicUsize,
}

// the recv side has the owned copy, and would have a custom Debug implementation
pub struct RecvSide {
    shared: Arc<Shared>,
}

// the send side I was hoping could be simplified by using #[derive]
#[derive(Debug)]
pub struct SendSide {
    shared: Weak<Shared>,
}
