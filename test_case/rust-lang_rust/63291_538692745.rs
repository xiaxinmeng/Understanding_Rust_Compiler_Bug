rust
pub fn new_uninit() -> Box<mem::MaybeUninit<T>> {
    let layout = alloc::Layout::new::<mem::MaybeUninit<T>>();
    let ptr = unsafe {
        Global.alloc(layout)
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout))
    };
    Box(ptr.cast().into())
}
