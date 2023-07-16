rust
pub fn handle_alloc_error(layout: Layout) -> ! {
    let hook = HOOK.load(Ordering::SeqCst);
    if !hook.is_null() {
        let hook: fn(Layout) = unsafe { mem::transmute(hook) }
        hook(layout);
    }
    intrinsics::abort()
}

static HOOK: AtomicPtr<()> = …;
