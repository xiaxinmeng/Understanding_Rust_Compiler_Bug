
                                                                                                         ^
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/os.rs:567:9: 567:28 error: use of deprecated item: use CStr::from_ptr(p).to_bytes() instead, #[deny(deprecated)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/libstd/os.rs:567         ffi::c_str_to_bytes(&*argv.offset(i as int)).to_vec()
                                                                                               ^~~~~~~~~~~~~~~~~~~
