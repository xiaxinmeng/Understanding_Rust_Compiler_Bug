plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00756921
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:51:28]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:51:30]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:51:31]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:51:35]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]      |
[00:51:40] 179  |                   #[thread_local]
[00:51:40]      |                     ^^^^^^^^^^^^
[00:51:40]      | 
[00:51:40]      | 
[00:51:40]     ::: libstd/collections/hash/map.rs:2609:9
[00:51:40]      |
[00:51:40] 2609 | /         thread_local!(static KEYS: Cell<(u64, u64)> = {
[00:51:40] 2610 | |             Cell::new(sys::hashmap_random_keys())
[00:51:40] 2611 | |         });
[00:51:40]      |
[00:51:40]      |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]      |
[00:51:40] 142  | / macro_rules! thread_local {
[00:51:40] 143  | |     // empty (base case for the recursion)
[00:51:40] 144  | |     () => {};
[00:51:40] 144  | |     () => {};
[00:51:40] 145  | |
[00:51:40] ...    |
[00:51:40] 155  | |     );
[00:51:40] 156  | | }
[00:51:40]      | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]      |
[00:51:40] 179  |                   #[thread_local]
[00:51:40]      |                     ^^^^^^^^^^^^
[00:51:40]      | 
[00:51:40]      | 
[00:51:40]     ::: libstd/collections/hash/map.rs:2609:9
[00:51:40]      |
[00:51:40] 2609 | /         thread_local!(static KEYS: Cell<(u64, u64)> = {
[00:51:40] 2610 | |             Cell::new(sys::hashmap_random_keys())
[00:51:40] 2611 | |         });
[00:51:40] 
[00:51:40] 
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/io/stdio.rs:23:1
[00:51:40]     |
[00:51:40] 23  | / thread_local! {
[00:51:40] 24  | |     static LOCAL_STDOUT: RefCell<Option<Box<dyn Write + Send>>> = {
[00:51:40] 25  | |         RefCell::new(None)
[00:51:40] 27  | | }
[00:51:40]     | |_- in this macro invocation
[00:51:40]     |
[00:51:40]     |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]     |
[00:51:40] 142 | / macro_rules! thread_local {
[00:51:40] 143 | |     // empty (base case for the recursion)
[00:51:40] 144 | |     () => {};
[00:51:40] 144 | |     () => {};
[00:51:40] 145 | |
[00:51:40] ...   |
[00:51:40] 155 | |     );
[00:51:40] 156 | | }
[00:51:40]     | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/io/stdio.rs:23:1
[00:51:40]     |
[00:51:40] 23  | / thread_local! {
[00:51:40] 24  | |     static LOCAL_STDOUT: RefCell<Option<Box<dyn Write + Send>>> = {
[00:51:40] 25  | |         RefCell::new(None)
[00:51:40] 27  | | }
[00:51:40]     | |_- in this macro invocation
[00:51:40] 
[00:51:40] 
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/future.rs:53:1
[00:51:40]     |
[00:51:40] 53  | / thread_local! {
[00:51:40] 54  | |     static TLS_CX: Cell<Option<NonNull<task::Context<'static>>>> = Cell::new(None);
[00:51:40]     | |_- in this macro invocation
[00:51:40]     |
[00:51:40]     |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]     |
[00:51:40] 142 | / macro_rules! thread_local {
[00:51:40] 143 | |     // empty (base case for the recursion)
[00:51:40] 144 | |     () => {};
[00:51:40] 144 | |     () => {};
[00:51:40] 145 | |
[00:51:40] ...   |
[00:51:40] 155 | |     );
[00:51:40] 156 | | }
[00:51:40]     | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/future.rs:53:1
[00:51:40]     |
[00:51:40] 53  | / thread_local! {
[00:51:40] 54  | |     static TLS_CX: Cell<Option<NonNull<task::Context<'static>>>> = Cell::new(None);
[00:51:40]     | |_- in this macro invocation
[00:51:40] 
[00:51:40] 
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]     |
[00:51:40] 179 |                 #[thread_local]
[00:51:40]     |                   ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/sys_common/thread_info.rs:22:1
[00:51:40]     |
[00:51:40] 22  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = RefCell::new(None) }
[00:51:40]     | -------------------------------------------------------------------------------------- in this macro invocation
[00:51:40]     |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]     |
[00:51:40] 142 | / macro_rules! thread_local {
[00:51:40] 143 | |     // empty (base case for the recursion)
[00:51:40] 144 | |     () => {};
[00:51:40] 144 | |     () => {};
[00:51:40] 145 | |
[00:51:40] ...   |
[00:51:40] 155 | |     );
[00:51:40] 156 | | }
[00:51:40]     | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]     |
[00:51:40] 179 |                 #[thread_local]
[00:51:40]     |                   ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/sys_common/thread_info.rs:22:1
[00:51:40]     |
[00:51:40] 22  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = RefCell::new(None) }
[00:51:40]     | -------------------------------------------------------------------------------------- in this macro invocation
[00:51:40] 
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/panicking.rs:38:1
[00:51:40]     |
[00:51:40] 38  | / thread_local! {
[00:51:40] 39  | |     pub static LOCAL_STDERR: RefCell<Option<Box<dyn Write + Send>>> = {
[00:51:40] 40  | |         RefCell::new(None)
[00:51:40] 42  | | }
[00:51:40]     | |_- in this macro invocation
[00:51:40]     |
[00:51:40]     |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]     |
[00:51:40] 142 | / macro_rules! thread_local {
[00:51:40] 143 | |     // empty (base case for the recursion)
[00:51:40] 144 | |     () => {};
[00:51:40] 144 | |     () => {};
[00:51:40] 145 | |
[00:51:40] ...   |
[00:51:40] 155 | |     );
[00:51:40] 156 | | }
[00:51:40]     | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]     |
[00:51:40] 179 |                   #[thread_local]
[00:51:40]     |                     ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/panicking.rs:38:1
[00:51:40]     |
[00:51:40] 38  | / thread_local! {
[00:51:40] 39  | |     pub static LOCAL_STDERR: RefCell<Option<Box<dyn Write + Send>>> = {
[00:51:40] 40  | |         RefCell::new(None)
[00:51:40] 42  | | }
[00:51:40]     | |_- in this macro invocation
[00:51:40] 
[00:51:40] 
[00:51:40] error[E0659]: `thread_local` is ambiguous
[00:51:40]     |
[00:51:40] 179 |                 #[thread_local]
[00:51:40]     |                   ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/panicking.rs:238:5
[00:51:40]     |
[00:51:40] 238 |     thread_local! { static PANIC_COUNT: Cell<usize> = Cell::new(0) }
[00:51:40]     |
[00:51:40]     |
[00:51:40] note: `thread_local` could refer to the name defined here
[00:51:40]     |
[00:51:40] 142 | / macro_rules! thread_local {
[00:51:40] 143 | |     // empty (base case for the recursion)
[00:51:40] 144 | |     () => {};
[00:51:40] 144 | |     () => {};
[00:51:40] 145 | |
[00:51:40] ...   |
[00:51:40] 155 | |     );
[00:51:40] 156 | | }
[00:51:40]     | |_^
[00:51:40] note: `thread_local` could also refer to the name defined here
[00:51:40]     |
[00:51:40] 179 |                 #[thread_local]
[00:51:40]     |                   ^^^^^^^^^^^^
[00:51:40]     | 
[00:51:40]     | 
[00:51:40]    ::: libstd/panicking.rs:238:5
[00:51:40]     |
[00:51:40] 238 |     thread_local! { static PANIC_COUNT: Cell<usize> = Cell::new(0) }
[00:51:40] 
[00:51:45] error: aborting due to 6 previous errors
[00:51:45] 
[00:51:45] For more information about this error, try `rustc --explain E0659`.
[00:51:45] For more information about this error, try `rustc --explain E0659`.
[00:51:45] error: Could not compile `std`.
[00:51:45] 
[00:51:45] Caused by:
[00:51:45]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' -C metadata=d879867f50afc8df -C extra-filename=-d879867f50afc8df --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-381ef4303a4b8d72.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-157ed363282e1456.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-75d7082732f6c6fc.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-0e27b5e72d396482.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-6b88aff5affc9853.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-f41d7104b4e208fd.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-86caad002ed4f9b7.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-6f01c2159763e870.rlib --extern profiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-90288e148285012a.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-10443f513b1a041a.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0a53eb9e15b2b139.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-f9732fb739f53137.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-c3842baa2002649a.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-7eda5676ba5b12b7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-35d02304dcd871b8/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-d0dc0cd9b3a831c7/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/linux -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build/lib/linux -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build/lib/linux -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/tsan/build/lib/linux` (exit code: 1)
[00:51:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:51:45] expected success, got: exit code: 101
[00:51:45] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:51:45] travis_fold:end:stage1-std

[00:51:45] travis_time:end:stage1-std:start=1535944530166167759,finish=1535944613136543403,duration=82970375644

---
travis_time:end:0ffd8956:start=1535944614373784555,finish=1535944614381228261,duration=7443706
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026761d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10d03180
travis_time:start:10d03180
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d4dae14
$ dmesg | grep -i kill
