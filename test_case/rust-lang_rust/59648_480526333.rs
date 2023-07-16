plain
travis_time:end:014eb62c:start=1554568903370964474,finish=1554568979359721164,duration=75988756690
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
[01:15:30] 
[01:15:30] running 9 tests
[01:15:30] iiiiiiiii
[01:15:30] 
[01:15:30]  finished in 0.156
[01:15:30] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:47] 
[01:15:47] running 121 tests
[01:16:14] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:18] i.i......iii.i.....ii
[01:16:18] 
[01:16:18]  finished in 31.314
[01:16:18] travis_fold:end:test_debuginfo

---
[01:40:00]     Finished release [optimized] target(s) in 17.24s
[01:40:00]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_data_structures-f644b6b7e75576ad
[01:40:00] 
[01:40:00] running 141 tests
[01:40:00] .........................................................F..........................F.F............. 100/141
[01:40:00] failures:
[01:40:00] 
[01:40:00] ---- owning_ref::tests::owning_ref::try_map2 stdout ----
[01:40:00] ---- owning_ref::tests::owning_ref::try_map2 stdout ----
[01:40:00] thread 'owning_ref::tests::owning_ref::try_map2' panicked at 'assertion failed: OwningRef::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_err()', src/librustc_data_structures/owning_ref/mod.rs:1484:13
[01:40:00] 
[01:40:00] ---- owning_ref::tests::owning_ref_mut::try_map2 stdout ----
[01:40:00] ---- owning_ref::tests::owning_ref_mut::try_map2 stdout ----
[01:40:00] thread 'owning_ref::tests::owning_ref_mut::try_map2' panicked at 'assertion failed: OwningRefMut::new(y).try_map_mut(|x|
[01:40:00]                                      x.downcast_mut::<i32>().ok_or(())).is_err()', src/librustc_data_structures/owning_ref/mod.rs:1881:13
[01:40:00] ---- owning_ref::tests::owning_ref_mut::try_map4 stdout ----
[01:40:00] ---- owning_ref::tests::owning_ref_mut::try_map4 stdout ----
[01:40:00] thread 'owning_ref::tests::owning_ref_mut::try_map4' panicked at 'assertion failed: OwningRefMut::new(y).try_map(|x| x.downcast_ref::<i32>().ok_or(())).is_err()', src/librustc_data_structures/owning_ref/mod.rs:1901:13
[01:40:00] 
[01:40:00] failures:
[01:40:00]     owning_ref::tests::owning_ref::try_map2
[01:40:00]     owning_ref::tests::owning_ref_mut::try_map2
[01:40:00]     owning_ref::tests::owning_ref_mut::try_map2
[01:40:00]     owning_ref::tests::owning_ref_mut::try_map4
[01:40:00] 
[01:40:00] test result: FAILED. 138 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:40:00] 
[01:40:00] error: test failed, to rerun pass '--lib'
[01:40:00] 
[01:40:00] 
[01:40:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:40:00] 
[01:40:00] 
[01:40:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:00] Build completed unsuccessfully in 0:36:47
[01:40:00] Build completed unsuccessfully in 0:36:47
[01:40:00] make: *** [check] Error 1
[01:40:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:28a4aaa8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 18:23:08 UTC 2019
