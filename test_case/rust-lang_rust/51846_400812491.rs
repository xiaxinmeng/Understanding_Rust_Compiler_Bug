plain
[00:03:33]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:34]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:35]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:39]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:44] warning: function is never used: `hashmap_random_keys`
[00:03:44]   --> libstd/sys/unix/rand.rs:14:1
[00:03:44]    |
[00:03:44] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:44]    |
[00:03:44]    = note: #[warn(dead_code)] on by default
[00:03:44] 
[00:03:44] warning: function is never used: `getrandom`
[00:03:44] warning: function is never used: `getrandom`
[00:03:44]   --> libstd/sys/unix/rand.rs:36:5
[00:03:44]    |
[00:03:44] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:44] 
[00:03:44] warning: function is never used: `getrandom_fill_bytes`
[00:03:44]   --> libstd/sys/unix/rand.rs:45:5
[00:03:44]    |
[00:03:44]    |
[00:03:44] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:44] 
[00:03:44] 
[00:03:44] warning: function is never used: `is_getrandom_available`
[00:03:44]   --> libstd/sys/unix/rand.rs:67:5
[00:03:44]    |
[00:03:44] 67 |     fn is_getrandom_available() -> bool {
[00:03:44] 
[00:03:44] warning: function is never used: `fill_bytes`
[00:03:44]   --> libstd/sys/unix/rand.rs:93:5
[00:03:44]    |
[00:03:44]    |
[00:03:44] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:44] 
[00:03:53]     Finished release [optimized] target(s) in 47.48s
[00:03:53] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:03:53] travis_fold:end:stage0-std
---
[01:07:27] 
[01:07:28] 
[01:07:28] running 486 tests
[01:07:47] ....................................................................................................
[01:08:12] ........................................i......................F.F.................................F
[01:08:50] ....................................................................................................
[01:09:08] ......................................................................................
[01:09:08] failures:
[01:09:08] 
[01:09:08] 
[01:09:08] ---- hash/map.rs - hash::map::HashMap<K, V, RandomState>::new (line 643) stdout ----
[01:09:08] error: variable does not need to be mutable
[01:09:08]  --> hash/map.rs:645:5
[01:09:08]   |
[01:09:08] 5 | let mut map: HashMap<&str, i32> = HashMap::new();
[01:09:08]   |     ----^^^
[01:09:08]   |     |
[01:09:08]   |     help: remove this `mut`
[01:09:08] note: lint level defined here
[01:09:08]  --> hash/map.rs:642:9
[01:09:08]   |
[01:09:08] 2 | #![deny(warnings)]
[01:09:08] 2 | #![deny(warnings)]
[01:09:08]   |         ^^^^^^^^
[01:09:08]   = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[01:09:08] thread 'hash/map.rs - hash::map::HashMap<K, V, RandomState>::new (line 643)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:09:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:09:08] 
[01:09:08] ---- hash/map.rs - hash::map::HashMap<K, V, RandomState>::with_capacity (line 660) stdout ----
[01:09:08] ---- hash/map.rs - hash::map::HashMap<K, V, RandomState>::with_capacity (line 660) stdout ----
[01:09:08] error: variable does not need to be mutable
[01:09:08]  --> hash/map.rs:662:5
[01:09:08]   |
[01:09:08] 5 | let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
[01:09:08]   |     ----^^^
[01:09:08]   |     |
[01:09:08]   |     help: remove this `mut`
[01:09:08] note: lint level defined here
[01:09:08]  --> hash/map.rs:659:9
[01:09:08]   |
[01:09:08] 2 | #![deny(warnings)]
[01:09:08] 2 | #![deny(warnings)]
[01:09:08]   |         ^^^^^^^^
[01:09:08]   = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[01:09:08] thread 'hash/map.rs - hash::map::HashMap<K, V, RandomState>::with_capacity (line 660)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:09:08] 
[01:09:08] ---- hash/map.rs - hash::map::OccupiedEntry<'a, K, V>::replace_key (line 2409) stdout ----
[01:09:08] ---- hash/map.rs - hash::map::OccupiedEntry<'a, K, V>::replace_key (line 2409) stdout ----
[01:09:08] error: variable does not need to be mutable
[01:09:08]  --> hash/map.rs:2415:5
[01:09:08]   |
[01:09:08] 9 | let mut known_strings: Vec<Rc<String>> = Vec::new();
[01:09:08]   |     ----^^^^^^^^^^^^^
[01:09:08]   |     |
[01:09:08]   |     help: remove this `mut`
[01:09:08] note: lint level defined here
[01:09:08]  --> hash/map.rs:2408:9
[01:09:08]   |
[01:09:08] 2 | #![deny(warnings)]
[01:09:08] 2 | #![deny(warnings)]
[01:09:08]   |         ^^^^^^^^
[01:09:08]   = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[01:09:08] thread 'hash/map.rs - hash::map::OccupiedEntry<'a, K, V>::replace_key (line 2409)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:09:08] 
[01:09:08] 
[01:09:08] failures:
---
[01:09:08] 
[01:09:08] error: test failed, to rerun pass '--doc'
[01:09:08] 
[01:09:08] 
[01:09:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:09:08] 
[01:09:08] 
[01:09:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:08] Build completed unsuccessfully in 0:26:09
[01:09:08] Build completed unsuccessfully in 0:26:09
[01:09:08] make: *** [check] Error 1
[01:09:08] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d64be65
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:072699e7:start=1530130099565560029,finish=1530130099573453675,duration=7893646
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a9717e0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c532812
$ dmesg | grep -i kill
