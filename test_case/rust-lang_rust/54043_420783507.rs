plain
[00:43:07] 
[00:43:07] warning: `[]` cannot be resolved, ignoring it...
[00:43:07]     --> libstd/collections/hash/map.rs:1862:40
[00:43:07]      |
[00:43:07] 1862 | /// See the [`HashMap::raw_entry_mut`][] docs for usage examples.
[00:43:07]      |
[00:43:07]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:07] 
[00:43:07] warning: `[]` cannot be resolved, ignoring it...
[00:43:07] warning: `[]` cannot be resolved, ignoring it...
[00:43:07]     --> libstd/collections/hash/map.rs:1906:36
[00:43:07]      |
[00:43:07] 1906 | /// See the [`HashMap::raw_entry`][] docs for usage examples.
[00:43:07]      |
[00:43:07]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:07] 
[00:43:07] warning: `[Seek::seek_relative]` cannot be resolved, ignoring it...
---
[01:11:01] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:02]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:11:04] error[E0432]: unresolved import `super::RawEntry`
[01:11:04]      |
[01:11:04]      |
[01:11:04] 4273 |         use super::RawEntry::{Occupied, Vacant};
[01:11:04]      |                    ^^^^^^^^ Could not find `RawEntry` in `super`
[01:11:04] 
[01:11:09] error[E0599]: no method named `search_by` found for type `collections::hash::map::RawEntryBuilder<'_, i32, i32, collections::hash::map::RandomState>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 1908 | pub struct RawEntryBuilder<'a, K: 'a, V: 'a, S: 'a> {
[01:11:09]      | --------------------------------------------------- method `search_by` not found for this
[01:11:09] ...
[01:11:09] 4280 |         match map.raw_entry().search_by(&1) {
[01:11:09] 
[01:11:09] 
[01:11:09] error[E0599]: no method named `raw_entry_immut` found for type `collections::hash::map::HashMap<i32, i32>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 415  | pub struct HashMap<K, V, S = RandomState> {
[01:11:09]      | ----------------------------------------- method `raw_entry_immut` not found for this
[01:11:09] ...
[01:11:09] 4287 |         assert_eq!(map.raw_entry_immut().hash_with(|mut h| {
[01:11:09]      |
[01:11:09]      |
[01:11:09]      = help: did you mean `raw_entry_mut`?
[01:11:09] error[E0599]: no method named `hash` found for type `i32` in the current scope
[01:11:09]     --> libstd/collections/hash/map.rs:4288:18
[01:11:09]      |
[01:11:09]      |
[01:11:09] 4288 |             1i32.hash(&mut h);
[01:11:09]      |
[01:11:09]      = help: items from traits can only be used if the trait is in scope
[01:11:09]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:09]              `use core::hash::Hash;`
[01:11:09]              `use core::hash::Hash;`
[01:11:09] 
[01:11:09] error[E0599]: no method named `hash_by` found for type `collections::hash::map::RawEntryBuilder<'_, i32, i32, collections::hash::map::RandomState>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 1908 | pub struct RawEntryBuilder<'a, K: 'a, V: 'a, S: 'a> {
[01:11:09]      | --------------------------------------------------- method `hash_by` not found for this
[01:11:09] ...
[01:11:09] 4296 |         match map.raw_entry().hash_by(&2).search_by(&2) {
[01:11:09] 
[01:11:09] 
[01:11:09] error[E0599]: no method named `raw_entry_immut` found for type `collections::hash::map::HashMap<i32, i32>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 415  | pub struct HashMap<K, V, S = RandomState> {
[01:11:09]      | ----------------------------------------- method `raw_entry_immut` not found for this
[01:11:09] ...
[01:11:09] 4304 |         assert_eq!(map.raw_entry_immut().search_by(&2).unwrap(), (&2, &200));
[01:11:09]      |
[01:11:09]      |
[01:11:09]      = help: did you mean `raw_entry_mut`?
[01:11:09] 
[01:11:09] error[E0599]: no method named `hash_with` found for type `collections::hash::map::RawEntryBuilder<'_, i32, i32, collections::hash::map::RandomState>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 1908 | pub struct RawEntryBuilder<'a, K: 'a, V: 'a, S: 'a> {
[01:11:09]      | --------------------------------------------------- method `hash_with` not found for this
[01:11:09] ...
[01:11:09] 4308 |         match map.raw_entry().hash_with(|mut h| {
[01:11:09] 
[01:11:09] error[E0599]: no method named `hash` found for type `i32` in the current scope
[01:11:09]     --> libstd/collections/hash/map.rs:4309:18
[01:11:09]      |
[01:11:09]      |
[01:11:09] 4309 |             3i32.hash(&mut h);
[01:11:09]      |
[01:11:09]      = help: items from traits can only be used if the trait is in scope
[01:11:09]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:09]              `use core::hash::Hash;`
[01:11:09]              `use core::hash::Hash;`
[01:11:09] 
[01:11:09] error[E0599]: no method named `raw_entry_immut` found for type `collections::hash::map::HashMap<i32, i32>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 415  | pub struct HashMap<K, V, S = RandomState> {
[01:11:09]      | ----------------------------------------- method `raw_entry_immut` not found for this
[01:11:09] ...
[01:11:09] 4317 |         assert_eq!(map.raw_entry_immut().search_by(&3), None);
[01:11:09]      |
[01:11:09]      |
[01:11:09]      = help: did you mean `raw_entry_mut`?
[01:11:09] 
[01:11:09] error[E0599]: no method named `search_by` found for type `collections::hash::map::RawEntryBuilder<'_, i32, i32, collections::hash::map::RandomState>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 1908 | pub struct RawEntryBuilder<'a, K: 'a, V: 'a, S: 'a> {
[01:11:09]      | --------------------------------------------------- method `search_by` not found for this
[01:11:09] ...
[01:11:09] 4322 |         match map.raw_entry().search_by(&10) {
[01:11:09] 
[01:11:09] 
[01:11:09] error[E0599]: no method named `raw_entry_immut` found for type `collections::hash::map::HashMap<i32, i32>` in the current scope
[01:11:09]      |
[01:11:09]      |
[01:11:09] 415  | pub struct HashMap<K, V, S = RandomState> {
[01:11:09]      | ----------------------------------------- method `raw_entry_immut` not found for this
[01:11:09] ...
[01:11:09] 4328 |         assert_eq!(map.raw_entry_immut().hash_by(&10).search_by(&10).unwrap(), (&10, &1000));
[01:11:09]      |
[01:11:09]      |
[01:11:09]      = help: did you mean `raw_entry_mut`?
[01:11:16] error: aborting due to 11 previous errors
[01:11:16] 
[01:11:16] Some errors occurred: E0432, E0599.
[01:11:16] For more information about an error, try `rustc --explain E0432`.
[01:11:16] For more information about an error, try `rustc --explain E0432`.
[01:11:16] error: Could not compile `std`.
[01:11:16] 
[01:11:16] To learn more, run the command again with --verbose.
[01:11:16] 
[01:11:16] 
[01:11:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:11:16] 
[01:11:16] 
[01:11:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:16] Build completed unsuccessfully in 0:27:26
[01:11:16] Build completed unsuccessfully in 0:27:26
[01:11:16] make: *** [check] Error 1
[01:11:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24240f30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:069dffc6:start=1536783582137457295,finish=1536783582232386178,duration=94928883
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14213f9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0164b185
$ dmesg | grep -i kill
