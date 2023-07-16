plain
status: exit status: 0
child output:
    0: std::backtrace::Backtrace::create
    1: std::backtrace::Backtrace::force_capture
    2: backtrace_apple_no_dsymutil::main
    4: std::rt::lang_start::{{closure}}
    5: std::rt::lang_start_internal
    6: _main
------------------------------------------
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'failed to find "/Users/runner/work/rust/rust/src/test/ui/backtrace-apple-no-dsymutil.rs" in output', /Users/runner/work/rust/rust/src/test/ui/backtrace-apple-no-dsymutil.rs:29:5
------------------------------------------



