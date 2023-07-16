plain
[01:05:44]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:05:45] [RUSTC-TIMING] panic_unwind test:false 0.257
[01:05:45] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
[01:05:45] 
[01:05:47] error[E0063]: missing field `__align` in initializer of `sys::redox::net::netc::in6_addr`
[01:05:47]     |
[01:05:47]     |
[01:05:47] 876 |             inner: c::in6_addr {
[01:05:47]     |                    ^^^^^^^^^^^ missing `__align`
[01:05:48] error: aborting due to previous error
[01:05:48] 
[01:05:48] For more information about this error, try `rustc --explain E0063`.
[01:05:48] [RUSTC-TIMING] std test:false 3.596
[01:05:48] [RUSTC-TIMING] std test:false 3.596
[01:05:48] error: Could not compile `std`.
[01:05:48] 
[01:05:48] Caused by:
[01:05:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=66946813e275c7d3 -C extra-filename=-66946813e275c7d3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps --target x86_64-unknown-redox -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc-22cfd78394ae55d8.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_jemalloc-a4568a7392ee48d0.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_system-b2168c9091f18465.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcompiler_builtins-49b3d1f2a97507e3.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcore-498f77a3860e253a.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liblibc-aed8e3326796429e.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_abort-aab9335a8d0b9f3c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_unwind-ee1801e0d62b13a2.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libunwind-cb98e7330ccaff35.rlib -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/build/compiler_builtins-8e667b642db51e76/out` (exit code: 1)
[01:05:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:05:48] expected success, got: exit code: 101
[01:05:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[01:05:48] travis_fold:end:stage2-std

[01:05:48] travis_time:end:stage2-std:start=1533439158351279738,finish=1533439199349222700,duration=40997942962

---
travis_time:end:0cb76188:start=1533439201329022740,finish=1533439201351610806,duration=22588066
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0331f19d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a9c26e7
travis_time:start:0a9c26e7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2aa5dc21
$ dmesg | grep -i kill
