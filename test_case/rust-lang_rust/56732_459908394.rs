plain
travis_time:end:04327378:start=1549060065589144727,finish=1549060178710326314,duration=113121181587
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
[01:06:51] 
[01:06:51] running 119 tests
[01:07:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:07:19] i......iii.i.....ii
[01:07:19] 
[01:07:19]  finished in 28.322
[01:07:19] travis_fold:end:test_debuginfo

---
[01:26:41]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:26:43] error[E0308]: mismatched types
[01:26:43]   --> src/librustc_driver/test.rs:96:20
[01:26:43]    |
[01:26:43] 96 |         crate_cfg: HashSet::new(),
[01:26:43]    |
[01:26:43]    |
[01:26:43]    = note: expected type `std::collections::HashSet<(std::string::String, std::option::Option<std::string::String>), std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[01:26:43]               found type `std::collections::HashSet<_, std::collections::hash_map::RandomState>`
[01:26:43] error[E0308]: mismatched types
[01:26:43]    --> src/librustc_driver/test.rs:102:54
[01:26:43]     |
[01:26:43]     |
[01:26:43] 102 |         diagnostic_output: DiagnosticOutput::Emitter(emitter),
[01:26:43]     |                                                      ^^^^^^^ expected trait `errors::emitter::Emitter + rustc_data_structures::sync::Send + std::marker::Send`, found trait `errors::emitter::Emitter + rustc_data_structures::sync::Send`
[01:26:43]     |
[01:26:43]     = note: expected type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + std::marker::Send + 'static)>`
[01:26:43]                found type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + 'static)>`
[01:26:43] 
[01:26:43] error[E0063]: missing field `lint_caps` in initializer of `rustc_interface::Config`
[01:26:43]   --> src/librustc_driver/test.rs:94:18
[01:26:43] 94 |     let config = interface::Config {
[01:26:43]    |                  ^^^^^^^^^^^^^^^^^ missing `lint_caps`
[01:26:43] 
[01:26:43] error: aborting due to 3 previous errors
---
[01:26:43] 
[01:26:43] To learn more, run the command again with --verbose.
[01:26:43] 
[01:26:43] 
[01:26:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:26:43] 
[01:26:43] 
[01:26:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:43] Build completed unsuccessfully in 0:31:01
[01:26:43] Build completed unsuccessfully in 0:31:01
[01:26:43] make: *** [check] Error 1
[01:26:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01453584
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 23:56:33 UTC 2019
---
travis_time:end:07784aea:start=1549065394825852707,finish=1549065394830478012,duration=4625305
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1063a0a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:064025d2
travis_time:start:064025d2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b31b363
$ dmesg | grep -i kill
