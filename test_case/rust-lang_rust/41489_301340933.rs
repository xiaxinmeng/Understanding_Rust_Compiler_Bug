
[01:02:04] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:315
[01:02:04] 
[01:02:04] ---- [ui] ui/mismatched_types/issue-36053-2.rs stdout ----
[01:02:04] 	ui: /checkout/src/test/ui/mismatched_types/issue-36053-2.rs
[01:02:04] normalized stderr:
[01:02:04] error: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]>` in the current scope
[01:02:04]   --> $DIR/issue-36053-2.rs:17:55
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                                       ^^^^^
[01:02:04]    |
[01:02:04]    = note: the method `count` exists but the following trait bounds were not satisfied:
[01:02:04]            `[closure@$DIR/issue-36053-2.rs:17:39: 17:53] : std::ops::FnMut<(&_,)>`
[01:02:04]            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]> : std::iter::Iterator`
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnMut<(&'r str,)>`, but the trait `for<'r> std::ops::FnMut<(&'r &str,)>` is required
[01:02:04]   --> $DIR/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnMut<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnMut<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnOnce<(&'r str,)>`, but the trait `for<'r> std::ops::FnOnce<(&'r &str,)>` is required
[01:02:04]   --> $DIR/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnOnce<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnOnce<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error: aborting due to 3 previous errors
[01:02:04] 
[01:02:04] 
[01:02:04] 
[01:02:04] expected stderr:
[01:02:04] error: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]>` in the current scope
[01:02:04]   --> $DIR/issue-36053-2.rs:17:55
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                                       ^^^^^
[01:02:04]    |
[01:02:04]    = note: the method `count` exists but the following trait bounds were not satisfied: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53] : std::ops::FnMut<(&_,)>`, `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]> : std::iter::Iterator`
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnMut<(&'r str,)>`, but the trait `for<'r> std::ops::FnMut<(&'r &str,)>` is required
[01:02:04]   --> $DIR/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnMut<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnMut<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnOnce<(&'r str,)>`, but the trait `for<'r> std::ops::FnOnce<(&'r &str,)>` is required
[01:02:04]   --> $DIR/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnOnce<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnOnce<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error: aborting due to 3 previous errors
[01:02:04] 
[01:02:04] 
[01:02:04] 
[01:02:04] diff of stderr:
[01:02:04] 
[01:02:04]  error: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]>` in the current scope
[01:02:04]    --> $DIR/issue-36053-2.rs:17:55
[01:02:04]     |
[01:02:04]  17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]     |                                                       ^^^^^
[01:02:04]     |
[01:02:04] +   = note: the method `count` exists but the following trait bounds were not satisfied:
[01:02:04] +           `[closure@$DIR/issue-36053-2.rs:17:39: 17:53] : std::ops::FnMut<(&_,)>`
[01:02:04] +           `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]> : std::iter::Iterator`
[01:02:04] -   = note: the method `count` exists but the following trait bounds were not satisfied: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53] : std::ops::FnMut<(&_,)>`, `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:17:39: 17:53]> : std::iter::Iterator`
[01:02:04]  
[01:02:04]  error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnMut<(&'r str,)>`, but the trait `for<'r> std::ops::FnMut<(&'r &str,)>` is required
[01:02:04]    --> $DIR/issue-36053-2.rs:17:32
[01:02:04]     |
[01:02:04]  17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]     |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnMut<(&'r str,)>`
[01:02:04]     |                                |
[01:02:04]     |                                requires `for<'r> std::ops::FnMut<(&'r &str,)>`
[01:02:04]     |                                expected &str, found str
[01:02:04]  
[01:02:04]  error[E0281]: type mismatch: `[closure@$DIR/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnOnce<(&'r str,)>`, but the trait `for<'r> std::ops::FnOnce<(&'r &str,)>` is required
[01:02:04]    --> $DIR/issue-36053-2.rs:17:32
[01:02:04]     |
[01:02:04]  17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]     |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnOnce<(&'r str,)>`
[01:02:04]     |                                |
[01:02:04]     |                                requires `for<'r> std::ops::FnOnce<(&'r &str,)>`
[01:02:04]     |                                expected &str, found str
[01:02:04]  
[01:02:04]  error: aborting due to 3 previous errors
[01:02:04]  
[01:02:04] 
[01:02:04] The actual stderr differed from the expected stderr.
[01:02:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2.stderr
[01:02:04] To update references, run this command from build directory:
[01:02:04] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'mismatched_types/issue-36053-2.rs'
[01:02:04] 
[01:02:04] error: 1 errors occurred comparing output.
[01:02:04] status: exit code: 101
[01:02:04] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/ui/mismatched_types/issue-36053-2.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2.stage2-x86_64-unknown-linux-gnu.ui.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:02:04] stdout:
[01:02:04] ------------------------------------------
[01:02:04] 
[01:02:04] ------------------------------------------
[01:02:04] stderr:
[01:02:04] ------------------------------------------
[01:02:04] error: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:39: 17:53]>` in the current scope
[01:02:04]   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:55
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                                       ^^^^^
[01:02:04]    |
[01:02:04]    = note: the method `count` exists but the following trait bounds were not satisfied:
[01:02:04]            `[closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:39: 17:53] : std::ops::FnMut<(&_,)>`
[01:02:04]            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:39: 17:53]> : std::iter::Iterator`
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnMut<(&'r str,)>`, but the trait `for<'r> std::ops::FnMut<(&'r &str,)>` is required
[01:02:04]   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnMut<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnMut<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error[E0281]: type mismatch: `[closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:39: 17:53]` implements the trait `for<'r> std::ops::FnOnce<(&'r str,)>`, but the trait `for<'r> std::ops::FnOnce<(&'r &str,)>` is required
[01:02:04]   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:17:32
[01:02:04]    |
[01:02:04] 17 |     once::<&str>("str").fuse().filter(|a: &str| true).count();
[01:02:04]    |                                ^^^^^^ -------------- implements `for<'r> std::ops::FnOnce<(&'r str,)>`
[01:02:04]    |                                |
[01:02:04]    |                                requires `for<'r> std::ops::FnOnce<(&'r &str,)>`
[01:02:04]    |                                expected &str, found str
[01:02:04] 
[01:02:04] error: aborting due to 3 previous errors
[01:02:04] 
[01:02:04] 
[01:02:04] ------------------------------------------
[01:02:04] 
[01:02:04] thread '[ui] ui/mismatched_types/issue-36053-2.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2472
[01:02:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:04] 
[01:02:04] 
[01:02:04] failures:
[01:02:04]     [ui] ui/mismatched_types/issue-36053-2.rs
[01:02:04] 
[01:02:04] test result: FAILED. 274 passed; 1 failed; 1 ignored; 0 measured
[01:02:04] 
