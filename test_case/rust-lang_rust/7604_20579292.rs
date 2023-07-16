
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt/build/src/libstd/iterator.rs:1090:4: 1092:5 error: method `size_hint` has an incompatible type: expected uint but found enum option::Option
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt/build/src/libstd/iterator.rs:1090     fn size_hint(&self) -> (Option<uint>, Option<uint>) {
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt/build/src/libstd/iterator.rs:1091         self.iter.size_hint()
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt/build/src/libstd/iterator.rs:1092     }
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd.so] Error 101
