plain
travis_time:end:18c9218d:start=1549495777983132261,finish=1549495780528557231,duration=2545424970
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
[01:08:07] 
[01:08:07] running 119 tests
[01:08:34] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:08:39] i......iii.i.....ii
[01:08:39] 
[01:08:39]  finished in 32.023
[01:08:39] travis_fold:end:test_debuginfo

---
[01:29:02]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:29:03] error[E0425]: cannot find function `get_codegen_backend` in the crate root
[01:29:03]    --> src/librustc_driver/test.rs:115:32
[01:29:03]     |
[01:29:03] 115 |     let cstore = CStore::new(::get_codegen_backend(&sess).metadata_loader());
[01:29:03]     |                                ^^^^^^^^^^^^^^^^^^^ not found in the crate root
[01:29:03]     |
[01:29:03] 3   | use rustc_interface::util::get_codegen_backend;
[01:29:03]     |
[01:29:03] 
[01:29:03] 
[01:29:03] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:29:03]    --> src/librustc_driver/test.rs:109:16
[01:29:03]     |
[01:29:03] 109 |       let sess = session::build_session_(
[01:29:03] 110 | |         options,
[01:29:03] 111 | |         None,
[01:29:03] 112 | |         diagnostic_handler,
[01:29:03] 112 | |         diagnostic_handler,
[01:29:03] 113 | |         Lrc::new(SourceMap::new(FilePathMapping::empty())),
[01:29:03]     | |_____^ expected 5 parameters
[01:29:03] 
[01:29:03] error[E0061]: this function takes 12 parameters but 13 parameters were supplied
[01:29:03]    --> src/librustc_driver/test.rs:153:5
---
[01:29:04] 
[01:29:04] To learn more, run the command again with --verbose.
[01:29:04] 
[01:29:04] 
[01:29:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:29:04] 
[01:29:04] 
[01:29:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:04] Build completed unsuccessfully in 0:32:40
[01:29:04] Build completed unsuccessfully in 0:32:40
[01:29:04] make: *** [check] Error 1
[01:29:04] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9d61b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 00:58:55 UTC 2019
---
travis_time:end:18b8c84b:start=1549501138002366362,finish=1549501138007142866,duration=4776504
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1031bf9a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo travis_time:start:043ff9c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02a495a8
$ dmesg | grep -i kill
