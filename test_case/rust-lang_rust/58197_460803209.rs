plain
travis_time:end:1f6ae502:start=1549400382472696980,finish=1549400383507986849,duration=1035289869
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:07]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:05:07]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:07]    Compiling rustc-demangle v0.1.10
[00:05:13]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:48:1
[00:05:20]    |
[00:05:20] 48 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_char = i8;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:50:1
[00:05:20]    |
[00:05:20] 50 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_schar = i8;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:52:1
[00:05:20]    |
[00:05:20] 52 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_uchar = u8;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:54:1
[00:05:20]    |
[00:05:20] 54 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_short = i16;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:56:1
[00:05:20]    |
[00:05:20] 56 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_ushort = u16;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:58:1
[00:05:20]    |
[00:05:20] 58 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_int = i32;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:60:1
[00:05:20]    |
[00:05:20] 60 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_uint = u32;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:69:1
[00:05:20]    |
[00:05:20] 69 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_long = i64;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:72:1
[00:05:20]    |
[00:05:20] 72 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_ulong = u64;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:74:1
[00:05:20]    |
[00:05:20] 74 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_longlong = i64;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:76:1
[00:05:20]    |
[00:05:20] 76 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_ulonglong = u64;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:78:1
[00:05:20]    |
[00:05:20] 78 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_float = f32;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:80:1
[00:05:20]    |
[00:05:20] 80 | #[stable(feature = "raw_os", since = "1.1.0")] pub type c_double = f64;
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]   --> src/libstd/os/raw/mod.rs:82:1
[00:05:20]    |
[00:05:20] 82 | #[stable(feature = "raw_os", since = "1.1.0")]
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0711]: feature `raw_os` is declared stable since 1.1.0, but was previously declared stable since 1.30.0
[00:05:20]  --> src/libstd/os/raw/mod.rs:9:1
[00:05:20]   |
[00:05:20] 9 | #![stable(feature = "raw_os", since = "1.1.0")]
[00:05:20] 
[00:05:20] error: aborting due to 15 previous errors
[00:05:20] 
[00:05:20] For more information about this error, try `rustc --explain E0711`.
[00:05:20] For more information about this error, try `rustc --explain E0711`.
[00:05:20] error: Could not compile `std`.
[00:05:20] 
[00:05:20] To learn more, run the command again with --verbose.
[00:05:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:20] expected success, got: exit code: 101
[00:05:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:20] Build completed unsuccessfully in 0:00:53
[00:05:20] make: *** [all] Error 1
[00:05:20] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03ad2dd0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 21:05:14 UTC 2019
---
travis_time:end:21751ec0:start=1549400715531026670,finish=1549400715536152944,duration=5126274
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04bab376
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d7997c7
travis_time:start:2d7997c7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0571bbdc
$ dmesg | grep -i kill
