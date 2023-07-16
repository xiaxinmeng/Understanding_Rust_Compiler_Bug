plain
[01:01:14] [RUSTC-TIMING] panic_abort test:false 0.137
[01:01:20] [RUSTC-TIMING] alloc test:false 7.407
[01:01:20]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:01:20] [RUSTC-TIMING] panic_unwind test:false 0.314
[01:01:24] error[E0609]: no field `dirp` on type `&mut sys::unix::fs::ReadDir`
[01:01:24]    --> libstd/sys/unix/fs.rs:232:52
[01:01:24]     |
[01:01:24] 232 |                 let entry_ptr = libc::readdir(self.dirp.0);
[01:01:24] 
[01:01:26] error: aborting due to previous error
[01:01:26] 
[01:01:26] For more information about this error, try `rustc --explain E0609`.
[01:01:26] For more information about this error, try `rustc --explain E0609`.
[01:01:26] [RUSTC-TIMING] std test:false 5.772
[01:01:26] error: Could not compile `std`.
[01:01:26] 
[01:01:26] Caused by:
[01:01:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=3fed1de603a2895b -C extra-filename=-3fed1de603a2895b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps --target x86_64-unknown-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_jemalloc-ef3d35b058f29bdc.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc-4f4f362b39a7ae79.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liblibc-1d1ad3278eec445a.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcore-32cc14345520ec4a.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libunwind-ed8fc7ec397df7ff.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libstd_unicode-a73afe93d98762e2.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_unwind-6864429c0391c9fa.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_abort-09154b7a50d1d88a.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_system-37dcefab5d368769.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcompiler_builtins-96b7e577a9b42ad9.rlib -l backtrace -l zircon -l fdio -l launchpad -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/build/compiler_builtins-aa886a5ffe5369ef/out` (exit code: 101)
[01:01:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:01:26] expected success, got: exit code: 101
[01:01:26] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:01:26] travis_fold:end:stage2-std

[01:01:26] travis_time:end:stage2-std:start=1527712333488201725,finish=1527712416797898541,duration=83309696816

