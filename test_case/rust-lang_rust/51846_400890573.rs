plain
[00:03:38]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:39]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:39]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:44]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:49] warning: function is never used: `hashmap_random_keys`
[00:03:49]   --> libstd/sys/unix/rand.rs:14:1
[00:03:49]    |
[00:03:49] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:49]    |
[00:03:49]    = note: #[warn(dead_code)] on by default
[00:03:49] 
[00:03:49] warning: function is never used: `getrandom`
[00:03:49] warning: function is never used: `getrandom`
[00:03:49]   --> libstd/sys/unix/rand.rs:36:5
[00:03:49]    |
[00:03:49] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:49] 
[00:03:49] warning: function is never used: `getrandom_fill_bytes`
[00:03:49]   --> libstd/sys/unix/rand.rs:45:5
[00:03:49]    |
[00:03:49]    |
[00:03:49] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:49] 
[00:03:49] 
[00:03:49] warning: function is never used: `is_getrandom_available`
[00:03:49]   --> libstd/sys/unix/rand.rs:67:5
[00:03:49]    |
[00:03:49] 67 |     fn is_getrandom_available() -> bool {
[00:03:49] 
[00:03:49] warning: function is never used: `fill_bytes`
[00:03:49]   --> libstd/sys/unix/rand.rs:93:5
[00:03:49]    |
[00:03:49]    |
[00:03:49] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:49] 
[00:03:58]     Finished release [optimized] target(s) in 47.86s
[00:03:58] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:03:58] travis_fold:end:stage0-std
---
[01:23:51] travis_fold:end:stage0-linkchecker

[01:23:51] travis_time:end:stage0-linkchecker:start=1530152767380550258,finish=1530152770293424419,duration=2912874161

[01:24:07] alloc/hash_map/index.html:8: broken link - hash/trait.Hasher.html
[01:24:07] alloc/hash_map/struct.DefaultHasher.html:1: broken link - hash/trait.Hasher.html
[01:24:07] alloc/hash_map/struct.RandomState.html:3: broken link - hash/trait.Hasher.html
[01:24:08] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:24:08] 
[01:24:08] 
[01:24:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:24:08] expected success, got: exit code: 101
[01:24:08] expected success, got: exit code: 101
[01:24:08] 
[01:24:08] 
[01:24:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:08] Build completed unsuccessfully in 0:40:50
[01:24:08] make: *** [check] Error 1
[01:24:08] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b60a270
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
