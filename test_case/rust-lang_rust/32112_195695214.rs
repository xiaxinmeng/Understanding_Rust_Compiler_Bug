
../src/libstd\sys/unix\net.rs:24:1: 24:31 warning: `pub extern crate` does not work as expected and should not be used. Likely to become an error. Prefer `extern crate` and `pub use`.
../src/libstd\sys/unix\net.rs:24 pub extern crate libc as netc;
                                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
../src/libstd\sys/windows\handle.rs:138:48: 138:56 error: unresolved name `c::FALSE` [E0425]
../src/libstd\sys/windows\handle.rs:138             let wait = if wait {c::TRUE} else {c::FALSE};
                                                                                       ^~~~~~~~
../src/libstd\sys/windows\handle.rs:138:48: 138:56 help: run `rustc --explain E0425` to see a detailed explanation
../src/libstd\sys/windows\pipe.rs:143:57: 143:65 error: unresolved name `c::FALSE` [E0425]
../src/libstd\sys/windows\pipe.rs:143             c::WaitForMultipleObjects(2, objs.as_ptr(), c::FALSE, c::INFINITE)
                                                                                              ^~~~~~~~
../src/libstd\sys/windows\pipe.rs:143:57: 143:65 help: run `rustc --explain E0425` to see a detailed explanation
