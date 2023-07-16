plain
travis_time:end:333602bf:start=1554527599732812772,finish=1554527602194817259,duration=2462004487
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
[01:14:57] 
[01:14:57] running 9 tests
[01:14:57] iiiiiiiii
[01:14:57] 
[01:14:57]  finished in 0.154
[01:14:57] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:14] 
[01:15:14] running 121 tests
[01:15:40] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:44] i.i......iii.i.....ii
[01:15:44] 
[01:15:44]  finished in 30.256
[01:15:44] travis_fold:end:test_debuginfo

---
[01:42:00]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[01:42:01] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[01:42:01]    --> src/libsyntax_pos/symbol.rs:745:22
[01:42:01]     |
[01:42:01] 745 |         GLOBALS.set(&Globals::new(), || {
[01:42:01]     | 
[01:42:01]    ::: src/libsyntax_pos/lib.rs:58:5
[01:42:01]     |
[01:42:01]     |
[01:42:01] 58  |     pub fn new(edition: Edition) -> Globals {
[01:42:01]     |     --------------------------------------- defined here
[01:42:01] error: aborting due to previous error
[01:42:01] 
[01:42:01] For more information about this error, try `rustc --explain E0061`.
[01:42:01] error: Could not compile `syntax_pos`.
[01:42:01] error: Could not compile `syntax_pos`.
[01:42:01] 
[01:42:01] To learn more, run the command again with --verbose.
[01:42:01] 
[01:42:01] 
[01:42:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_pos" "--" "--quiet"
[01:42:01] 
[01:42:01] 
[01:42:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:01] Build completed unsuccessfully in 0:39:00
[01:42:01] Build completed unsuccessfully in 0:39:00
[01:42:01] make: *** [check] Error 1
[01:42:01] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021d37a9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 06:55:35 UTC 2019
---
travis_time:end:11017df7:start=1554533736692787588,finish=1554533736697734184,duration=4946596
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:34843233
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14e414ce
travis_time:start:14e414ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:227a3d1a
$ dmesg | grep -i kill
