
test result: ok. 15 passed; 0 failed; 3 ignored; 0 measured

rustc: x86_64-unknown-linux-gnu/stage2/test/libctest-arm-linux-androideabi
run: x86_64-unknown-linux-gnu/stage2/test/libctest-arm-linux-androideabi via adb
1597 KB/s (1348596 bytes in 0.824s)

running 1 test
test work_on_windows ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

0 KB/s (19 bytes in 0.078s)
rustc: x86_64-unknown-linux-gnu/stage2/test/stdtest-arm-linux-androideabi
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/process.rs:1077:17: 1077:26 error: failed to resolve. Use of undeclared type or module `env`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/process.rs:1077         let r = env::vars();
                                                                                                                          ^~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/process.rs:1077:17: 1077:26 error: unresolved name `env::vars`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/process.rs:1077         let r = env::vars();
                                                                                                                          ^~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/process.rs:841:17: 841:24 error: unresolved name `os::env`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/process.rs:841         let r = os::env();
                                                                                                                  ^~~~~~~
error: aborting due to 3 previous errors
make: *** [x86_64-unknown-linux-gnu/stage2/test/stdtest-arm-linux-androideabi] Error 101
