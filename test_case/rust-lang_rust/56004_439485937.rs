plain
travis_time:end:059de964:start=1542387025816795292,finish=1542387028470251161,duration=2653455869
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:04:16]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:18]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:32]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:04:33]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:04:43] error[E0277]: the trait bound `&rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item>: rustc_data_structures::sync::ParallelIterator` is not satisfied
[00:04:43]    --> librustc/hir/mod.rs:764:17
[00:04:43]     |
[00:04:43] 764 |                 par_iter(&self.items).for_each(|(_, item)| {
[00:04:43]     |                 ^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `&rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item>`
[00:04:43]     |
[00:04:43]     = note: required because of the requirements on the impl of `rayon::iter::IntoParallelIterator` for `&rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item>`
[00:04:43]     = note: required by `rustc_data_structures::sync::par_iter`
[00:04:43] 
[00:04:43] error[E0599]: no method named `for_each` found for type `&rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item>` in the current scope
[00:04:43]    --> librustc/hir/mod.rs:764:39
[00:04:43]     |
[00:04:43] 764 |                 par_iter(&self.items).for_each(|(_, item)| {
[00:04:43]     |
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]             `&mut &rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item> : std::iter::Iterator`
[00:04:43]             `&mut rustc_data_structures::sorted_map::HybridSortedMap<syntax::ast::NodeId, hir::Item> : std::iter::Iterator`
[00:04:43] 
[00:04:43] error[E0277]: the trait bound `&rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem>: rustc_data_structures::sync::ParallelIterator` is not satisfied
[00:04:43]    --> librustc/hir/mod.rs:770:17
[00:04:43]     |
[00:04:43] 770 |                 par_iter(&self.trait_items).for_each(|(_, trait_item)| {
[00:04:43]     |                 ^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `&rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem>`
[00:04:43]     |
[00:04:43]     = note: required because of the requirements on the impl of `rayon::iter::IntoParallelIterator` for `&rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem>`
[00:04:43]     = note: required by `rustc_data_structures::sync::par_iter`
[00:04:43] 
[00:04:43] error[E0599]: no method named `for_each` found for type `&rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem>` in the current scope
[00:04:43]    --> librustc/hir/mod.rs:770:45
[00:04:43]     |
[00:04:43] 770 |                 par_iter(&self.trait_items).for_each(|(_, trait_item)| {
[00:04:43]     |
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]             `&mut &rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem> : std::iter::Iterator`
[00:04:43]             `&mut rustc_data_structures::sorted_map::HybridSortedMap<hir::TraitItemId, hir::TraitItem> : std::iter::Iterator`
[00:04:43] 
[00:04:43] error[E0277]: the trait bound `&rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem>: rustc_data_structures::sync::ParallelIterator` is not satisfied
[00:04:43]    --> librustc/hir/mod.rs:776:17
[00:04:43]     |
[00:04:43] 776 |                 par_iter(&self.impl_items).for_each(|(_, impl_item)| {
[00:04:43]     |                 ^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `&rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem>`
[00:04:43]     |
[00:04:43]     = note: required because of the requirements on the impl of `rayon::iter::IntoParallelIterator` for `&rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem>`
[00:04:43]     = note: required by `rustc_data_structures::sync::par_iter`
[00:04:43] 
[00:04:43] error[E0599]: no method named `for_each` found for type `&rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem>` in the current scope
[00:04:43]    --> librustc/hir/mod.rs:776:44
[00:04:43]     |
[00:04:43] 776 |                 par_iter(&self.impl_items).for_each(|(_, impl_item)| {
[00:04:43]     |
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:04:43]             `&mut &rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem> : std::iter::Iterator`
[00:04:43]             `&mut rustc_data_structures::sorted_map::HybridSortedMap<hir::ImplItemId, hir::ImplItem> : std::iter::Iterator`
[00:04:56] error: aborting due to 6 previous errors
[00:04:56] 
[00:04:56] Some errors occurred: E0277, E0599.
[00:04:56] For more information about an error, try `rustc --explain E0277`.
[00:04:56] For more information about an error, try `rustc --explain E0277`.
[00:04:56] error: Could not compile `rustc`.
[00:04:56] 
[00:04:56] To learn more, run the command again with --verbose.
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:56] expected success, got: exit code: 101
[00:04:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:04:56] travis_fold:end:stage0-rustc

[00:04:56] travis_time:end:stage0-rustc:start=1542387272767922939,finish=1542387334749473077,duration=61981550138

---
travis_time:end:07d8a56b:start=1542387335426506998,finish=1542387335432791561,duration=6284563
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01dcc29b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1aaa8f9c
travis_time:start:1aaa8f9c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:135a7619
$ dmesg | grep -i kill
