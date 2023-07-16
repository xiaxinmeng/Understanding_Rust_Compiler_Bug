plain
[00:56:05]     |
[00:56:05] 356 | #[macro_use]
[00:56:05]     | ^^^^^^^^^^^^
[00:56:05]     |
[00:56:05]     = note: `-D unused-imports` implied by `-D warnings`
[00:56:08] error: aborting due to previous error
[00:56:08] 
[00:56:08] [RUSTC-TIMING] std test:false 4.817
[00:56:08] error: Could not compile `std`.
[00:56:08] error: Could not compile `std`.
[00:56:08] 
[00:56:08] Caused by:
[00:56:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=534ee0ae04cab6fa -C extra-filename=-534ee0ae04cab6fa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps --target x86_64-unknown-cloudabi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libstd_unicode-3f84849f7dbcad89.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liblibc-e15e12cf48959a6c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_unwind-300353311ec4986d.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcore-d2782d980fa6d36f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_system-b8ede3f3f7aa18d1.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libunwind-6ede823d95076f67.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_abort-44a3c397af9e7e18.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc-2e2068a4e932757e.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_jemalloc-175aa4b3e77e8161.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcompiler_builtins-7f5b3dee4bd6a99b.rlib -l unwind -l c -l compiler_rt -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/build/compiler_builtins-1305ec87249085e8/out` (exit code: 101)
[00:56:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-cloudabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:56:08] expected success, got: exit code: 101
[00:56:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:56:08] travis_fold:end:stage2-std

[00:56:08] travis_time:end:stage2-std:start=1525030681652993185,finish=1525030736882830562,duration=55229837377

