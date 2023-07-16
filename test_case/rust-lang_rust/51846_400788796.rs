plain
[00:03:52]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:53]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:53]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:58]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:02] warning: function is never used: `hashmap_random_keys`
[00:04:02]   --> libstd/sys/unix/rand.rs:14:1
[00:04:02]    |
[00:04:02] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:04:02]    |
[00:04:02]    = note: #[warn(dead_code)] on by default
[00:04:02] 
[00:04:02] warning: function is never used: `getrandom`
[00:04:02] warning: function is never used: `getrandom`
[00:04:02]   --> libstd/sys/unix/rand.rs:36:5
[00:04:02]    |
[00:04:02] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:04:02] 
[00:04:02] warning: function is never used: `getrandom_fill_bytes`
[00:04:02]   --> libstd/sys/unix/rand.rs:45:5
[00:04:02]    |
[00:04:02]    |
[00:04:02] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:04:02] 
[00:04:02] 
[00:04:02] warning: function is never used: `is_getrandom_available`
[00:04:02]   --> libstd/sys/unix/rand.rs:67:5
[00:04:02]    |
[00:04:02] 67 |     fn is_getrandom_available() -> bool {
[00:04:02] 
[00:04:02] warning: function is never used: `fill_bytes`
[00:04:02]   --> libstd/sys/unix/rand.rs:93:5
[00:04:02]    |
[00:04:02]    |
[00:04:02] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:04:02] 
[00:04:11]     Finished release [optimized] target(s) in 47.03s
[00:04:11] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:11] travis_fold:end:stage0-std
---
[01:05:01]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:05:02] error[E0432]: unresolved import `std::rand`
[01:05:02]     --> liballoc/hash/map.rs:2763:14
[01:05:02]      |
[01:05:02] 2763 |     use std::rand::{thread_rng, Rng};
[01:05:02]      |              ^^^^ Could not find `rand` in `std`
[01:05:02] 
[01:05:02] error[E0412]: cannot find type `Vec` in this scope
[01:05:02]     --> liballoc/hash/set.rs:1635:42
[01:05:02]      |
[01:05:02] 1635 |         let v = hs.into_iter().collect::<Vec<char>>();
[01:05:02] help: possible candidates are found in other modules, you can import them into scope
[01:05:02]      |
[01:05:02]      |
[01:05:02] 1404 |     use std::prelude::v1::Vec;
[01:05:02] 1404 |     use std::vec::Vec;
[01:05:02]      |
[01:05:02] 1404 |     use vec::Vec;
[01:05:02]      |
[01:05:02]      |
[01:05:02] 
[01:05:02] error[E0405]: cannot find trait `Hash` in module `hash`
[01:05:02]     --> liballoc/hash/set.rs:1729:20
[01:05:02]      |
[01:05:02] 1729 |         impl hash::Hash for Foo {
[01:05:02]      |                    ^^^^ not found in `hash`
[01:05:02] help: possible candidate is found in another module, you can import it into scope
[01:05:02] 1404 |     use std::hash::Hash;
[01:05:02]      |
[01:05:02] 
[01:05:02] 
[01:05:02] error[E0405]: cannot find trait `Hasher` in module `hash`
[01:05:02]     --> liballoc/hash/set.rs:1730:30
[01:05:02]      |
[01:05:02] 1730 |             fn hash<H: hash::Hasher>(&self, h: &mut H) {
[01:05:02]      |                              ^^^^^^ not found in `hash`
[01:05:02] help: possible candidate is found in another module, you can import it into scope
[01:05:02] 1404 |     use std::hash::Hasher;
[01:05:02]      |
[01:05:02] 
[01:05:02] error: unused import: `Rng`
[01:05:02] error: unused import: `Rng`
[01:05:02]     --> liballoc/hash/map.rs:2763:33
[01:05:02]      |
[01:05:02] 2763 |     use std::rand::{thread_rng, Rng};
[01:05:02]      |
[01:05:02]      = note: `-D unused-imports` implied by `-D warnings`
[01:05:02] 
[01:05:02] error: unused import: `core::hash`
---
[01:05:02] warning: build failed, waiting for other jobs to finish...
[01:05:30] error: build failed
[01:05:30] 
[01:05:30] 
[01:05:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:05:30] 
[01:05:30] 
[01:05:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:30] Build completed unsuccessfully in 0:23:28
[01:05:30] Build completed unsuccessfully in 0:23:28
[01:05:30] Makefile:58: recipe for target 'check' failed
[01:05:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d01e7f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
