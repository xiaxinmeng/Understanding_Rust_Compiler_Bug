plain
travis_time:end:2cae2245:start=1558906905998030792,finish=1558906908543440531,duration=2545409739
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:34] 
[01:19:34] running 143 tests
[01:19:37] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:19:39] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:39] 
[01:19:39]  finished in 4.805
[01:19:39] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:41] 
[01:19:41] running 9 tests
[01:19:41] iiiiiiiii
[01:19:41] 
[01:19:41]  finished in 0.154
[01:19:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:56] 
[01:19:56] running 122 tests
[01:20:21] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:26] .i.i......iii.i.....ii
[01:20:26] 
[01:20:26]  finished in 30.269
[01:20:26] travis_fold:end:test_debuginfo

---
[01:33:31]    Doc-tests std
[01:33:33] 
[01:33:33] running 920 tests
[01:33:54] i................................................................................................... 100/920
[01:34:03] .......................FF.FFF.FFFF.iii......i......i...i......i..................................... 200/920
[01:34:06] ....................................F............................................................... 300/920
[01:34:21] ....ii.............................................................................................. 500/920
[01:34:26] .................................................................................................... 600/920
[01:34:34] .............................................iiii................................................... 700/920
[01:34:48] .................................................................................................... 800/920
[01:34:48] .................................................................................................... 800/920
[01:34:54] ..................................................................iiii.............................. 900/920
[01:34:55] ....................
[01:34:55] failures:
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error + 'a>::from (line 225) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]   --> error.rs:247:27
[01:34:55]    |
[01:34:55] 25 | let a_boxed_error = Box::<Error>::from(an_error);
[01:34:55]    |                           ^^^^^ help: use `dyn`: `dyn Error`
[01:34:55] note: lint level defined here
[01:34:55]   --> error.rs:223:9
[01:34:55]    |
[01:34:55] 1  | #![deny(warnings)]
[01:34:55] 1  | #![deny(warnings)]
[01:34:55]    |         ^^^^^^^^
[01:34:55]    = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error + 'a>::from (line 225)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error + Send + Sync + 'a>::from (line 264) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]   --> error.rs:290:27
[01:34:55]    |
[01:34:55] 29 | let a_boxed_error = Box::<Error + Send + Sync>::from(an_error);
[01:34:55]    |                           ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Error + Send + Sync`
[01:34:55] note: lint level defined here
[01:34:55]   --> error.rs:262:9
[01:34:55]    |
[01:34:55] 1  | #![deny(warnings)]
[01:34:55] 1  | #![deny(warnings)]
[01:34:55]    |         ^^^^^^^^
[01:34:55]    = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error + Send + Sync + 'a>::from (line 264)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error + Send + Sync + 'a>::from (line 365) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:370:27
[01:34:55]   |
[01:34:55] 8 | let a_boxed_error = Box::<Error + Send + Sync>::from(a_str_error);
[01:34:55]   |                           ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Error + Send + Sync`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:363:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error + Send + Sync + 'a>::from (line 365)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error + Send + Sync + 'a>::from (line 409) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:415:27
[01:34:55]   |
[01:34:55] 9 | let a_boxed_error = Box::<Error + Send + Sync>::from(a_cow_str_error);
[01:34:55]   |                           ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Error + Send + Sync`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:407:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error + Send + Sync + 'a>::from (line 409)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error + Send + Sync>::from (line 307) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:312:27
[01:34:55]   |
[01:34:55] 8 | let a_boxed_error = Box::<Error + Send + Sync>::from(a_string_error);
[01:34:55]   |                           ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Error + Send + Sync`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:305:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error + Send + Sync>::from (line 307)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error>::from (line 342) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:347:27
[01:34:55]   |
[01:34:55] 8 | let a_boxed_error = Box::<Error>::from(a_string_error);
[01:34:55]   |                           ^^^^^ help: use `dyn`: `dyn Error`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:340:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error>::from (line 342)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error>::from (line 387) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:392:27
[01:34:55]   |
[01:34:55] 8 | let a_boxed_error = Box::<Error>::from(a_str_error);
[01:34:55]   |                           ^^^^^ help: use `dyn`: `dyn Error`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:385:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error>::from (line 387)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Box<Error>::from (line 433) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> error.rs:439:27
[01:34:55]   |
[01:34:55] 9 | let a_boxed_error = Box::<Error>::from(a_cow_str_error);
[01:34:55]   |                           ^^^^^ help: use `dyn`: `dyn Error`
[01:34:55] note: lint level defined here
[01:34:55]  --> error.rs:431:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Box<Error>::from (line 433)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- error.rs - error::Error::cause (line 77) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]   --> error.rs:97:32
[01:34:55]    |
[01:34:55] 22 |     fn cause(&self) -> Option<&Error> {
[01:34:55]    |                                ^^^^^ help: use `dyn`: `dyn Error`
[01:34:55] note: lint level defined here
[01:34:55]   --> error.rs:76:9
[01:34:55]    |
[01:34:55] 1  | #![deny(warnings)]
[01:34:55] 1  | #![deny(warnings)]
[01:34:55]    |         ^^^^^^^^
[01:34:55]    = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'error.rs - error::Error::cause (line 77)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
[01:34:55] 
[01:34:55] ---- fs.rs - fs::read_dir (line 1973) stdout ----
[01:34:55] error: trait objects without an explicit `dyn` are deprecated
[01:34:55]  --> fs.rs:1979:32
[01:34:55]   |
[01:34:55] 9 | fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
[01:34:55]   |                                ^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(&DirEntry)`
[01:34:55] note: lint level defined here
[01:34:55]  --> fs.rs:1971:9
[01:34:55]   |
[01:34:55] 1 | #![deny(warnings)]
[01:34:55] 1 | #![deny(warnings)]
[01:34:55]   |         ^^^^^^^^
[01:34:55]   = note: #[deny(bare_trait_objects)] implied by #[deny(warnings)]
[01:34:55] error: aborting due to previous error
[01:34:55] 
[01:34:55] thread 'fs.rs - fs::read_dir (line 1973)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:34:55] 
---
[01:34:55] 
[01:34:55] error: test failed, to rerun pass '--doc'
[01:34:55] 
[01:34:55] 
[01:34:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:34:55] 
[01:34:55] 
[01:34:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:55] Build completed unsuccessfully in 0:26:58
[01:34:55] Build completed unsuccessfully in 0:26:58
[01:34:55] make: *** [check] Error 1
[01:34:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bfa552e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 23:16:56 UTC 2019
---
travis_time:end:15b84aa8:start=1558912617953970755,finish=1558912617958959979,duration=4989224
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:077f5b9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e810c84
travis_time:start:0e810c84
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f18943e
$ dmesg | grep -i kill
