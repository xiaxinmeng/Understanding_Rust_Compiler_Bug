plain
travis_time:end:143462d9:start=1552482467012353591,finish=1552482547382462472,duration=80370108881
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:33] 
[01:21:33] running 120 tests
[01:21:59] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:04] .i......iii.i.....ii
[01:22:04] 
[01:22:04]  finished in 31.187
[01:22:04] travis_fold:end:test_debuginfo

---
[01:45:56]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:46:09] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:46:09]   --> src/libsyntax/test_snippet.rs:57:23
[01:46:09]    |
[01:46:09] 57 |           let emitter = EmitterWriter::new(Box::new(Shared { data: output.clone() }),
[01:46:09]    |  _______________________^
[01:46:09] 58 | |                                         Some(source_map.clone()),
[01:46:09] 60 | |                                         false);
[01:46:09]    | |______________________________________________^ expected 5 parameters
[01:46:09] 
[01:46:09] error: aborting due to previous error
[01:46:09] error: aborting due to previous error
[01:46:09] 
[01:46:09] For more information about this error, try `rustc --explain E0061`.
[01:46:09] error: Could not compile `syntax`.
[01:46:09] 
[01:46:09] To learn more, run the command again with --verbose.
[01:46:09] 
[01:46:09] 
[01:46:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:46:09] 
[01:46:09] 
[01:46:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:09] Build completed unsuccessfully in 0:36:35
[01:46:09] Build completed unsuccessfully in 0:36:35
[01:46:09] Makefile:48: recipe for target 'check' failed
[01:46:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b93bdd5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 14:55:25 UTC 2019
---
travis_time:end:195f27db:start=1552488926818995976,finish=1552488926875656975,duration=56660999
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f405a3c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:028f894e
$ dmesg | grep -i kill
