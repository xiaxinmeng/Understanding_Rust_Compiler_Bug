plain
travis_time:end:03c00924:start=1561313219021875745,finish=1561313221482241957,duration=2460366212
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
[01:06:49] 
[01:06:49] running 9 tests
[01:06:49] iiiiiiiii
[01:06:49] 
[01:06:49]  finished in 0.156
[01:06:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:06] 
[01:07:06] running 122 tests
[01:07:31] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:36] .i.i......iii.i.....ii
[01:07:36] 
[01:07:36]  finished in 30.434
[01:07:36] travis_fold:end:test_debuginfo

---
[01:28:56]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:29:01] error: hidden lifetime parameters in types are deprecated
[01:29:01]   --> src/libsyntax/util/parser_testing.rs:23:64
[01:29:01]    |
[01:29:01] 23 | pub fn string_to_parser(ps: &ParseSess, source_str: String) -> Parser {
[01:29:01] 
[01:29:09] error: aborting due to previous error
[01:29:09] 
[01:29:09] error: Could not compile `syntax`.
[01:29:09] error: Could not compile `syntax`.
[01:29:09] 
[01:29:09] To learn more, run the command again with --verbose.
[01:29:09] 
[01:29:09] 
[01:29:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:29:09] 
[01:29:09] 
[01:29:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:09] Build completed unsuccessfully in 1:23:59
---
travis_time:end:03d9c598:start=1561318582755011415,finish=1561318582761247371,duration=6235956
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:048c76a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e4f5bf8
travis_time:start:0e4f5bf8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08c22952
$ dmesg | grep -i kill
