plain
travis_time:end:01374f2c:start=1552420356877489189,finish=1552420468112425435,duration=111234936246
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
[01:21:39] 
[01:21:39] running 120 tests
[01:22:05] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:10] .i......iii.i.....ii
[01:22:10] 
[01:22:10]  finished in 30.739
[01:22:10] travis_fold:end:test_debuginfo

---
[01:42:50]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:42:58] error[E0425]: cannot find value `json` in this scope
[01:42:58]     --> src/librustc/session/config.rs:2925:55
[01:42:58]      |
[01:42:58] 2925 |             .push(SearchPath::from_cli_opt("all=mno", json));
[01:42:58] 
[01:42:58] error[E0425]: cannot find value `json` in this scope
[01:42:58]     --> src/librustc/session/config.rs:2927:58
[01:42:58]      |
[01:42:58]      |
[01:42:58] 2927 |             .push(SearchPath::from_cli_opt("native=abc", json));
[01:42:58] 
[01:42:58] error[E0425]: cannot find value `json` in this scope
[01:42:58]     --> src/librustc/session/config.rs:2929:57
[01:42:58]      |
[01:42:58]      |
[01:42:58] 2929 |             .push(SearchPath::from_cli_opt("crate=def", json));
[01:42:58] 
[01:42:58] error[E0425]: cannot find value `json` in this scope
[01:42:58]     --> src/librustc/session/config.rs:2931:62
[01:42:58]      |
[01:42:58]      |
[01:42:58] 2931 |             .push(SearchPath::from_cli_opt("dependency=ghi", json));
[01:42:58] 
[01:42:58] error[E0425]: cannot find value `json` in this scope
[01:42:58]     --> src/librustc/session/config.rs:2933:61
[01:42:58]      |
[01:42:58]      |
[01:42:58] 2933 |             .push(SearchPath::from_cli_opt("framework=jkl", json));
[01:42:58] 
[01:43:26] error: aborting due to 5 previous errors
[01:43:26] 
[01:43:26] For more information about this error, try `rustc --explain E0425`.
[01:43:26] For more information about this error, try `rustc --explain E0425`.
[01:43:26] error: Could not compile `rustc`.
[01:43:26] 
[01:43:26] To learn more, run the command again with --verbose.
[01:43:26] 
[01:43:26] 
[01:43:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:43:26] 
[01:43:26] 
[01:43:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:26] Build completed unsuccessfully in 0:34:01
[01:43:26] Build completed unsuccessfully in 0:34:01
[01:43:26] make: *** [check] Error 1
[01:43:26] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2695d388
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 21:38:04 UTC 2019
---
travis_time:end:0d124dfc:start=1552426686289025679,finish=1552426686295420300,duration=6394621
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b265fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0406050b
travis_time:start:0406050b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:136ab85b
$ dmesg | grep -i kill
