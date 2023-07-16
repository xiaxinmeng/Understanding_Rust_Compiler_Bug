plain
[00:26:00] [RUSTC-TIMING] panic_unwind test:false 0.276
[00:26:05] error: unused variable: `fd`
[00:26:05]    --> libstd/sys/unix/fs.rs:481:27
[00:26:05]     |
[00:26:05] 481 |         fn ensure_cloexec(fd: &FileDesc) -> io::Result<()> {
[00:26:05]     |                           ^^ help: consider using `_fd` instead
[00:26:05]     = note: `-D unused-variables` implied by `-D warnings`
[00:26:05] 
[00:26:05] error: aborting due to previous error
[00:26:05] 
[00:26:05] 
[00:26:05] [RUSTC-TIMING] std test:false 4.449
[00:26:05] error: Could not compile `std`.
[00:26:05] 
[00:26:05] Caused by:
[00:26:05]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=8ebd5be427249cd3 -C extra-filename=-8ebd5be427249cd3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps --target i686-unknown-freebsd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libpanic_abort-f88a00d162e3144e.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libunwind-aa83f963a589d5c7.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liblibc-69a491b68a83ff00.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc_system-b06b948ecc04fcd6.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libstd_unicode-cb230abdcba680b1.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc-8d719fed8238a551.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc_jemalloc-4cb9ec8db81fe30b.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libpanic_unwind-f134dd64f219bf1b.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libcore-6eb1a77d793ab07f.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libcompiler_builtins-27a9e89a44272f11.rlib -L native=/checkout/obj/build/i686-unknown-freebsd/native/libbacktrace/.libs -l static=backtrace -l execinfo -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/build/compiler_builtins-5785bd67f4678604/out -L native=/checkout/obj/build/i686-unknown-freebsd/native/jemalloc/lib` (exit code: 101)
[00:26:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-freebsd" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:26:05] expected success, got: exit code: 101
[00:26:05] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:26:05] travis_fold:end:stage1-std

[00:26:05] travis_time:end:stage1-std:start=1526258460915195427,finish=1526258522546259710,duration=61631064283

