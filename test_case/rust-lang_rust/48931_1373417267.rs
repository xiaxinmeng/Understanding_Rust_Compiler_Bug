rust
#[repr(transparent)]
struct UcWrap<T>(UnsafeCell<T>);

unsafe impl<T> Send for UcWrap<T> {}
unsafe impl<T> Sync for UcWrap<T> {}

static VAR: UcWrap<...> = ...
