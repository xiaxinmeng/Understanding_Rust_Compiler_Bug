plain
travis_time:end:08e27446:start=1542570299639718729,finish=1542570301793634430,duration=2153915701
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:02] .................................................................................................... 100/5022
[00:51:05] .................................................................................................... 200/5022
[00:51:08] .............................ii............................................ii...................ii.. 300/5022
[00:51:11] ..............................................................................................iii... 400/5022
[00:51:14] .....iiiiiiii.iii............................iii...........................................i........ 500/5022
[00:51:21] .................................................................................................... 700/5022
[00:51:27] .................................................................................i...........i...... 800/5022
[00:51:30] .................................................................................................... 900/5022
[00:51:30] .................................................................................................... 900/5022
[00:51:34] .iiiii..................iiiiii...................................................................... 1000/5022
[00:51:36] ...........................................................................iiiiiiii................. 1100/5022
[00:51:41] .................................................................................................... 1300/5022
[00:51:43] .................................................................................................... 1400/5022
[00:51:45] ................................i................................................................... 1500/5022
[00:51:48] .i.........ii.........................................................i............................. 1600/5022
---
[00:52:09] .................................................................................................... 2200/5022
[00:52:13] .................................................................................................... 2300/5022
[00:52:17] .................................................................................................... 2400/5022
[00:52:21] .................................................................................................... 2500/5022
[00:52:24] .................................................................................iiiiiiiii.......... 2600/5022
[00:52:31] ...............................................ii................................................... 2800/5022
[00:52:34] .................................................................................................... 2900/5022
[00:52:38] .................................................................................................... 3000/5022
[00:52:41] ..........................................i......................................................... 3100/5022
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:13] 
[01:06:13] running 116 tests
[01:06:16] i..ii...iii...iiii....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:06:16] i.i....iiii.....
[01:06:16] 
[01:06:16]  finished in 3.462
[01:06:16] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:30] 
[01:06:30] running 118 tests
[01:06:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:07:02] ......iii.i.....ii
[01:07:02] 
[01:07:02]  finished in 31.792
[01:07:02] travis_fold:end:test_debuginfo

---
[01:31:07] travis_fold:start:test_stage1-rustc_data_structures
travis_time:start:test_stage1-rustc_data_structures
Testing rustc_data_structures stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:31:07]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[01:31:10] error[E0271]: type mismatch resolving `<sorted_map::Iter<'_, _, _> as std::iter::Iterator>::Item == &_`
[01:31:10]    --> librustc_data_structures/sorted_map.rs:350:35
[01:31:10]     |
[01:31:10] 350 |             assert_eq!(map.iter().cloned().collect::<Vec<_>>(), expected);
[01:31:10]     |                                   ^^^^^^ expected tuple, found reference
[01:31:10]     |
[01:31:10]     = note: expected type `(&_, &_)`
[01:31:10] 
[01:31:10] 
[01:31:10] error[E0599]: no method named `collect` found for type `std::iter::Cloned<sorted_map::Iter<'_, _, _>>` in the current scope
[01:31:10]    --> librustc_data_structures/sorted_map.rs:350:44
[01:31:10]     |
[01:31:10] 350 |             assert_eq!(map.iter().cloned().collect::<Vec<_>>(), expected);
[01:31:10]     |
[01:31:10]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:31:10]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:31:10]             `std::iter::Cloned<sorted_map::Iter<'_, _, _>> : std::iter::Iterator`
[01:31:10]             `&mut std::iter::Cloned<sorted_map::Iter<'_, _, _>> : std::iter::Iterator`
[01:31:10] 
[01:31:10] error[E0271]: type mismatch resolving `<sorted_map::Iter<'_, {integer}, {integer}> as std::iter::Iterator>::Item == &_`
[01:31:10]    --> librustc_data_structures/sorted_map.rs:478:35
[01:31:10]     |
[01:31:10] 478 |             assert_eq!(map.iter().cloned().collect::<Vec<_>>(), expected);
[01:31:10]     |                                   ^^^^^^ expected tuple, found reference
[01:31:10]     |
[01:31:10]     = note: expected type `(&{integer}, &{integer})`
[01:31:10] 
[01:31:10] 
[01:31:10] error[E0599]: no method named `collect` found for type `std::iter::Cloned<sorted_map::Iter<'_, {integer}, {integer}>>` in the current scope
[01:31:10]    --> librustc_data_structures/sorted_map.rs:478:44
[01:31:10]     |
[01:31:10] 478 |             assert_eq!(map.iter().cloned().collect::<Vec<_>>(), expected);
[01:31:10]     |
[01:31:10]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:31:10]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:31:10]             `std::iter::Cloned<sorted_map::Iter<'_, {integer}, {integer}>> : std::iter::Iterator`
[01:31:10]             `&mut std::iter::Cloned<sorted_map::Iter<'_, {integer}, {integer}>> : std::iter::Iterator`
[01:31:10] error: aborting due to 4 previous errors
[01:31:10] 
[01:31:10] Some errors occurred: E0271, E0599.
[01:31:10] For more information about an error, try `rustc --explain E0271`.
[01:31:10] For more information about an error, try `rustc --explain E0271`.
[01:31:10] error: Could not compile `rustc_data_structures`.
[01:31:10] 
[01:31:10] To learn more, run the command again with --verbose.
[01:31:10] 
[01:31:10] 
[01:31:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:31:10] 
[01:31:10] 
[01:31:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:10] Build completed unsuccessfully in 0:43:52
[01:31:10] Build completed unsuccessfully in 0:43:52
[01:31:10] Makefile:58: recipe for target 'check' failed
[01:31:10] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c73445e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 18 21:16:22 UTC 2018
---
travis_time:end:32e57d09:start=1542575786097449101,finish=1542575786286473907,duration=189024806
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:37265737
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c07349c
$ dmesg | grep -i kill
