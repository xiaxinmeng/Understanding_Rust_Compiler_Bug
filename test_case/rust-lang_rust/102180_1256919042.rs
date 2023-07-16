plain
+++ <stderr output>
... 8 lines skipped ...
    = note: BACKTRACE:
    = note: inside `std::sys::PLATFORM::thread::Thread::join` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
~   = note: inside `std::thread::JoinInner::<'_, ()>::join` at RUSTLIB/std/src/thread/mod.rs:LL:CC
    = note: inside `std::thread::JoinHandle::<()>::join` at RUSTLIB/std/src/thread/mod.rs:LL:CC
... 10 lines skipped ...


full stderr:
full stderr:
error: Undefined Behavior: trying to join a detached thread
  --> D:\a\rust\rust\library\std\src\sys\windows\thread.rs:73:27
   |
LL |         let rc = unsafe { c::WaitForSingleObject(self.handle.as_raw_handle(), c::INFINITE) };
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::sys::windows::thread::Thread::join` at D:\a\rust\rust\library\std\src\sys\windows\thread.rs:73:27
   = note: inside `std::thread::JoinInner::<'_, ()>::join` at D:\a\rust\rust\library\std\src\thread\mod.rs:1390:9
   = note: inside `std::thread::JoinHandle::<()>::join` at D:\a\rust\rust\library\std\src\thread\mod.rs:1523:9
note: inside `main` at tests/fail\concurrency\windows_join_detached.rs:20:5
  --> tests/fail\concurrency\windows_join_detached.rs:20:5
   |
LL |     thread.join().unwrap();

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
