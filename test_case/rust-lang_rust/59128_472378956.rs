plain
travis_time:end:058f13b0:start=1552468863544412186,finish=1552468939821239933,duration=76276827747
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
[01:20:40] 
[01:20:40] running 120 tests
[01:21:10] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:15] .i......iii.i.....ii
[01:21:15] 
[01:21:15]  finished in 34.890
[01:21:15] travis_fold:end:test_debuginfo

---
[01:46:36]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:46:46] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:46:46]     --> src/libsyntax/parse/lexer/mod.rs:1930:23
[01:46:46]      |
[01:46:46] 1930 |           let emitter = errors::emitter::EmitterWriter::new(Box::new(io::sink()),
[01:46:46]      |  _______________________^
[01:46:46] 1931 | |                                                           Some(sm.clone()),
[01:46:46] 1933 | |                                                           false);
[01:46:46]      | |________________________________________________________________^ expected 5 parameters
[01:46:46] 
[01:46:48] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:46:48] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:46:48]   --> src/libsyntax/test_snippet.rs:57:23
[01:46:48]    |
[01:46:48] 57 |           let emitter = EmitterWriter::new(Box::new(Shared { data: output.clone() }),
[01:46:48]    |  _______________________^
[01:46:48] 58 | |                                         Some(source_map.clone()),
[01:46:48] 60 | |                                         false);
[01:46:48]    | |______________________________________________^ expected 5 parameters
[01:46:48] 
[01:46:49] error: aborting due to 2 previous errors
[01:46:49] error: aborting due to 2 previous errors
[01:46:49] 
[01:46:49] For more information about this error, try `rustc --explain E0061`.
[01:46:49] error: Could not compile `syntax`.
[01:46:49] 
[01:46:49] To learn more, run the command again with --verbose.
[01:46:49] 
[01:46:49] 
[01:46:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:46:49] 
[01:46:49] 
[01:46:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:49] Build completed unsuccessfully in 0:39:10
[01:46:49] Build completed unsuccessfully in 0:39:10
[01:46:49] make: *** [check] Error 1
[01:46:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:013b568b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 11:09:19 UTC 2019
---
travis_time:end:06d6b0a0:start=1552475361816035108,finish=1552475361823304755,duration=7269647
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0099a8f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d79ed17
travis_time:start:2d79ed17
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27d86140
$ dmesg | grep -i kill
