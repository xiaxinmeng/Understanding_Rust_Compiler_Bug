plain
[01:03:39] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:40]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1041 |     assert_eq!(v[..0].iter().cloned().sum_nonempty::<i32>(), None);
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1042 |     assert_eq!(v[1..2].iter().cloned().sum_nonempty::<i32>(), Some(1));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1043 |     assert_eq!(v[1..3].iter().cloned().sum_nonempty::<i32>(), Some(3));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1044 |     assert_eq!(v.iter().cloned().sum_nonempty::<i32>(), Some(55));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1066 |     assert_eq!(v[..0].iter().cloned().product_nonempty::<i32>(), None);
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1067 |     assert_eq!(v[..1].iter().cloned().product_nonempty::<i32>(), Some(0));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1068 |     assert_eq!(v[1..3].iter().cloned().product_nonempty::<i32>(), Some(2));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:47] 
[01:03:47] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[01:03:47]      |
[01:03:47]      |
[01:03:47] 1069 |     assert_eq!(v[1..5].iter().cloned().product_nonempty::<i32>(), Some(24));
[01:03:47]      |
[01:03:47]      |
[01:03:47]      = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[01:03:54] error: aborting due to 8 previous errors
[01:03:54] 
[01:03:54] For more information about this error, try `rustc --explain E0658`.
[01:03:54] error: Could not compile `core`.
[01:03:54] error: Could not compile `core`.
[01:03:54] 
[01:03:54] To learn more, run the command again with --verbose.
[01:03:54] 
[01:03:54] 
[01:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:03:54] 
[01:03:54] 
[01:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:54] Build completed unsuccessfully in 0:18:47
[01:03:54] Build completed unsuccessfully in 0:18:47
[01:03:54] Makefile:58: recipe for target 'check' failed
[01:03:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b8506d5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
