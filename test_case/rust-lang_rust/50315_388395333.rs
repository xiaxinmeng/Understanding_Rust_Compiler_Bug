plain
[00:24:30]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:24:30]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:24:32]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:24:32]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]   --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]    |
[00:25:23] 47 |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 75 |         int_to_float!(i, i32, f32)
[00:25:23]    |         -------------------------- in this macro invocation
[00:25:23]    |
[00:25:23]    = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]   --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]    |
[00:25:23] 47 |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 80 |         int_to_float!(i, i32, f64)
[00:25:23]    |         -------------------------- in this macro invocation
[00:25:23]    |
[00:25:23]    = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]   --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]    |
[00:25:23] 47 |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 91 |             int_to_float!(i, i64, f64)
[00:25:23]    |             -------------------------- in this macro invocation
[00:25:23]    |
[00:25:23]    = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]   --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]    |
[00:25:23] 47 |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 97 |         int_to_float!(i, i128, f32)
[00:25:23]    |         --------------------------- in this macro invocation
[00:25:23]    |
[00:25:23]    = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]     |
[00:25:23] 47  |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 102 |         int_to_float!(i, i128, f64)
[00:25:23]     |         --------------------------- in this macro invocation
[00:25:23]     |
[00:25:23]     = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]     |
[00:25:23] 47  |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 107 |         int_to_float!(i, u32, f32)
[00:25:23]     |         -------------------------- in this macro invocation
[00:25:23]     |
[00:25:23]     = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value`
[00:25:23]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:47:23
[00:25:23]     |
[00:25:23] 47  |                 ((a & <$ity as Int>::UnsignedInt::max_value()).wrapping_shl((n + mant_dig_plus_two) - sd) != 0) as <$ity as Int>::UnsignedInt
[00:25:23] ...
[00:25:23] ...
[00:25:23] 112 |         int_to_float!(i, u32, f64)
[00:25:23]     |         -------------------------- in this macro invocation
[00:25:23]     |
[00:25:23]     = note: can't use a type alias as a constructor
[00:25:23] 
[00:25:23] error[E0423]: expected function, found associated type `Int::UnsignedInt::max_value^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:25:23] ...
[00:25:23] 130 |         int_to_float!(i, u128, f64)
[00:25:23]     |         --------------------------- in this macro invocation
[00:25:23]     |
[00:25:23]     = note: can't use a type alias as a constructor
[00:25:25] error: aborting due to 9 previous errors
[00:25:25] 
[00:25:25] For more information about this error, try `rustc --explain E0423`.
[00:25:25] error: Could not compile `compiler_builtins`.
[00:25:25] error: Could not compile `compiler_builtins`.
[00:25:25] 
[00:25:25] Caused by:
[00:25:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 --cfg feature="c" --cfg feature="compiler-builtins" --cfg feature="default" --cfg feature="rustbuild" -C metadata=b4e196030fb3059f -C extra-filename=-b4e196030fb3059f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-07c899b6c3d44697.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-3c62409c53174beb/out --cfg use_c -l static=compiler-rt` (exit code: 101)
[00:25:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:25:25] expected success, got: exit code: 101
[00:25:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:25:25] travis_fold:end:stage1-std

[00:25:25] travis_time:end:stage1-std:start=1526051846241512344,finish=1526051915849613440,duration=69608101096


[00:25:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:25] Build completed unsuccessfully in 0:20:05
[00:25:25] make: *** [all] Error 1
[00:25:25] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:259d81e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
