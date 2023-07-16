plain
[00:03:35]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:36]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:36]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:41]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:46] warning: function is never used: `hashmap_random_keys`
[00:03:46]   --> libstd/sys/unix/rand.rs:14:1
[00:03:46]    |
[00:03:46] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:46]    |
[00:03:46]    = note: #[warn(dead_code)] on by default
[00:03:46] 
[00:03:46] warning: function is never used: `getrandom`
[00:03:46] warning: function is never used: `getrandom`
[00:03:46]   --> libstd/sys/unix/rand.rs:36:5
[00:03:46]    |
[00:03:46] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:46] 
[00:03:46] warning: function is never used: `getrandom_fill_bytes`
[00:03:46]   --> libstd/sys/unix/rand.rs:45:5
[00:03:46]    |
[00:03:46]    |
[00:03:46] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:46] 
[00:03:46] 
[00:03:46] warning: function is never used: `is_getrandom_available`
[00:03:46]   --> libstd/sys/unix/rand.rs:67:5
[00:03:46]    |
[00:03:46] 67 |     fn is_getrandom_available() -> bool {
[00:03:46] 
[00:03:46] warning: function is never used: `fill_bytes`
[00:03:46]   --> libstd/sys/unix/rand.rs:93:5
[00:03:46]    |
[00:03:46]    |
[00:03:46] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:46] 
[00:03:56]     Finished release [optimized] target(s) in 51.25s
[00:03:56] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:03:56] travis_fold:end:stage0-std
---
[01:11:13]      |
[01:11:13] 2762 |     use cell::RefCell;
[01:11:13]      |         ^^^^ Did you mean `core::cell`?
[01:11:13] 
[01:11:13] error[E0433]: failed to resolve. Maybe a missing `extern crate realstd;`?
[01:11:13]     --> liballoc/hash/map.rs:2764:9
[01:11:13]      |
[01:11:13] 2764 |     use realstd::collections::CollectionAllocErr::*;
[01:11:13]      |         ^^^^^^^ Maybe a missing `extern crate realstd;`?
[01:11:13] 
[01:11:13] error[E0433]: failed to resolve. Maybe a missing `extern crate realstd;`?
[01:11:13]     --> liballoc/hash/map.rs:2765:9
[01:11:13]      |
[01:11:13] 2765 |     use realstd::mem::size_of;
[01:11:13]      |         ^^^^^^^ Maybe a missing `extern crate realstd;`?
[01:11:13] error[E0432]: unresolved import `realstd`
[01:11:13]     --> liballoc/hash/map.rs:2766:9
[01:11:13]      |
[01:11:13] 2766 |     use realstd::usize;
[01:11:13] 2766 |     use realstd::usize;
[01:11:13]      |         ^^^^^^^ Maybe a missing `extern crate realstd;`?
[01:11:13] error[E0433]: failed to resolve. Use of undeclared type or module `Vec`
[01:11:13]     --> liballoc/hash/map.rs:2836:74
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2836 |     thread_local! { static DROP_VECTOR: RefCell<Vec<i32>> = RefCell::new(Vec::new()) }
[01:11:13]      |                                                                          ^^^ Use of undeclared type or module `Vec`
[01:11:13] 
[01:11:13] error[E0412]: cannot find type `Vec` in this scope
[01:11:13]     --> liballoc/hash/map.rs:2836:49
[01:11:13]      |
[01:11:13] 2836 |     thread_local! { static DROP_VECTOR: RefCell<Vec<i32>> = RefCell::new(Vec::new()) }
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::prelude::v1::Vec;
[01:11:13] 2759 |     use std::vec::Vec;
[01:11:13]      |
[01:11:13] 2759 |     use vec::Vec;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0412]: cannot find type `Vec` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3182:19
[01:11:13]      |
[01:11:13] 3182 |         let keys: Vec<_> = map.keys().cloned().collect();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::prelude::v1::Vec;
[01:11:13] 2759 |     use std::vec::Vec;
[01:11:13]      |
[01:11:13] 2759 |     use vec::Vec;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0412]: cannot find type `Vec` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3193:21
[01:11:13]      |
[01:11:13] 3193 |         let values: Vec<_> = map.values().cloned().collect();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::prelude::v1::Vec;
[01:11:13] 2759 |     use std::vec::Vec;
[01:11:13]      |
[01:11:13] 2759 |     use vec::Vec;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0412]: cannot find type `Vec` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3207:21
[01:11:13]      |
[01:11:13] 3207 |         let values: Vec<_> = map.values().cloned().collect();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::prelude::v1::Vec;
[01:11:13] 2759 |     use std::vec::Vec;
[01:11:13]      |
[01:11:13] 2759 |     use vec::Vec;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0425]: cannot find function `size_of` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3661:35
[01:11:13]      |
[01:11:13] 3661 |         let size_of_multiplier = (size_of::<usize>() + size_of::<(u8, u8)>()).next_power_of_two();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13] 2759 |     use std::intrinsics::size_of;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::mem::size_of;
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0425]: cannot find function `size_of` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3661:56
[01:11:13]      |
[01:11:13] 3661 |         let size_of_multiplier = (size_of::<usize>() + size_of::<(u8, u8)>()).next_power_of_two();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13] 2759 |     use std::intrinsics::size_of;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::mem::size_of;
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0425]: cannot find function `size_of` in this scope
[01:11:13]     --> liballoc/hash/map.rs:3668:12
[01:11:13]      |
[01:11:13] 3668 |         if size_of::<usize>() < 8 {
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13] 2759 |     use std::intrinsics::size_of;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 2759 |     use std::mem::size_of;
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0412]: cannot find type `Vec` in this scope
[01:11:13]     --> liballoc/hash/set.rs:1634:42
[01:11:13]      |
[01:11:13] 1634 |         let v = hs.into_iter().collect::<Vec<char>>();
[01:11:13] help: possible candidates are found in other modules, you can import them into scope
[01:11:13]      |
[01:11:13]      |
[01:11:13] 1404 |     use std::prelude::v1::Vec;
[01:11:13] 1404 |     use std::vec::Vec;
[01:11:13]      |
[01:11:13] 1404 |     use vec::Vec;
[01:11:13]      |
[01:11:13]      |
[01:11:13] 
[01:11:13] error[E0405]: cannot find trait `Hash` in module `hash`
[01:11:13]     --> liballoc/hash/set.rs:1728:20
[01:11:13]      |
[01:11:13] 1728 |         impl hash::Hash for Foo {
[01:11:13]      |                    ^^^^ not found in `hash`
[01:11:13] help: possible candidate is found in another module, you can import it into scope
[01:11:13] 1404 |     use std::hash::Hash;
[01:11:13]      |
[01:11:13] 
[01:11:13] 
[01:11:13] error[E0405]: cannot find trait `Hasher` in module `hash`
[01:11:13]     --> liballoc/hash/set.rs:1729:30
[01:11:13]      |
[01:11:13] 1729 |             fn hash<H: hash::Hasher>(&self, h: &mut H) {
[01:11:13]      |                              ^^^^^^ not found in `hash`
[01:11:13] help: possible candidate is found in another module, you can import it into scope
[01:11:13] 1404 |     use std::hash::Hasher;
[01:11:13]      |
[01:11:13] 
[01:11:13] 
[01:11:13] error: unused import: `realstd::collections::CollectionAllocErr::*`
[01:11:13]     --> liballoc/hash/map.rs:2764:9
[01:11:13]      |
[01:11:13] 2764 |     use realstd::collections::CollectionAllocErr::*;
[01:11:13]      |
[01:11:13]      = note: `-D unused-imports` implied by `-D warnings`
[01:11:13] 
[01:11:13] 
[01:11:13] error: unused import: `realstd::mem::size_of`
[01:11:13]     --> liballoc/hash/map.rs:2765:9
[01:11:13]      |
[01:11:13] 2765 |     use realstd::mem::size_of;
[01:11:13] 
[01:11:14] error: aborting due to 17 previous errors
[01:11:14] 
[01:11:14] Some errors occurred: E0405, E0412, E0425, E0432, E0433.
[01:11:14] Some errors occurred: E0405, E0412, E0425, E0432, E0433.
[01:11:14] For more information about an error, try `rustc --explain E0405`.
[01:11:14] error: Could not compile `alloc`.
[01:11:14] warning: build failed, waiting for other jobs to finish...
[01:11:45] error: build failed
[01:11:45] 
[01:11:45] 
[01:11:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:11:45] 
[01:11:45] 
[01:11:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:45] Build completed unsuccessfully in 0:26:11
[01:11:45] Build completed unsuccessfully in 0:26:11
[01:11:45] Makefile:58: recipe for target 'check' failed
[01:11:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11fe7b53
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:16b71040:start=1530117480368677570,finish=1530117480376193130,duration=7515560
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:072f3186
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0454a700
$ dmesg | grep -i kill
