plain
[01:10:41] 
[01:10:41] error[E0425]: cannot find value `i` in this scope
[01:10:41]    --> libstd/sys/redox/fs.rs:174:32
[01:10:41]     |
[01:10:41] 174 |         let upper = self.data[(i + 1)..].iter()
[01:10:41]     |                                ^ help: try: `self.i`
[01:10:41] 
[01:10:44] error[E0277]: the trait bound `&&u8: core::cmp::PartialEq<u8>` is not satisfied
[01:10:44]    --> libstd/sys/redox/fs.rs:175:33
[01:10:44]     |
[01:10:44] 175 |             .filter(|byte| byte == b'\n')
[01:10:44]     |                                 ^^ can't compare `&&u8` with `u8`
[01:10:44]     |
[01:10:44]     = help: the trait `core::cmp::PartialEq<u8>` is not implemented for `&&u8`
[01:10:44] error: aborting due to 2 previous errors
[01:10:44] 
[01:10:44] Some errors occurred: E0277, E0425.
[01:10:44] For more information about an error, try `rustc --explain E0277`.
[01:10:44] For more information about an error, try `rustc --explain E0277`.
[01:10:44] [RUSTC-TIMING] std test:false 3.659
[01:10:44] error: Could not compile `std`.
[01:10:44] 
[01:10:44] Caused by:
[01:10:44]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=d76f05f9c3d5b8d7 -C extra-filename=-d76f05f9c3d5b8d7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps --target x86_64-unknown-redox -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcompiler_builtins-469e43b4d14111bc.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libstd_unicode-203a18552228315c.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_jemalloc-7625c0c6ecb8ea45.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc-c76fe45e7a577193.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libunwind-21d76117e79c3f0e.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcore-83234ee7fc2192a2.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_unwind-3c9d143744128f78.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_abort-22fef0215914bac1.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_system-9feb6a57bde20c89.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liblibc-38812fa12d8f5729.rlib -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace/.libs -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/build/compiler_builtins-2f812540b55ccc61/out` (exit code: 101)
[01:10:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:10:44] expected success, got: exit code: 101
[01:10:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:10:44] travis_fold:end:stage2-std

[01:10:44] travis_time:end:stage2-std:start=1525325793802945326,finish=1525325848177816514,duration=54374871188

