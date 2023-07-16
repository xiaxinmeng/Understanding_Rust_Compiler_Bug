plain
travis_time:end:046cea0c:start=1559940390406189444,finish=1559940477565180704,duration=87158991260
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
[01:03:58] 
[01:03:58] running 144 tests
[01:04:00] i..iii......iii.iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:04:02] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:04:02] 
[01:04:02]  finished in 4.544
[01:04:02] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:04] 
[01:04:04] running 9 tests
[01:04:04] iiiiiiiii
[01:04:04] 
[01:04:04]  finished in 0.150
[01:04:04] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:20] 
[01:04:20] running 122 tests
[01:04:44] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:49] .i.i......iii.i.....ii
[01:04:49] 
[01:04:49]  finished in 29.605
[01:04:49] travis_fold:end:test_debuginfo

---
[01:29:52]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:29:56] error[E0061]: this function takes 3 parameters but 2 parameters were supplied
[01:29:56]     --> src/librustdoc/html/markdown.rs:1088:24
[01:29:56]      |
[01:29:56] 623  |     fn parse(string: &str, allow_error_code_check: ErrorCodes, enable_per_target_ignores: bool) -> LangString {
[01:29:56]      |     --------------------------------------------------------------------------------------------------------- defined here
[01:29:56] ...
[01:29:56] 1088 |             assert_eq!(LangString::parse(s, ErrorCodes::Yes), LangString {
[01:29:56] 
[01:29:58] error: aborting due to previous error
[01:29:58] 
[01:29:58] For more information about this error, try `rustc --explain E0061`.
[01:29:58] For more information about this error, try `rustc --explain E0061`.
[01:29:58] error: Could not compile `rustdoc`.
[01:29:58] warning: build failed, waiting for other jobs to finish...
[01:32:02] error: build failed
[01:32:02] 
[01:32:02] 
[01:32:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:32:02] 
[01:32:02] 
[01:32:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:02] Build completed unsuccessfully in 1:27:52
---
travis_time:end:00044810:start=1559946010261075355,finish=1559946010266097788,duration=5022433
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18535561
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b702ab6
travis_time:start:1b702ab6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d8ced18
$ dmesg | grep -i kill
