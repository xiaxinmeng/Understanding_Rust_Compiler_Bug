plain
[00:04:38]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:39] error[E0308]: mismatched types
[00:04:39]     --> liballoc/arc.rs:1000:45
[00:04:39]      |
[00:04:39] 1000 |                 ptr: NonNull::new_unchecked(WEAK_EMPTY as *mut T),
[00:04:39]      |                                             ^^^^^^^^^^^^^^^^^^^^ expected struct `arc::ArcInner`, found type parameter
[00:04:39]      |
[00:04:39]      = note: expected type `*mut arc::ArcInner<T>`
[00:04:39]                 found type `*mut T`
[00:04:39] error[E0308]: mismatched types
[00:04:39]     --> liballoc/arc.rs:1065:58
[00:04:39]      |
[00:04:39]      |
[00:04:39] 1065 |                     ptr: unsafe { NonNull::new_unchecked(self.ptr) },
[00:04:39]      |                                                          ^^^^^^^^ expected *-ptr, found struct `core::ptr::NonNull`
[00:04:39]      |
[00:04:39]      = note: expected type `*mut arc::ArcInner<T>`
[00:04:39]                 found type `core::ptr::NonNull<arc::ArcInner<T>>`
[00:04:39] 
[00:04:39] error[E0606]: casting `*mut arc::ArcInner<T>` as `usize` is invalid
[00:04:39]     --> liballoc/arc.rs:1037:24
[00:04:39]      |
[00:04:39] 1037 |         let inner = if self.ptr.as_ptr() as usize == WEAK_EMPTY {
[00:04:39]      |
[00:04:39]      |
[00:04:39]      = help: cast through a thin pointer first
[00:04:39] 
[00:04:39] error[E0277]: the trait bound `T: core::marker::Sized` is not satisfied
[00:04:39]     --> liballoc/arc.rs:1090:20
[00:04:39]      |
[00:04:39] 1090 |             return Weak::new();
[00:04:39]      |                    ^^^^^^^^^ `T` does not have a constant size known at compile-time
[00:04:39]      |
[00:04:39]      = help: the trait `core::marker::Sized` is not implemented for `T`
[00:04:39]      = help: consider adding a `where T: core::marker::Sized` bound
[00:04:39] note: required by `<arc::Weak<T>>::new`
[00:04:39]     --> liballoc/arc.rs:997:5
[00:04:39]      |
[00:04:39] 997  |     pub fn new() -> Weak<T> {
[00:04:39] 
[00:04:39] 
[00:04:39] error[E0606]: casting `*mut arc::ArcInner<T>` as `usize` is invalid
[00:04:39]     --> liballoc/arc.rs:1089:24
[00:04:39]      |
[00:04:39] 1089 |         let inner = if self.ptr.as_ptr() as usize == WEAK_EMPTY {
[00:04:39]      |
[00:04:39]      |
[00:04:39]      = help: cast through a thin pointer first
[00:04:39] error[E0308]: mismatched types
[00:04:39]     --> liballoc/arc.rs:1176:55
[00:04:39]      |
[00:04:39]      |
[00:04:39] 1176 |                 let non_null = NonNull::new_unchecked(self.ptr);
[00:04:39]      |                                                       ^^^^^^^^ expected *-ptr, found struct `core::ptr::NonNull`
[00:04:39]      |
[00:04:39]      = note: expected type `*mut _`
[00:04:39]                 found type `core::ptr::NonNull<arc::ArcInner<T>>`
[00:04:39] 
[00:04:39] error[E0606]: casting `*mut arc::ArcInner<T>` as `usize` is invalid
[00:04:39]     --> liballoc/arc.rs:1167:24
[00:04:39]      |
[00:04:39] 1167 |         let inner = if self.ptr.as_ptr() as usize == WEAK_EMPTY {
[00:04:39]      |
[00:04:39]      |
[00:04:39]      = help: cast through a thin pointer first
[00:04:42] error: aborting due to 7 previous errors
[00:04:42] 
[00:04:42] Some errors occurred: E0277, E0308, E0606.
[00:04:42] For more information about an error, try `rustc --explain E0277`.
[00:04:42] For more information about an error, try `rustc --explain E0277`.
[00:04:42] error: Could not compile `alloc`.
[00:04:42] 
[00:04:42] Caused by:
[00:04:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=5c6ca57f52fc716b -C extra-filename=-5c6ca57f52fc716b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bad063b3019d016c.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-af41331a61619951/out` (exit code: 101)
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:42] expected success, got: exit code: 101
[00:04:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:42] travis_fold:end:stage0-std

[00:04:42] travis_time:end:stage0-std:start=1525289571517417792,finish=1525289627748884126,duration=56231466334


[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:42] Build completed unsuccessfully in 0:00:57
[00:04:42] Makefile:79: recipe for target 'tidy' failed
[00:04:42] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05213b80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
