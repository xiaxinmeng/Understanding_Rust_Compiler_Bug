
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/os.rs:374:35: 374:49 error: mismatched types: expected `*const u8`, found `*const i8` (expected u8, found i8)
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/os.rs:374                 Some(CString::new(s as *const i8, false).as_bytes_no_nul().to_vec())
                                                                                                                               ^~~~~~~~~~~~~~
