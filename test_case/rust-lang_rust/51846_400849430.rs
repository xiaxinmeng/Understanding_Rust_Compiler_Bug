plain
[00:03:47]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:48]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:48]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:53]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:57] warning: function is never used: `hashmap_random_keys`
[00:03:57]   --> libstd/sys/unix/rand.rs:14:1
[00:03:57]    |
[00:03:57] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:57]    |
[00:03:57]    = note: #[warn(dead_code)] on by default
[00:03:57] 
[00:03:57] warning: function is never used: `getrandom`
[00:03:57] warning: function is never used: `getrandom`
[00:03:57]   --> libstd/sys/unix/rand.rs:36:5
[00:03:57]    |
[00:03:57] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:57] 
[00:03:57] warning: function is never used: `getrandom_fill_bytes`
[00:03:57]   --> libstd/sys/unix/rand.rs:45:5
[00:03:57]    |
[00:03:57]    |
[00:03:57] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:57] 
[00:03:57] 
[00:03:57] warning: function is never used: `is_getrandom_available`
[00:03:57]   --> libstd/sys/unix/rand.rs:67:5
[00:03:57]    |
[00:03:57] 67 |     fn is_getrandom_available() -> bool {
[00:03:57] 
[00:03:57] warning: function is never used: `fill_bytes`
[00:03:57]   --> libstd/sys/unix/rand.rs:93:5
[00:03:57]    |
[00:03:57]    |
[00:03:57] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:57] 
[00:04:06]     Finished release [optimized] target(s) in 47.49s
[00:04:06] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:06] travis_fold:end:stage0-std
---
[01:13:37] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:38]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:13:42] error[E0152]: duplicate lang item found: `hashmap_random_keys`.
[01:13:42]     |
[01:13:42]     |
[01:13:42] 445 | / fn hashmap_random_keys() -> (u64, u64) {
[01:13:42] 446 | |     use sys;
[01:13:42] 447 | |     use cell::Cell;
[01:13:42] ...   |
[01:13:42] 468 | |     })
[01:13:42] 469 | | }
[01:13:42]     | |_^
[01:13:42]     | |_^
[01:13:42]     |
[01:13:42]     = note: first defined in crate `std`.
[01:13:42] error: aborting due to previous error
[01:13:42] 
[01:13:42] For more information about this error, try `rustc --explain E0152`.
[01:13:42] error: Could not compile `std`.
[01:13:42] error: Could not compile `std`.
[01:13:42] 
[01:13:42] To learn more, run the command again with --verbose.
[01:13:42] 
[01:13:42] 
[01:13:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:13:42] 
[01:13:42] 
[01:13:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:42] Build completed unsuccessfully in 0:31:36
[01:13:42] Build completed unsuccessfully in 0:31:36
[01:13:42] make: *** [check] Error 1
[01:13:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ed06e54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
