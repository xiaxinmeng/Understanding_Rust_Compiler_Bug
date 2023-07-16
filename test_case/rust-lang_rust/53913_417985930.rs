plain
[00:20:41]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:20:41]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:20:42]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:20:42]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:20:49] error[E0659]: `thread_local` is ambiguous
[00:20:49]      |
[00:20:49] 179  |                   #[thread_local]
[00:20:49]      |                     ^^^^^^^^^^^^
[00:20:49]      | 
[00:20:49]      | 
[00:20:49]     ::: libstd/collections/hash/map.rs:2609:9
[00:20:49]      |
[00:20:49] 2609 | /         thread_local!(static KEYS: Cell<(u64, u64)> = {
[00:20:49] 2610 | |             Cell::new(sys::hashmap_random_keys())
[00:20:49] 2611 | |         });
[00:20:49]      |
[00:20:49]      |
[00:20:49] note: `thread_local` could refer to the name defined here
[00:20:49]      |
[00:20:49]      |
[00:20:49] 142  | / macro_rules! thread_38;5;12m| }
[00:20:49]     | |_- in this macro invocation
[00:20:49]     |
[00:20:49] note: `thread_local` could refer to the name defined here
[00:20:49]     |
[00:20:49] 142 | / macro_rules! thread_local {
[00:20:49] 143 | |     // empty (base case for the recursion)
[00:20:49] 144 | |     () => {};
[00:20:49] 144 | |     () => {};
[00:20:49] 145 | |
[00:20:49] ...   |
[00:20:49] 155 | |     );
[00:20:49] 156 | | }
[00:20:49]     | |_^
[00:20:49] note: `thread_local` could also refer to the name defined here
[00:20:49]     |
[00:20:49] 179 |                   #[thread_local]
[00:20:49]     |                     ^^^^^^^^^^^^
[00:20:49]     | 
[00:20:49]     | 
[00:20:49]    ::: libstd/io/stdio.rs:23:1
[00:20:49]     |
[00:20:49] 23  | / thread_local! {
[00:20:49] 24  | |     static LOCAL_STDOUT: RefCell<Option<Box<dyn Write + Send>>> = {
[00:20:49] 25  | |         RefCell::new(None)
[00:20:49] 27  | | }
[00:20:49]     | |_- in this macro invocation
[00:20:49] 
[00:20:49] 
[00:20:49] error[E0659]: `thread_local` is ambiguous
[00:20:49]     |
[00:20:49] 179 |                   #[thread_local]
[00:20:49]     |                     ^^^^^^^^^^^^
[00:20:49]     | 
[00:20:49]     | 
[00:20:49]    ::: libstd/future.rs:53:1
[00:20:49]     |
[00:20:49] 53  | / thread_local! {
[00:20:49] 54  | |     static TLS_CX: Cell<Option<NonNull<task::Context<'static>>>> = Cell::new(None);
[00:20:49]     | |_- in this macro invocation
[00:20:49]     |
[00:20:49]     |
[00:20:49] note: `thread_local` could refer to the name defined here
[00:20:49]     |
[00:20:49] 142 | / macro_rules! thread_local {
[00:20:49] 143 | |     // empty (base case for the recursion)
[00:20:49] 144 | |     () => {};
---
[00:20:49]     |                     ^^^^^^^^^^^^
[00:20:49]     | 
[00:20:49]    ::: libstd/panicking.rs:38:1
[00:20:49]     |
[00:20:49] 38  | / thread_local! {
[00:20:49] 39  | |     pub static LOCAL_STDERR: RefCell<Option<Box<dyn Write + Send>>> = {
[00:20:49] 40  | |         RefCell::new(None)
[00:20:49] 42  | | }
[00:20:49]     | |_- in this macro invocation
[00:20:49]     |
[00:20:49]     |
[00:20:49] note: `thread_local` could refer to the name defined here
[00:20:49]     |
[00:20:49] 142 | / macro_rules! thread_local {
[00:20:49] 143 | |     // empty (base case for the recursion)
[00:20:49] 144 | |     () => {};
[00:20:49] 144 | |     () => {};
[00:20:49] 145 | |
[00:20:49] ...   |
[00:20:49] 155 | |     );
[00:20:49] 156 | | }
[00:20:49]     | |_^
[00:20:49] note: `thread_local` could also refer to the name defined here
[00:20:49]     |
[00:20:49] 179 |                   #[thread_local]
[00:20:49]     |                     ^^^^^^^^^^^^
[00:20:49]     | 
[00:20:49]     | 
[00:20:49]    ::: libstd/panicking.rs:38:1
[00:20:49]     |
[00:20:49] 38  | / thread_local! {
[00:20:49] 39  | |     pub static LOCAL_STDERR: RefCell<Option<Box<dyn Write + Send>>> = {
[00:20:49] 40  | m|
[00:20:49] 142 | / macro_rules! thread_local {
[00:20:49] 143 | |     // empty (base case for the recursion)
[00:20:49] 144 | |     () => {};
[00:20:49] ...   |
[00:20:49] 155 | |     );
[00:20:49] 156 | | }
[00:20:49]     | |_^
[00:20:49]     | |_^
[00:20:49] note: `thread_local` could also refer to the name defined here
[00:20:49]     |
[00:20:49] 179 |                 #[thread_local]
[00:20:49]     |                   ^^^^^^^^^^^^
[00:20:49]     | 
[00:20:49]     | 
[00:20:49]    ::: libstd/panicking.rs:238:5
[00:20:49]     |
[00:20:49] 238 |     thread_local! { static PANIC_COUNT: Cell<usize> = Cell::new(0) }
[00:20:49] 
[00:20:52] error: aborting due to 6 previous errors
[00:20:52] 
[00:20:52] For more information about this error, try `rustc --explain E0659`.
[00:20:52] For more information about this error, try `rustc --explain E0659`.
[00:20:52] error: Could not compile `std`.
[00:20:52] 
[00:20:52] Caused by:
[00:20:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=d36e1a231adc37cd -C extra-filename=-d36e1a231adc37cd --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c653c24a5aa8b822.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-c9c5677c39e3582f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-03fd58e3ea6b5474.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/def68b0
