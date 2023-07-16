
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/libstd/path.rs:680:13: 680:14 error: unreachable pattern [E0001]
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/libstd/path.rs:680             _ => Some(Component::Normal(unsafe { u8_slice_as_os_str(comp) }))
                                                                                                   ^
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/libstd/path.rs:680:13: 680:14 help: pass `--explain E0001` to see a detailed explanation
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.std] Error 101
program finished with exit code 2
