
--- tests/fail\concurrency\windows_join_detached.stderr
+++ <stderr output>
... 8 lines skipped ...
    = note: BACKTRACE:
    = note: inside `std::sys::PLATFORM::thread::Thread::join` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
~   = note: inside `std::thread::JoinInner::<'_, ()>::join` at RUSTLIB/std/src/thread/mod.rs:LL:CC
    = note: inside `std::thread::JoinHandle::<()>::join` at RUSTLIB/std/src/thread/mod.rs:LL:CC
 note: inside `main` at $DIR/windows_join_detached.rs:LL:CC
... 10 lines skipped ...
