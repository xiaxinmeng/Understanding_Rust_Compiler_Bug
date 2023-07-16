plain
[00:00:52] configure: rust.quiet-tests     := True
---
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/boxed.rs:243:13
[00:03:45]     |
[00:03:45] 243 | unsafe impl<#[may_dangle] T: ?Sized> Drop for Box<T> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/arc.rs:903:13
[00:03:45]     |
[00:03:45] 903 | unsafe impl<#[may_dangle] T: ?Sized> Drop for Arc<T> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/rc.rs:808:13
[00:03:45]     |
[00:03:45] 808 | unsafe impl<#[may_dangle] T: ?Sized> Drop for Rc<T> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/raw_vec.rs:712:13
[00:03:45]     |
[00:03:45] 712 | unsafe impl<#[may_dangle] T, A: Alloc> Drop for RawVec<T, A> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/btree/map.rs:141:13
[00:03:45]     |
[00:03:45] 141 | unsafe impl<#[may_dangle] K, #[may_dangle] V> Drop for BTreeMap<K, V> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/btree/map.rs:141:30
[00:03:45]     |
[00:03:45] 141 | unsafe impl<#[may_dangle] K, #[may_dangle] V> Drop for BTreeMap<K, V> {
[00:03:45]     |                              ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]    --> liballoc/linked_list.rs:791:13
[00:03:45]     |
[00:03:45] 791 | unsafe impl<#[may_dangle] T> Drop for LinkedList<T> {
[00:03:45]     |             ^^^^^^^^^^^^^
[00:03:45]     |
[00:03:45]     = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]     --> liballoc/vec.rs:2129:13
[00:03:45]      |
[00:03:45] 2129 | unsafe impl<#[may_dangle] T> Drop for Vec<T> {
[00:03:45]      |             ^^^^^^^^^^^^^
[00:03:45]      |
[00:03:45]      = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]     --> liballoc/vec.rs:2427:13
[00:03:45]      |
[00:03:45] 2427 | unsafe impl<#[may_dangle] T> Drop for IntoIter<T> {
[00:03:45]      |             ^^^^^^^^^^^^^
[00:03:45]      |
[00:03:45]      = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error[E0658]: attributes on type parameter bindings are experimental (see issue #34761)
[00:03:45]   --> liballoc/vec_deque.rs:76:13
[00:03:45]    |
[00:03:45] 76 | unsafe impl<#[may_dangle] T> Drop for VecDeque<T> {
[00:03:45]    |             ^^^^^^^^^^^^^
[00:03:45]    |
[00:03:45]    = help: add #![feature(generic_param_attrs)] to the crate attributes to enable
[00:03:45]
[00:03:45] error: aborting due to 10 previous errors
[00:03:45]
[00:03:45] For more information about this error, try `rustc --explain E0658`.
[00:03:45] error: Could not compile `alloc`.
[00:03:45]
[00:03:45] Caused by:
[00:03:45]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=29528069f2a9d6eb -C extra-filename=-29528069f2a9d6eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-1b8789e893adb899.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-305fbd4ba0177b74.rlib` (exit code: 101)
[00:03:45] warning: build failed, waiting for other jobs to finish...
[00:03:51] error: build failed
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:51] expected success, got: exit code: 101
[00:03:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1075:9
[00:03:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:51] travis_fold:end:stage0-std
[00:03:51] travis_time:end:stage0-std:start=1522937352694087486,finish=1522937388498677220,duration=35804589734
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:37
[00:03:51] make: *** [tidy] Error 1
[00:03:51] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
