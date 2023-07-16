plain
[01:11:12] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:12]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/option.rs:305:27
[01:11:26]     |
[01:11:26] 305 |     assert_eq!(ref_option.deref(), Some(&42));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/option.rs:309:27
[01:11:26]     |
[01:11:26] 309 |     assert_eq!(ref_option.deref(), None);
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:239:23
[01:11:26]     |
[01:11:26] 239 |     assert_eq!(ref_ok.deref_ok(), Ok(&42));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:240:23
[01:11:26]     |
[01:11:26] 240 |     assert_eq!(ref_ok.deref_ok(), Ok(&42));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:241:2m                       ^^^^^^^^^
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:247:24
[01:11:26]     |
[01:11:26] 247 |     assert_eq!(ref_err.deref(), Err(&41));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:250:23
[01:11:26]     |
[01:11:26] 250 |     assert_eq!(ref_ok.deref_err(), Ok(&&42));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:26] 
[01:11:26] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:11:26]    --> libcore/../libcore/tests/result.rs:253:24
[01:11:26]     |
[01:11:26] 253 |     assert_eq!(ref_err.deref_ok(), Err(&&41));
[01:11:26]     |
[01:11:26]     |
[01:11:26]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:11:27] error: aborting due to 10 previous errors
[01:11:27] 
[01:11:27] For more information about this error, try `rustc --explain E0658`.
[01:11:27] error: Could not compile `core`.
[01:11:27] error: Could not compile `core`.
[01:11:27] 
[01:11:27] To learn more, run the command again with --verbose.
[01:11:27] 
[01:11:27] 
[01:11:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:11:27] 
[01:11:27] 
[01:11:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:27] Build completed unsuccessfully in 0:29:42
[01:11:27] Build completed unsuccessfully in 0:29:42
[01:11:27] Makefile:58: recipe for target 'check' failed
[01:11:27] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fdab3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
