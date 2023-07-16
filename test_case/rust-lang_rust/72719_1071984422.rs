
    // Some older versions of LLDB seem to have problems with multiple
    // instances running in parallel, so only run one test thread at a
    // time.
    env::set_var("RUST_TEST_THREADS", "1");
