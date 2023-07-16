plain
travis_time:end:06d26c90:start=1548650940413054112,finish=1548651020006891918,duration=79593837806
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
[01:12:06] 
[01:12:06] running 118 tests
[01:12:31] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:36] ......iii.i.....ii
[01:12:36] 
[01:12:36]  finished in 29.906
[01:12:36] travis_fold:end:test_debuginfo

---
[01:33:44] 
[01:33:44] error[E0308]: mismatched types
[01:33:44]    --> src/librustc_driver/test.rs:104:23
[01:33:44]     |
[01:33:44] 104 |         emitter: Some(emitter),
[01:33:44]     |                       ^^^^^^^ expected trait `std::io::Write`, found trait `errors::emitter::Emitter`
[01:33:44]     = note: expected type `std::boxed::Box<dyn std::io::Write + std::marker::Send>`
[01:33:44]     = note: expected type `std::boxed::Box<dyn std::io::Write + std::marker::Send>`
[01:33:44]                found type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + 'static)>`
[01:33:44] error: aborting due to 3 previous errors
[01:33:44] 
[01:33:44] For more information about this error, try `rustc --explain E0308`.
[01:33:45] error: Could not compile `rustc_driver`.
[01:33:45] error: Could not compile `rustc_driver`.
[01:33:45] 
[01:33:45] To learn more, run the command again with --verbose.
[01:33:45] 
[01:33:45] 
[01:33:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:33:45] 
[01:33:45] 
[01:33:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:45] Build completed unsuccessfully in 0:33:16
[01:33:45] Build completed unsuccessfully in 0:33:16
[01:33:45] make: *** [check] Error 1
[01:33:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02e38cae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 06:24:13 UTC 2019
