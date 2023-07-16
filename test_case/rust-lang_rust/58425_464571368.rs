plain
travis_time:end:037c875e:start=1550455553814798650,finish=1550455626830561442,duration=73015762792
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
[01:12:02] 
[01:12:02] running 119 tests
[01:12:27] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:31] i......iii.i.....ii
[01:12:31] 
[01:12:31]  finished in 29.275
[01:12:31] travis_fold:end:test_debuginfo

---
[01:33:25]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:33:26] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:33:26]    --> src/librustc_driver/test.rs:109:16
[01:33:26]     |
[01:33:26] 109 |       let sess = session::build_session_(
[01:33:26] 110 | |         options,
[01:33:26] 111 | |         None,
[01:33:26] 112 | |         diagnostic_handler,
[01:33:26] 112 | |         diagnostic_handler,
[01:33:26] 113 | |         Lrc::new(SourceMap::new(FilePathMapping::empty())),
[01:33:26]     | |_____^ expected 5 parameters
[01:33:26] 
[01:33:27] error: aborting due to previous error
[01:33:27] 
[01:33:27] 
[01:33:27] For more information about this error, try `rustc --explain E0061`.
[01:33:27] error: Could not compile `rustc_driver`.
[01:33:27] 
[01:33:27] To learn more, run the command again with --verbose.
[01:33:27] 
[01:33:27] 
[01:33:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:33:27] 
[01:33:27] 
[01:33:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:27] Build completed unsuccessfully in 0:33:03
[01:33:27] Build completed unsuccessfully in 0:33:03
[01:33:27] Makefile:48: recipe for target 'check' failed
[01:33:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19b9f858
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 18 03:40:43 UTC 2019
