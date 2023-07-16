
error: Undefined Behavior: StorageLive on a local that was already live
    --> /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1058:30
     |
1058 |                     let Some(id) = last.checked_add(1) else {
     |                              ^^ StorageLive on a local that was already live
     |
     = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
     = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
     = note: BACKTRACE:
     = note: inside `std::thread::ThreadId::new` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1058:30
     = note: inside `std::thread::Thread::new` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1159:43
     = note: inside `std::rt::init` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/rt.rs:104:22
     = note: inside closure at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/rt.rs:147:42
     = note: inside `std::panicking::try::do_call::<[closure@std::rt::lang_start_internal::{closure#1}], ()>` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/panicking.rs:492:40
     = note: inside `std::panicking::try::<(), [closure@std::rt::lang_start_internal::{closure#1}]>` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/panicking.rs:456:19
     = note: inside `std::panic::catch_unwind::<[closure@std::rt::lang_start_internal::{closure#1}], ()>` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
     = note: inside `std::rt::lang_start_internal` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/rt.rs:147:5
     = note: inside `std::rt::lang_start::<()>` at /home/ben/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/rt.rs:165:17

error: aborting due to previous error
