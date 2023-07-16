 bash
---- sync::atomic::AtomicU8::new_0 stdout ----
    error: could not exec the linker `gcc`: The filename or extension is too long. (os error 206)
thread 'sync::atomic::AtomicU8::new_0' panicked at 'Box<Any>', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustc\session/mod.rs:171
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'sync::atomic::AtomicU8::new_0' panicked at 'couldn't compile the test', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustdoc\test.rs:285

---- sync::atomic::AtomicUsize::compare_exchange_0 stdout ----
    thread 'sync::atomic::AtomicUsize::compare_exchange_0' panicked at 'couldn't run the test: The filename or extension is too long. (os error 206)', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustdoc\test.rs:308

---- sync::atomic::AtomicUsize::compare_and_swap_0 stdout ----
    thread 'sync::atomic::AtomicUsize::compare_and_swap_0' panicked at 'couldn't run the test: The filename or extension is too long. (os error 206)', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustdoc\test.rs:308

---- unimplemented!_0 stdout ----
    error: could not exec the linker `gcc`: The filename or extension is too long. (os error 206)
thread 'unimplemented!_0' panicked at 'Box<Any>', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustc\session/mod.rs:171
thread 'unimplemented!_0' panicked at 'couldn't compile the test', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustdoc\test.rs:285


failures:
    sync::atomic::AtomicU8::new_0
    sync::atomic::AtomicUsize::compare_and_swap_0
    sync::atomic::AtomicUsize::compare_exchange_0
    unimplemented!_0

test result: FAILED. 944 passed; 4 failed; 9 ignored; 0 measured
