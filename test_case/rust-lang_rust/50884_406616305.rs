plain
[01:06:51] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:52]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1042 |     assert_eq!(v[..0].iter().cloned().sum_nonempty::<i32>(), None);
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1043 |     assert_eq!(v[1..2].iter().cloned().sum_nonempty::<i32>(), Some(1));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1044 |     assert_eq!(v[1..3].iter().cloned().sum_nonempty::<i32>(), Some(3));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1045 |     assert_eq!(v.iter().cloned().sum_nonempty::<i32>(), Some(55));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1067 |     assert_eq!(v[..0].iter().cloned().product_nonempty::<i32>(), None);
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1068 |     assert_eq!(v[..1].iter().cloned().product_nonempty::<i32>(), Some(0));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1069 |     assert_eq!(v[1..3].iter().cloned().product_nonempty::<i32>(), Some(2));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:06:59] 
[01:06:59] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:06:59]      |
[01:06:59]      |
[01:06:59] 1070 |     assert_eq!(v[1..5].iter().cloned().product_nonempty::<i32>(), Some(24));
[01:06:59]      |
[01:06:59]      |
[01:06:59]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:07:07] error: aborting due to 8 previous errors
[01:07:07] 
[01:07:07] For more information about this error, try `rustc --explain E0658`.
[01:07:07] error: Could not compile `core`.
[01:07:07] error: Could not compile `core`.
[01:07:07] 
[01:07:07] To learn more, run the command again with --verbose.
[01:07:07] 
[01:07:07] 
[01:07:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:07:07] 
[01:07:07] 
[01:07:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:07] Build completed unsuccessfully in 0:19:35
[01:07:07] Build completed unsuccessfully in 0:19:35
[01:07:07] Makefile:58: recipe for target 'check' failed
[01:07:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09651798
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
145348 ./obj/build/bootstrap/debug/incremental
133184 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133180 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130536 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130532 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f32xp7cfib-1qry09o-2iqfvo5raelnm
128732 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstroc
65412 ./src/llvm-emscripten/test/CodeGen
60840 ./src/llvm-emscripten/lib
58528 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
55236 ./src/llvm/test/MC
