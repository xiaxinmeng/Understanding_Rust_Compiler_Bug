plain
[00:03:02]    Compiling cc v1.0.10
[00:03:02]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:02]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:02]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:03] error: unknown macro variable `EndFeature`
[00:03:03]    --> libcore/num/wrapping.rs:590:5
[00:03:03]     |
[00:03:03] 590 | }", $EndFeature, "
[00:03:03]     |     ^^^^^^^^^^^ unknown macro variable
[00:03:03] ...
[00:03:03] 682 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:03]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:03] error: expected a literal
[00:03:03]    --> libcore/num/wrapping.rs:590:5
[00:03:03]     |
[00:03:03]     |
[00:03:03] 590 | }", $EndFeature, "
[00:03:03] ...
[00:03:03] ...
[00:03:03] 682 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
[00:03:03]     | -------------------------------------------------------------------------- in this macro invocation
[00:03:03] 
[00:03:03] error: unknown macro variable `EndFeature`
[00:03:03]    --> libcore/num/wrapping.rs:706:1
[00:03:03]     |
[00:03:03] 706 | $EndFeature, "
[00:03:03]     | ^^^^^^^^^^^ unknown macro variable
[00:03:03] ...
[00:03:03] 782 | wrapping_int_impl_signed! { isize i8 i16 i32 i64 i128 }
[00:03:03]     | ------------------------------------------------------- in this macro invocation
[00:03:03] error: expected a literal
[00:03:03]    --> libcore/num/wrapping.rs:706:1
[00:03:03]     |
[00:03:03]     |
[00:03:03] 706 | $EndFeature, "
[00:03:03] ...
[00:03:03] ...
[00:03:03] 782 | wrapping_int_impl_signed! { isize i8 i16 i32 i64 i128 }
[00:03:03]     | ------------------------------------------------------- in this macro invocation
[00:03:04] error: aborting due to 4 previous errors
[00:03:04] 
[00:03:04] error: Could not compile `core`.
[00:03:04] 
[00:03:04] 
[00:03:04] Caused by:
[00:03:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:04] warning: build failed, waiting for other jobs to finish...
[00:03:08] error: build failed
[00:03:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:08] expected success, got: exit code: 101
[00:03:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:08] travis_fold:end:stage0-std

[00:03:08] travis_time:end:stage0-std:start=1525556213914490530,finish=1525556220460427418,duration=6545936888


[00:03:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:08] Build completed unsuccessfully in 0:00:08
[00:03:08] Makefile:79: recipe for target 'tidy' failed
[00:03:08] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20b2aef6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
