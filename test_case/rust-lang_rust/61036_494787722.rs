plain
travis_time:end:059dab9c:start=1558523349983370890,finish=1558523350753065731,duration=769694841
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
[01:22:09] 
[01:22:09] running 143 tests
[01:22:12] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:22:14] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:22:14] 
[01:22:14]  finished in 4.707
[01:22:14] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:16] 
[01:22:16] running 9 tests
[01:22:16] iiiiiiiii
[01:22:16] 
[01:22:16]  finished in 0.157
[01:22:16] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:32] 
[01:22:32] running 122 tests
[01:22:58] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:23:03] .i.i......iii.i.....ii
[01:23:03] 
[01:23:03]  finished in 30.919
[01:23:03] travis_fold:end:test_debuginfo

---
[01:38:37]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:39:05] error[E0308]: mismatched types
[01:39:05]     --> src/librustc/session/config.rs:3215:39
[01:39:05]      |
[01:39:05] 3215 |         opts.debugging_opts.pgo_use = String::from("abc");
[01:39:05]      |
[01:39:05]      = note: expected type `std::option::Option<std::path::PathBuf>`
[01:39:05]                 found type `std::string::String`
[01:39:05] 
---
[01:39:15] 
[01:39:15] To learn more, run the command again with --verbose.
[01:39:15] 
[01:39:15] 
[01:39:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:39:15] 
[01:39:15] 
[01:39:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:15] Build completed unsuccessfully in 0:29:07
[01:39:15] Build completed unsuccessfully in 0:29:07
[01:39:15] make: *** [check] Error 1
[01:39:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b683636
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 22 12:48:36 UTC 2019
---
travis_time:end:166f7535:start=1558529318328525576,finish=1558529318333528088,duration=5002512
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05289720
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:102bef9e
travis_time:start:102bef9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ae7407c
$ dmesg | grep -i kill
