
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:337:9: 338:79 error: foreign function is never used: `pthread_getattr_np`, #[deny(dead_code)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:337     pub fn pthread_getattr_np(native: libc::pthread_t,
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:338                               attr: *mut libc::pthread_attr_t) -> libc::c_int;
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:339:9: 340:83 error: foreign function is never used: `pthread_attr_getguardsize`, #[deny(dead_code)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:339     pub fn pthread_attr_getguardsize(attr: *const libc::pthread_attr_t,
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:340                                      guardsize: *mut libc::size_t) -> libc::c_int;
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:341:9: 343:79 error: foreign function is never used: `pthread_attr_getstack`, #[deny(dead_code)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:341     pub fn pthread_attr_getstack(attr: *const libc::pthread_attr_t,
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:342                                  stackaddr: *mut *mut libc::c_void,
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/thread.rs:343                                  stacksize: *mut libc::size_t) -> libc::c_int;
error: aborting due to 3 previous errors
