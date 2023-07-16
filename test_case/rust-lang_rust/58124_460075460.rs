plain
travis_time:end:0a406fc7:start=1549212578669716099,finish=1549212579591395269,duration=921679170
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:29] 
[01:10:29] running 119 tests
[01:10:54] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:58] i......iii.i.....ii
[01:10:58] 
[01:10:58]  finished in 29.286
[01:10:58] travis_fold:end:test_debuginfo

---
[01:35:01]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[01:35:01] error[E0432]: unresolved import `Globals`
[01:35:01]    --> src/libsyntax_pos/symbol.rs:713:9
[01:35:01]     |
[01:35:01] 713 |     use Globals;
[01:35:01]     |         ^^^^^^^ no `Globals` external crate
[01:35:02] error: aborting due to previous error
[01:35:02] 
[01:35:02] For more information about this error, try `rustc --explain E0432`.
[01:35:02] error: Could not compile `syntax_pos`.
[01:35:02] error: Could not compile `syntax_pos`.
[01:35:02] 
[01:35:02] To learn more, run the command again with --verbose.
[01:35:02] 
[01:35:02] 
[01:35:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_pos" "--" "--quiet"
[01:35:02] 
[01:35:02] 
[01:35:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:02] Build completed unsuccessfully in 0:36:11
[01:35:02] Build completed unsuccessfully in 0:36:11
[01:35:02] make: *** [check] Error 1
[01:35:02] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f08d80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 18:24:53 UTC 2019
---
travis_time:end:174dfd90:start=1549218295071388753,finish=1549218295129520036,duration=58131283
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05eb64d1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:065a35be
$ dmesg | grep -i kill
