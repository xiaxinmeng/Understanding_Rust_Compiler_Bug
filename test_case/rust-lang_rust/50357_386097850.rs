plain
[00:03:25]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:25]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:27]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:27]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:27] error[E0606]: casting `usize` as `*mut arc::ArcInner<T>` is invalid
[00:03:27]     --> liballoc/arc.rs:1037:45
[00:03:27]      |
[00:03:27] 1037 |         let inner = if self.ptr.as_ptr() == WEAK_EMPTY as *mut _ {
[00:03:27] 
[00:03:27] 
[00:03:27] error[E0277]: the trait bound `T: core::marker::Sized` is not satisfied
[00:03:27]     --> liballoc/arc.rs:1090:20
[00:03:27]      |
[00:03:27] 1090 |             return Weak::new();
[00:03:27]      |                    ^^^^^^^^^ `T` does not have a constant size known at compile-time
[00:03:27]      |
[00:03:27]      = help: the trait `core::marker::Sized` is not implemented for `T`
[00:03:27]      = help: consider adding a `where T: core::marker::Sized` bound
[00:03:27] note: required by `<arc::Weak<T>>::new`
[00:03:27]     --> liballoc/arc.rs:997:5
[00:03:27]      |
[00:03:27] 997  |     pub fn new() -> Weak<T> {
[00:03:27] 
[00:03:27] 
[00:03:27] error[E0606]: casting `usize` as `*mut arc::ArcInner<T>` is invalid
[00:03:27]     --> liballoc/arc.rs:1089:45
[00:03:27]      |
[00:03:27] 1089 |         let inner = if self.ptr.as_ptr() == WEAK_EMPTY as *mut _ {
[00:03:27] 
[00:03:27] error[E0308]: mismatched types
[00:03:27]     --> liballoc/arc.rs:1176:55
[00:03:27]      |
[00:03:27]      |
[00:03:27] 1176 |                 let non_null = NonNull::new_unchecked(self.ptr);
[00:03:27]      |                                                       ^^^^^^^^ expected *-ptr, found struct `core::ptr::NonNull`
[00:03:27]      |
[00:03:27]      = note: expected type `*mut _`
[00:03:27]                 found type `core::ptr::NonNull<arc::ArcInner<T>>`
[00:03:27] 
[00:03:27] error[E0606]: casting `usize` as `*mut arc::ArcInner<T>` is invalid
[00:03:27]     --> liballoc/arc.rs:1167:45
[00:03:27]      |
[00:03:27] 1167 |         let inner = if self.ptr.as_ptr() == WEAK_EMPTY as *mut _ {
[00:03:27] 
[00:03:29] error: aborting due to 5 previous errors
[00:03:29] 
[00:03:29] Some errors occurred: E0277, E0308, E0606.
[00:03:29] Some errors occurred: E0277, E0308, E0606.
[00:03:29] For more information about an error, try `rustc --explain E0277`.
[00:03:29] error: Could not compile `alloc`.
[00:03:29] 
[00:03:29] Caused by:
[00:03:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=5c6ca57f52fc716b -C extra-filename=-5c6ca57f52fc716b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bad063b3019d016c.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-af41331a61619951/out` (exit code: 101)
[00:03:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:29] expected success, got: exit code: 101
[00:03:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:29] travis_fold:end:stage0-std

[00:03:29] travis_time:end:stage0-std:start=1525290261655725535,finish=1525290299296669614,duration=37640944079


[00:03:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:29] Build completed unsuccessfully in 0:00:39
[00:03:29] Makefile:79: recipe for target 'tidy' failed
[00:03:29] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:090c2da8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
