plain
travis_time:end:0c472992:start=1549066034120299804,finish=1549066176806258886,duration=142685959082
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
[01:13:04] 
[01:13:04] running 119 tests
[01:13:33] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:37] i......iii.i.....ii
[01:13:37] 
[01:13:37]  finished in 33.028
[01:13:37] travis_fold:end:test_debuginfo

---
[01:35:27]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:35:28] error[E0308]: mismatched types
[01:35:28]   --> src/librustc_driver/test.rs:71:9
[01:35:28]    |
[01:35:28] 71 |         box ExpectErrorEmitter { messages: v } as Box<dyn Emitter + sync::Send>,
[01:35:28]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait `errors::emitter::Emitter + rustc_data_structures::sync::Send + std::marker::Send`, found trait `errors::emitter::Emitter + rustc_data_structures::sync::Send`
[01:35:28]    |
[01:35:28]    = note: expected type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + std::marker::Send + 'static)>`
[01:35:28]               found type `std::boxed::Box<dyn errors::emitter::Emitter + rustc_data_structures::sync::Send>`
[01:35:28] 
[01:35:28] error[E0599]: no function or associated item named `new` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[01:35:28]   --> src/librustc_driver/test.rs:95:31
[01:35:28]    |
[01:35:28] 95 |         crate_cfg: FxHashSet::new(),
[01:35:28]    |                    |
[01:35:28]    |                    |
[01:35:28]    |                    function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[01:35:29] error: aborting due to 2 previous errors
[01:35:29] 
[01:35:29] Some errors occurred: E0308, E0599.
[01:35:29] For more information about an error, try `rustc --explain E0308`.
[01:35:29] For more information about an error, try `rustc --explain E0308`.
[01:35:29] error: Could not compile `rustc_driver`.
[01:35:29] 
[01:35:29] To learn more, run the command again with --verbose.
[01:35:29] 
[01:35:29] 
[01:35:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:35:29] 
[01:35:29] 
[01:35:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:29] Build completed unsuccessfully in 0:35:10
[01:35:29] Build completed unsuccessfully in 0:35:10
[01:35:29] make: *** [check] Error 1
[01:35:29] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:169dac4e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 01:45:14 UTC 2019
---
travis_time:end:026261b8:start=1549071916915543223,finish=1549071916976900039,duration=61356816
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:185b5bc1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055950a2
$ dmesg | grep -i kill
