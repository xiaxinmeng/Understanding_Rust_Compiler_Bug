Rust
#[lang = "start"]
fn lang_start<T: Termination>
    (main: fn() -> T, argc: isize, argv: *const *const u8) -> !
{
    use panic;
    use sys;
    use sys_common;
    use sys_common::thread_info;
    use thread::Thread;

    sys::init();

    sys::process::exit(unsafe {
        let main_guard = sys::thread::guard::init();
        sys::stack_overflow::init();

        // Next, set up the current Thread with the guard information we just
        // created. Note that this isn't necessary in general for new threads,
        // but we just do this to name the main thread and to give it correct
        // info about the stack bounds.
        let thread = Thread::new(Some("main".to_owned()));
        thread_info::set(main_guard, thread);

        // Store our args if necessary in a squirreled away location
        sys::args::init(argc, argv);

        // Let's run some code!
        let exitcode = panic::catch_unwind(|| main().report())
            .unwrap_or(101);

        sys_common::cleanup();
        exitcode
    });
}
