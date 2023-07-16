plain
[01:38:56] -   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:44
[01:38:56] + error: lifetime may not live long enough
[01:38:56] +   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
[01:38:56] 3    |
[01:38:56] 4 LL |     fn f(self: Pin<&Self>) -> impl Clone { self }
[01:38:56] -    |                               ----------   ^^^^ ...but this borrow...
[01:38:56] -    |                               this return type evaluates to the `'static` lifetime...
[01:38:56] -    |
[01:38:56] -    |
[01:38:56] - note: ...can't outlive the anonymous lifetime #1 defined on the method body at 8:5
[01:38:56] -   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:5
[01:38:56] -    |
[01:38:56] - LL |     fn f(self: Pin<&Self>) -> impl Clone { self }
[01:38:56] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:38:56] - help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 8:5
[01:38:56] +    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
[01:38:56] +    |                    let's call the lifetime of this reference `'1`
[01:38:56] +    |                    let's call the lifetime of this reference `'1`
[01:38:56] + help: to allow this impl Trait to capture borrowed data with lifetime `'1`, add `'_` as a constraint
[01:38:56] 15    |
[01:38:56] 16 LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self }
[01:38:56] 
[01:38:56] 
[01:38:56] The actual stderr differed from the expected stderr.
[01:38:56] The actual stderr differed from the expected stderr.
[01:38:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/arbitrary_self_types_pin_lifetime_impl_trait.nll.stderr
[01:38:56] To update references, rerun the tests and pass the `--bless` flag
[01:38:56] To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait.rs`
[01:38:56] error: 1 errors occurred comparing output.
[01:38:56] status: exit code: 1
[01:38:56] status: exit code: 1
[01:38:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll" "-Zborrowck=mir" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.nll/auxiliary" "-A" "unused"
[01:38:56] ------------------------------------------
[01:38:56] 
[01:38:56] ------------------------------------------
[01:38:56] stderr:
[01:38:56] stderr:
[01:38:56] ------------------------------------------
[01:38:56] error: lifetime may not live long enough
[01:38:56]   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs:8:31
[01:38:56]    |
[01:38:56] LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
[01:38:56]    |                    -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
[01:38:56]    |                    let's call the lifetime of this reference `'1`
[01:38:56]    |                    let's call the lifetime of this reference `'1`
[01:38:56] help: to allow this impl Trait to capture borrowed data with lifetime `'1`, add `'_` as a constraint
[01:38:56]    |
[01:38:56] LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self } //~ ERROR cannot infer an appropriate lifetime
[01:38:56] 
[01:38:56] error: aborting due to previous error
[01:38:56] 
[01:38:56] 
---
[01:38:56] - error[E0623]: lifetime mismatch
[01:38:56] + error: lifetime may not live long enough
[01:38:56] 2   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:8:46
[01:38:56] 3    |
[01:38:56] 4 LL |     fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
[01:38:56] 
[01:38:56] -    |                              ----     ----   ^ ...but data from `f` is returned here
[01:38:56] -    |                              this parameter and the return type are declared with different lifetimes...
[01:38:56] -    |                              this parameter and the return type are declared with different lifetimes...
[01:38:56] +    |                    -         -               ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
[01:38:56] +    |                    |         let's call the lifetime of this reference `'1`
[01:38:56] +    |                    let's call the lifetime of this reference `'2`
[01:38:56] 8 
[01:38:56] - error[E0623]: lifetime mismatch
[01:38:56] - error[E0623]: lifetime mismatch
[01:38:56] -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:10:76
[01:38:56] + error: lifetime may not live long enough
[01:38:56] +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:10:69
[01:38:56] 11    |
[01:38:56] 12 LL |     fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
[01:38:56] -    |                               ----              -----------------          ^ ...but data from `f` is returned here
[01:38:56] -    |                               this parameter and the return type are declared with different lifetimes...
[01:38:56] -    |                               this parameter and the return type are declared with different lifetimes...
[01:38:56] +    |                    -          -                                     ^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
[01:38:56] +    |                    |          let's call the lifetime of this reference `'1`
[01:38:56] +    |                    let's call the lifetime of this reference `'2`
[01:38:56] 16 
[01:38:56] - error[E0623]: lifetime mismatch
[01:38:56] - error[E0623]: lifetime mismatch
[01:38:56] + error: lifetime may not live long enough
[01:38:56] 18   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch.rs:15:58
[01:38:56] 19    |
[01:38:56] 20 LL |     fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
[01:38:56] 
[01:38:56] -    |                                         ------     ---   ^^^ ...but data from `arg` is returned here
[01:38:56] -    |                                         this parameter and the return type are declared with different lifetimes...
[01:38:56] -    |                                         this parameter and the return type are declared with different lifetimes...
[01:38:56] +    |            --  ---- has type `std::pin::Pin<&'1 Foo>`    ^^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
[01:38:56] +    |            lifetime `'a` defined here
[01:38:56] 24 
[01:38:56] 25 error: aborting due to 3 previous errors
[01:38:56] 26 
[01:38:56] 26 
[01:38:56] 
[01:38:56] 
[01:38:56] The actual stderr differed from the expected stderr.
[01:38:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll/arbitrary_self_types_pin_lifetime_mismatch.nll.stderr
[01:38:56] To update references, rerun the tests and pass the `--bless` flag
[01:38:56] To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch.rs`
[01:38:56] error: 1 errors occurred comparing output.
[01:38:56] status: exit code: 1
[01:38:56] status: exit code: 1
[01:38:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll" "-Zborrowck=mir" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.nll/auxiliary" "-A" "unused"
[01:38:56] ------------------------------------------
[01:38:56] 
[01:38:56] ------------------------------------------
[01:38:56] stderr:
[01:38:56] stderr:
[01:38:56] ------------------------------------------
[01:38:56] error: lifetime may not live long enough
[01:38:56]   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:8:46
[01:38:56]    |
[01:38:56] LL |     fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0623
[01:38:56]    |                    -         -               ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
[01:38:56]    |                    |         let's call the lifetime of this reference `'1`
[01:38:56]    |                    let's call the lifetime of this reference `'2`
[01:38:56] 
[01:38:56] error: lifetime may not live long enough
[01:38:56] error: lifetime may not live long enough
[01:38:56]   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:10:69
[01:38:56]    |
[01:38:56] LL |     fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) } //~ ERROR E0623
[01:38:56]    |                    -          -                                     ^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
[01:38:56]    |                    |          let's call the lifetime of this reference `'1`
[01:38:56]    |                    let's call the lifetime of this reference `'2`
[01:38:56] 
[01:38:56] error: lifetime may not live long enough
[01:38:56] error: lifetime may not live long enough
[01:38:56]   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch.rs:15:58
[01:38:56]    |
[01:38:56] LL |     fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
[01:38:56]    |            --  ---- has type `std::pin::Pin<&'1 Foo>`    ^^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
[01:38:56]    |            lifetime `'a` defined here
[01:38:56] 
[01:38:56] error: aborting due to 3 previous errors
[01:38:56] 
---
[01:38:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:38:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:56] 
[01:38:56] 
[01:38:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:38:56] 
[01:38:56] 
[01:38:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:56] Build completed unsuccessfully in 0:09:05
[01:38:56] Build completed unsuccessfully in 0:09:05
[01:38:56] Makefile:48: recipe for target 'check' failed
[01:38:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:096b85be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun 28 19:15:31 UTC 2019
---
travis_time:end:00238fbf:start=1561749333574252833,finish=1561749333579537738,duration=5284905
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13a645bd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05c78260
travis_time:start:05c78260
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e6a87c6
$ dmesg | grep -i kill
