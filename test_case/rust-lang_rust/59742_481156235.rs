plain
travis_time:end:017a8ef5:start=1554792560318343512,finish=1554792562568734330,duration=2250390818
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:20] 
[01:15:20] running 9 tests
[01:15:20] iiiiiiiii
[01:15:20] 
[01:15:20]  finished in 0.164
[01:15:20] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:37] 
[01:15:37] running 121 tests
[01:16:06] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:11] i.i......iii.i.....ii
[01:16:11] 
[01:16:11]  finished in 34.122
[01:16:11] travis_fold:end:test_debuginfo

---
[01:42:58]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:886:22
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 886 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:902:22
[01:43:04]     |
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 902 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:921:22
[01:43:04]     |
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 921 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:943:22
[01:43:04]     |
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 943 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:962:22
[01:43:04]     |
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 962 |           let output = make_test(input, Some("std"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]    --> src/librustdoc/test.rs:982:22
[01:43:04]     |
[01:43:04]     |
[01:43:04] 342 | / pub fn make_test(s: &str,
[01:43:04] 343 | |                  cratename: Option<&str>,
[01:43:04] 344 | |                  dont_insert_main: bool,
[01:43:04] 345 | |                  opts: &TestOptions,
[01:43:04] ...   |
[01:43:04] 498 | |     (prog, line_offset)
[01:43:04]     | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 982 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1000:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1000 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1020:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1020 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1033:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1033 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1051:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1051 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1068:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1068 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1085:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1085 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1100:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1100 |           let output = make_test(input, None, true, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1115:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1115 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1134:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1134 |           let output = make_test(input, None, false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1149:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1149 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:43:04] 
[01:43:04] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:43:04]     --> src/librustdoc/test.rs:1168:22
[01:43:04]      |
[01:43:04]      |
[01:43:04] 342  | / pub fn make_test(s: &str,
[01:43:04] 343  | |                  cratename: Option<&str>,
[01:43:04] 344  | |                  dont_insert_main: bool,
[01:43:04] 345  | |                  opts: &TestOptions,
[01:43:04] ...    |
[01:43:04] 498  | |     (prog, line_offset)
[01:43:04]      | |_- defined here
[01:43:04] ...
[01:43:04] ...
[01:43:04] 1168 |           let output = make_test(input, Some("my_crate"), false, &opts);
[01:43:04] 
[01:43:04] error: aborting due to 17 previous errors
[01:43:04] 
[01:43:04] For more information about this error, try `rustc --explain E0061`.
[01:43:04] For more information about this error, try `rustc --explain E0061`.
[01:43:04] error: Could not compile `rustdoc`.
[01:43:04] 
[01:43:04] To learn more, run the command again with --verbose.
[01:43:04] 
[01:43:04] 
[01:43:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:43:04] 
[01:43:04] 
[01:43:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:04] Build completed unsuccessfully in 0:40:24
[01:43:04] Build completed unsuccessfully in 0:40:24
[01:43:04] Makefile:48: recipe for target 'check' failed
[01:43:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e63f5dd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  9 08:32:37 UTC 2019
---
travis_time:end:0bb35fcb:start=1554798759919450555,finish=1554798759925763518,duration=6312963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05865c2c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:140fe948
travis_time:start:140fe948
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cdbb430
$ dmesg | grep -i kill
