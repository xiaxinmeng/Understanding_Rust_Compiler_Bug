plain
travis_time:end:016dcfaf:start=1554252980999365318,finish=1554253056044412847,duration=75045047529
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:24] 
[01:24:24] running 9 tests
[01:24:24] iiiiiiiii
[01:24:24] 
[01:24:24]  finished in 0.162
[01:24:24] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:41] 
[01:24:41] running 121 tests
[01:25:07] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:12] i.i......iii.i.....ii
[01:25:12] 
[01:25:12]  finished in 31.000
[01:25:12] travis_fold:end:test_debuginfo

---
[01:48:23] travis_fold:start:test_stage1-rustc_data_structures
travis_time:start:test_stage1-rustc_data_structures
Testing rustc_data_structures stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:48:23]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_ok` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1474:13
[01:48:28]      |
[01:48:28] 1474 |             OwningRef::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_ok();
[01:48:28]      |
[01:48:28]      = note: `-D unused-must-use` implied by `-D warnings`
[01:48:28] 
[01:48:28] 
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_err` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1484:13
[01:48:28]      |
[01:48:28] 1484 |             OwningRef::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_err();
[01:48:28] 
[01:48:28] 
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_ok` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1871:13
[01:48:28]      |
[01:48:28] 1871 |             OwningRefMut::new(y).try_map_mut(|x| x.downcast_mut::<i32>().ok_or(())).is_ok();
[01:48:28] 
[01:48:28] 
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_err` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1881:13
[01:48:28]      |
[01:48:28] 1881 |             OwningRefMut::new(y).try_map_mut(|x| x.downcast_mut::<i32>().ok_or(())).is_err();
[01:48:28] 
[01:48:28] 
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_ok` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1891:13
[01:48:28]      |
[01:48:28] 1891 |             OwningRefMut::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_ok();
[01:48:28] 
[01:48:28] 
[01:48:28] error: unused return value of `std::result::Result::<T, E>::is_err` that must be used
[01:48:28]     --> src/librustc_data_structures/owning_ref/mod.rs:1901:13
[01:48:28]      |
[01:48:28] 1901 |             OwningRefMut::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_err();
[01:48:28] 
[01:48:28] error: aborting due to 6 previous errors
[01:48:28] 
[01:48:28] error: Could not compile `rustc_data_structures`.
[01:48:28] error: Could not compile `rustc_data_structures`.
[01:48:28] 
[01:48:28] To learn more, run the command again with --verbose.
[01:48:28] 
[01:48:28] 
[01:48:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:48:28] 
[01:48:28] 
[01:48:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:28] Build completed unsuccessfully in 0:36:15
[01:48:28] Build completed unsuccessfully in 0:36:15
[01:48:28] make: *** [check] Error 1
[01:48:28] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ecd3c58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 02:46:14 UTC 2019
---
travis_time:end:21cc9c05:start=1554259576147536937,finish=1554259576154532158,duration=6995221
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bd0e83d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fd46d6b
travis_time:start:1fd46d6b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06915e62
$ dmesg | grep -i kill
