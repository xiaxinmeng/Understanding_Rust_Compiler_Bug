
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/os.rs:1708:17: 1708:34 error: unresolved import `os::unix::AsRawFd`. There is no `AsRawFd` in `sys::ext`
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/os.rs:1708             use os::unix::AsRawFd;
                                                                                                     ^~~~~~~~~~~~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/process.rs:576:13: 576:36 error: unresolved import `os::unix::ExitStatusExt`. There is no `ExitStatusExt` in `sys::ext`
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/process.rs:576         use os::unix::ExitStatusExt;
                                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
make: *** [x86_64-apple-darwin/stage2/test/stdtest-x86_64-apple-darwin] Error 101
