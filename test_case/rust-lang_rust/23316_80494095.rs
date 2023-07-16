
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/sys/unix/os.rs:256:37: 256:62 error: mismatched types:
 expected `ffi::os_str::OsString`,
    found `&_`
(expected struct `ffi::os_str::OsString`,
    found &-ptr) [E0308]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/libstd/sys/unix/os.rs:256         Ok(PathBuf::new::<OsString>(&OsStringExt::from_vec(v)))

