rust
pub fn handle_alloc_error(layout: Layout) -> ! {
    let hook = HOOK.load(Ordering::SeqCst);
    if hook.is_null() {
        // No-op when std is not linked
        __rust_maybe_default_alloc_error_hook(layout.size(), layout.align())
    } else {
        let hook: fn(Layout) = unsafe { mem::transmute(hook) }
        hook(layout);
    }
    // No-op when std is not linked
    __rust_maybe_abort_internal();
    intrinsics::abort()
}

static HOOK: AtomicPtr<()> = …;
