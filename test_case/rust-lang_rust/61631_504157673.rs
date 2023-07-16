Rust
use core::marker::{
    PhantomData,
    PhantomPinned,
};

// !Send + !Sync + !Unpin.
// lacks Copy + Clone
struct NoAutoTrait<T> {
    _mark: PhantomData<T>,
    nosend: PhantomData<*mut ()>,
    pinned: PhantomPinned,
}

struct Bug<'a, F: Copy + Send + Sync + Unpin = NoAutoTrait<&'a String>> {
    mark: PhantomData<&'a F>
}
