
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:83:9: 88:10 error: unnecessary `unsafe` block, #[deny(unused_unsafe)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:83         unsafe {
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:84             boxed::into_raw(box Node {
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:85                 value: None,
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:86                 next: AtomicPtr::new(ptr::null_mut::<Node<T>>()),
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:87             })
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sync/mpsc/spsc_queue.rs:88         }
error: aborting due to previous error
