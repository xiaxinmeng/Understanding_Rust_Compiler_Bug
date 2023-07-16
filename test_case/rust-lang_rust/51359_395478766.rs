
[00:45:30] [RUSTC-TIMING] alloc_jemalloc test:false 0.092
[00:45:33] [RUSTC-TIMING] alloc test:false 5.116
[00:45:33]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:45:34] [RUSTC-TIMING] panic_unwind test:false 0.242
[00:45:35] error[E0425]: cannot find value `FDIO_SPAWN_SHARE_JOB` in this scope
[00:45:35]   --> libstd/sys/unix/process/process_fuchsia.rs:97:13
[00:45:35]    |
[00:45:35] 97 |             FDIO_SPAWN_SHARE_JOB | FDIO_SPAWN_CLONE_LDSVC | FDIO_SPAWN_CLONE_NAMESPACE,
[00:45:35]    |             ^^^^^^^^^^^^^^^^^^^^ did you mean `FDIO_SPAWN_CLONE_JOB`?
[00:45:38] error: aborting due to previous error
[00:45:38] 
[00:45:38] For more information about this error, try `rustc --explain E0425`.
[00:45:38] [RUSTC-TIMING] std test:false 4.179
[00:45:38] [RUSTC-TIMING] std test:false 4.179
[00:45:38] error: Could not compile `std`.
[00:45:38] 
[00:45:38] Caused by:
[00:45:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=17dd438e5d184cbf -C extra-filename=-17dd438e5d184cbf --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps --target x86_64-unknown-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libunwind-1aa1c22034f50983.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_abort-188ece4b27241321.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libstd_unicode-39f4ce861fcffd4b.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_system-f14987dbf40b5a04.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liblibc-62ac5dd3ecae39a9.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcompiler_builtins-658dfdd7590d59a4.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcore-add11f9db7b34fd0.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_jemalloc-a4122c6c45d0cdc6.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_unwind-01fe06fcbc0bcf6c.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc-0e7fccf2d675931b.rlib -l backtrace -l zircon -l fdio -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/build/compiler_builtins-838ed0d7521287fa/out` (exit code: 101)
[00:45:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:45:38] expected success, got: exit code: 101
[00:45:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:45:38] travis_fold:end:stage2-std

[00:45:38] travis_time:end:stage2-std:start=1528339597057569295,finish=1528339655027496323,duration=57969927028
