plain
travis_time:end:013df1ec:start=1550057154270271884,finish=1550057155337474030,duration=1067202146
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:39]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:06:39]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:06:39]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:07:00]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:07:03] error[E0277]: the trait bound `T: core::cmp::Ord` is not satisfied
[00:07:03]     |
[00:07:03]     |
[00:07:03] 230 | impl<T: fmt::Debug> fmt::Debug for PeekMut<'_, T> {
[00:07:03]     |                     ^^^^^^^^^^ the trait `core::cmp::Ord` is not implemented for `T`
[00:07:03]     |
[00:07:03]     = help: consider adding a `where T: core::cmp::Ord` bound
[00:07:03] note: required by `collections::binary_heap::PeekMut`
[00:07:03]     |
[00:07:03]     |
[00:07:03] 224 | pub struct PeekMut<'a, T: 'a + Ord> {
[00:07:03] 
[00:07:03] 
[00:07:03] error[E0277]: the trait bound `T: core::cmp::Ord` is not satisfied
[00:07:03]     |
[00:07:03]     |
[00:07:03] 248 | impl<T> Deref for PeekMut<'_, T> {
[00:07:03]     |
[00:07:03]     |
[00:07:03] 224 | pub struct PeekMut<'a, T: 'a + Ord> {
[00:07:03] 
[00:07:03] 
[00:07:03] error[E0277]: the trait bound `T: core::cmp::Ord` is not satisfied
[00:07:03]     |
[00:07:03]     |
[00:07:03] 231 | /     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
[00:07:03] 232 | |         f.debug_tuple("PeekMut")
[00:07:03] 233 | |          .field(&self.heap.data[0])
[00:07:03] 234 | |          .finish()
[00:07:03] 235 | |     }
[00:07:03]     | |_____^ the trait `core::cmp::Ord` is not implemented for `T`
[00:07:03]     |
[00:07:03]     = help: consider adding a `where T: core::cmp::Ord` bound
[00:07:03] note: required by `collections::binary_heap::PeekMut`
[00:07:03]     |
[00:07:03]     |
[00:07:03] 224 | pub struct PeekMut<'a, T: 'a + Ord> {
[00:07:03] 
[00:07:03] 
[00:07:03] error[E0277]: the trait bound `T: core::cmp::Ord` is not satisfied
[00:07:03]     |
[00:07:03]     |
[00:07:03] 250 | /     fn deref(&self) -> &T {
[00:07:03] 251 | |         debug_assert!(!self.heap.is_empty());
[00:07:03] 252 | |         // SAFE: PeekMut is only instantiated for non-empty heaps
[00:07:03] 253 | |         unsafe { self.heap.data.get_unchecked(0) }
[00:07:03] 254 | |     }
[00:07:03]     | |_____^ the trait `core::cmp::Ord` is not implemented for `T`
[00:07:03]     |
[00:07:03]     = help: consider adding a `where T: core::cmp::Ord` bound
[00:07:03] note: required by `collections::binary_heap::PeekMut`
[00:07:03]     |
[00:07:03]     |
[00:07:03] 224 | pub struct PeekMut<'a, T: 'a + Ord> {
[00:07:03] 
[00:07:03] 
[00:07:03] error[E0277]: the trait bound `T: core::cmp::Ord` is not satisfied
[00:07:03]     |
[00:07:03] 259 | /     fn deref_mut(&mut self) -> &mut T {
[00:07:03] 259 | /     fn deref_mut(&mut self) -> &mut T {
[00:07:03] 260 | |         debug_assert!(!self.heap.is_empty());
[00:07:03] 261 | |         // SAFE: PeekMut is only instantiated for non-empty heaps
[00:07:03] 262 | |         unsafe { self.heap.data.get_unchecked_mut(0) }
[00:07:03] 263 | |     }
[00:07:03]     | |_____^ the trait `core::cmp::Ord` is not implemented for `T`
[00:07:03]     |
[00:07:03]     = help: consider adding a `where T: core::cmp::Ord` bound
[00:07:03] note: required by `collections::binary_heap::PeekMut`
[00:07:03]     |
[00:07:03]     |
[00:07:03] 224 | pub struct PeekMut<'a, T: 'a + Ord> {
[00:07:03] 
[00:07:03] error: aborting due to 6 previous errors
[00:07:03] 
[00:07:03] For more information about this error, try `rustc --explain E0277`.
[00:07:03] For more information about this error, try `rustc --explain E0277`.
[00:07:03] error: Could not compile `alloc`.
[00:07:03] 
[00:07:03] To learn more, run the command again with --verbose.
[00:07:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:07:03] expected success, got: exit code: 101
[00:07:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:03] Build completed unsuccessfully in 0:00:37
[00:07:04] make: *** [all] Error 1
[00:07:04] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ad1c6a7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 11:33:09 UTC 2019
---
travis_time:end:2b246322:start=1550057590479020959,finish=1550057590485513088,duration=6492129
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12e67d0e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16992a6a
travis_time:start:16992a6a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22e36e6b
$ dmesg | grep -i kill
