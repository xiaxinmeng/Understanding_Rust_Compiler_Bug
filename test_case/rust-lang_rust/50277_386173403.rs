plain
[00:42:46] [RUSTC-TIMING] panic_abort test:false 0.107
[00:42:49] [RUSTC-TIMING] alloc test:false 5.660
[00:42:49]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:42:49] [RUSTC-TIMING] panic_unwind test:false 0.283
[00:42:54] error[E0512]: transmute called with types of different sizes
[00:42:54]   --> libstd/sys/unix/weak.rs:67:17
[00:42:54]    |
[00:42:54] 67 |                 mem::transmute::<&AtomicUsize, Option<&F>>(&self.addr)
[00:42:54]    |
[00:42:54]    |
[00:42:54]    = note: source type: &core::sync::atomic::AtomicUsize (32 bits)
[00:42:54]    = note: target type: core::option::Option<&F> (64 bits)
[00:42:54] error: aborting due to previous error
[00:42:54] 
[00:42:54] For more information about this error, try `rustc --explain E0512`.
[00:42:54] [RUSTC-TIMING] std test:false 4.220
[00:42:54] [RUSTC-TIMING] std test:false 4.220
[00:42:54] error: Could not compile `std`.
[00:42:54] 
[00:42:54] Caused by:
[00:42:54]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=0bf399317c8f84bc -C extra-filename=-0bf399317c8f84bc --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps --target i586-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libunwind-2b252a17661c787a.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libstd_unicode-45d9722759933375.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libpanic_abort-dd6088d4b96e603b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/liballoc_jemalloc-22acab9569ca8bd1.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libcompiler_builtins-234f752677a6d2ee.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/liballoc-bc9548d20570105d.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/liblibc-033e182156d9d1df.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libcore-af0391c2e87334c4.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/libpanic_unwind-be599d7c4feb0ef6.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/liballoc_system-eb84ee30c447a758.rlib -L native=/checkout/obj/build/i586-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/build/compiler_builtins-e6754e8cc91cbe7e/out -L native=/checkout/obj/build/i586-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:42:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i586-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:42:54] expected success, got: exit code: 101
[00:42:54] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:42:54] travis_fold:end:stage2-std

[00:42:54] travis_time:end:stage2-std:start=1525312797853246529,finish=1525312855326291724,duration=57473045195

