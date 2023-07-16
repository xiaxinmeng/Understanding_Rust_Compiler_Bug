rust
    // Could also use a thread local to silence panics only in this thread
    static SILENCE_PANICS: AtomicBool = AtomicBool::new(true);

    panic::update_hook(|prev, info| {
        if !SILENCE_PANICS.load(Ordering::SeqCst) {
            prev(info);
        }
    });

    let works = panic::catch_unwind(proc_macro::Span::call_site).is_ok();
    WORKS.store(works as usize + 1, Ordering::Relaxed);

    SILENCE_PANICS.store(false, Ordering::SeqCst);
