rust
    let null_hook: Box<PanicHook> = Box::new(|_panic_info| { /* ignore */ });
    let sanity_check = &*null_hook as *const PanicHook;
    let original_hook = panic::take_hook();
    panic::set_hook(null_hook);

    let works = panic::catch_unwind(proc_macro::Span::call_site).is_ok();
    WORKS.store(works as usize + 1, Ordering::Relaxed);

    let hopefully_null_hook = panic::take_hook();
    panic::set_hook(original_hook);
    if sanity_check != &*hopefully_null_hook {
        panic!("observed race condition in proc_macro2::inside_proc_macro");
    }
