plain
travis_time:end:03177f7a:start=1555597059647557616,finish=1555597186721138093,duration=127073580477
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:28] 
[01:16:28] running 9 tests
[01:16:28] iiiiiiiii
[01:16:28] 
[01:16:28]  finished in 0.150
[01:16:28] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:44] 
[01:16:44] running 121 tests
[01:17:09] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:17:13] i.i......iii.i.....ii
[01:17:13] 
[01:17:13]  finished in 29.569
[01:17:13] travis_fold:end:test_debuginfo

---
[01:42:40]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:42:44] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:44]     --> src/librustdoc/html/markdown.rs:1148:26
[01:42:44]      |
[01:42:44] 50   | / pub struct Markdown<'a>(
[01:42:44] 51   | |     pub &'a str, pub &'a [(String, String)], pub RefCell<&'a mut IdMap>, pub ErrorCodes, pub Edition);
[01:42:44]      | |______________________________________________________________________________________________________- defined here
[01:42:44] ...
[01:42:44] 1148 |               let output = Markdown(input, &[], RefCell::new(&mut map), ErrorCodes::Yes).to_string();
[01:42:44] 
[01:42:44] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:44]     --> src/librustdoc/html/markdown.rs:1170:26
[01:42:44]      |
[01:42:44]      |
[01:42:44] 50   | / pub struct Markdown<'a>(
[01:42:44] 51   | |     pub &'a str, pub &'a [(String, String)], pub RefCell<&'a mut IdMap>, pub ErrorCodes, pub Edition);
[01:42:44]      | |______________________________________________________________________________________________________- defined here
[01:42:44] ...
[01:42:44] 1170 |               let output = Markdown(input, &[], RefCell::new(map), ErrorCodes::Yes).to_string();
[01:42:44] 
[01:42:44] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:42:44]     --> src/librustdoc/html/markdown.rs:1207:26
[01:42:44]      |
[01:42:44]      |
[01:42:44] 56   | pub struct MarkdownHtml<'a>(pub &'a str, pub RefCell<&'a mut IdMap>, pub ErrorCodes, pub Edition);
[01:42:44]      | -------------------------------------------------------------------------------------------------- defined here
[01:42:44] ...
[01:42:44] 1207 |             let output = MarkdownHtml(input, RefCell::new(&mut idmap), ErrorCodes::Yes).to_string();
[01:42:44] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:888:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 888 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:904:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 904 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:923:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 923 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:945:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 945 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:964:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 964 |           let output = make_test(input, Some("std"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]    --> src/librustdoc/test.rs:984:22
[01:42:46]     |
[01:42:46]     |
[01:42:46] 341 | / pub fn make_test(s: &str,
[01:42:46] 342 | |                  cratename: Option<&str>,
[01:42:46] 343 | |                  dont_insert_main: bool,
[01:42:46] 344 | |                  opts: &TestOptions,
[01:42:46] ...   |
[01:42:46] 500 | |     (prog, line_offset)
[01:42:46]     | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 984 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1002:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1002 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1022:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1022 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1035:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1035 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1053:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1053 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1070:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1070 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1087:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1087 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1102:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1102 |           let output = make_test(input, None, true, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1117:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1117 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1136:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1136 |           let output = make_test(input, None, false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1151:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1151 |           let output = make_test(input, Some("asdf"), false, &opts);
[01:42:46] 
[01:42:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:42:46]     --> src/librustdoc/test.rs:1170:22
[01:42:46]      |
[01:42:46]      |
[01:42:46] 341  | / pub fn make_test(s: &str,
[01:42:46] 342  | |                  cratename: Option<&str>,
[01:42:46] 343  | |                  dont_insert_main: bool,
[01:42:46] 344  | |                  opts: &TestOptions,
[01:42:46] ...    |
[01:42:46] 500  | |     (prog, line_offset)
[01:42:46]      | |_- defined here
[01:42:46] ...
[01:42:46] ...
[01:42:46] 1170 |           let output = make_test(input, Some("my_crate"), false, &opts);
[01:42:46] 
[01:42:46] error: aborting due to 20 previous errors
[01:42:46] 
[01:42:46] For more information about this error, try `rustc --explain E0061`.
[01:42:46] For more information about this error, try `rustc --explain E0061`.
[01:42:46] error: Could not compile `rustdoc`.
[01:42:46] 
[01:42:46] To learn more, run the command again with --verbose.
[01:42:46] 
[01:42:46] 
[01:42:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:42:46] 
[01:42:46] 
[01:42:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:46] Build completed unsuccessfully in 0:37:49
[01:42:46] Build completed unsuccessfully in 0:37:49
[01:42:46] make: *** [check] Error 1
[01:42:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:279789f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 16:02:42 UTC 2019
---
travis_time:end:11201d84:start=1555603364294673843,finish=1555603364299632420,duration=4958577
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0751ab04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:151f5650
travis_time:start:151f5650
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09abbb1c
$ dmesg | grep -i kill
