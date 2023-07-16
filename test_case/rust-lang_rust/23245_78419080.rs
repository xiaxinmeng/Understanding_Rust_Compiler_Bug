
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/backtrace.rs:87:5: 87:26 error: unused import, #[deny(unused_imports)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/backtrace.rs:87 use os::unix::prelude::*;
                                                                                                          ^~~~~~~~~~~~~~~~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/backtrace.rs:89:17: 89:24 error: unused import, #[deny(unused_imports)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/backtrace.rs:89 use ffi::{CStr, AsOsStr};
                                                                                                                      ^~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:61:13: 65:14 error: unnecessary `unsafe` block, #[deny(unused_unsafe)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:61             unsafe {
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:62                 let info = info();
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:63                 let diff = self.t as i64 - other.t as i64;
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:64                 Duration::nanoseconds(diff * info.numer as i64 / info.denom as i64)
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/sys/unix/time.rs:65             }
error: aborting due to 3 previous errors
