plain
travis_time:end:1d74860f:start=1560601873690909352,finish=1560601874503988692,duration=813079340
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:52] .................................................................................................... 1200/5680
[00:55:54] .................................................................................................... 1300/5680
[00:55:56] .................................................................................................... 1400/5680
[00:55:59] .................................................................................................... 1500/5680
[00:56:02] ...........................F........................................................................ 1600/5680
[00:56:08] ..i................................................................................................. 1800/5680
[00:56:12] .................................................................................................... 1900/5680
[00:56:15] .................................................................................................... 2000/5680
[00:56:19] .................................................................................................... 2100/5680
---
[00:58:41] 
[00:58:41] + error[E0308]: mismatched types
[00:58:41] +   --> $DIR/fat-ptr-cast.rs:22:41
[00:58:41] +    |
[00:58:41] + LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null();
[00:58:41] +    |                                         ^^^^^^^^^^^^^^^^^ types differ in mutability
[00:58:41] +    |
[00:58:41] +    = note: expected type `*mut (dyn Trait + 'static)`
[00:58:41] +               found type `*const _`
[00:58:41] + error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:58:41] +   --> $DIR/fat-ptr-cast.rs:23:32
[00:58:41] +    |
[00:58:41] +    |
[00:58:41] + LL |     let mut fail: *const str = core::ptr::null();
[00:58:41] +    |
[00:58:41] +    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:58:41] +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:58:41] +    = note: required by `std::ptr::null`
[00:58:41] +    = note: required by `std::ptr::null`
[00:58:41] + 
[00:58:41] 1 error[E0606]: casting `&[i32]` as `usize` is invalid
[00:58:41] 2   --> $DIR/fat-ptr-cast.rs:10:5
[00:58:41] 
[00:58:41] 
[00:58:41] 52 LL |     q as *const [i32];
[00:58:41] 54 
[00:58:41] 54 
[00:58:41] - error[E0606]: casting `usize` as `*mut (dyn Trait + 'static)` is invalid
[00:58:41] -   --> $DIR/fat-ptr-cast.rs:22:41
[00:58:41] -    |
[00:58:41] - LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null();
[00:58:41] - 
[00:58:41] - error[E0606]: casting `usize` as `*const str` is invalid
[00:58:41] -   --> $DIR/fat-ptr-cast.rs:23:32
[00:58:41] -    |
[00:58:41] -    |
[00:58:41] - LL |     let mut fail: *const str = core::ptr::null();
[00:58:41] - 
[00:58:41] 67 error: aborting due to 9 previous errors
[00:58:41] 68 
[00:58:41] - Some errors have detailed explanations: E0605, E0606, E0607.
[00:58:41] - Some errors have detailed explanations: E0605, E0606, E0607.
[00:58:41] - For more information about an error, try `rustc --explain E0605`.
[00:58:41] + Some errors have detailed explanations: E0277, E0308, E0605, E0606, E0607.
[00:58:41] + For more information about an error, try `rustc --explain E0277`.
[00:58:41] 71 
[00:58:41] 
[00:58:41] 
[00:58:41] The actual stderr differed from the expected stderr.
[00:58:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/fat-ptr-cast.stderr
[00:58:41] To update references, rerun the tests and pass the `--bless` flag
[00:58:41] To only update this specific test, also pass `--test-args fat-ptr-cast.rs`
[00:58:41] error: 1 errors occurred comparing output.
[00:58:41] status: exit code: 1
[00:58:41] status: exit code: 1
[00:58:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fat-ptr-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/auxiliary" "-A" "unused"
[00:58:41] ------------------------------------------
[00:58:41] 
[00:58:41] ------------------------------------------
[00:58:41] stderr:
[00:58:41] stderr:
[00:58:41] ------------------------------------------
[00:58:41] error[E0308]: mismatched types
[00:58:41]   --> /checkout/src/test/ui/fat-ptr-cast.rs:22:41
[00:58:41]    |
[00:58:41] LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null(); //~ ERROR casting
[00:58:41]    |                                         ^^^^^^^^^^^^^^^^^ types differ in mutability
[00:58:41]    |
[00:58:41]    = note: expected type `*mut (dyn Trait + 'static)`
[00:58:41]               found type `*const _`
[00:58:41] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:58:41]   --> /checkout/src/test/ui/fat-ptr-cast.rs:23:32
[00:58:41]    |
[00:58:41]    |
[00:58:41] LL |     let mut fail: *const str = core::ptr::null(); //~ ERROR casting
[00:58:41]    |
[00:58:41]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:58:41]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:58:41]    = note: required by `std::ptr::null`
[00:58:41]    = note: required by `std::ptr::null`
[00:58:41] 
[00:58:41] error[E0606]: casting `&[i32]` as `usize` is invalid
[00:58:41]    |
[00:58:41] LL |     a as usize; //~ ERROR casting
[00:58:41]    |     ^^^^^^^^^^
[00:58:41]    |
[00:58:41]    |
[00:58:41]    = help: cast through a raw pointer first
[00:58:41] 
[00:58:41] error[E0606]: casting `&[i32]` as `isize` is invalid
[00:58:41]    |
[00:58:41] LL |     a as isize; //~ ERROR casting
[00:58:41]    |     ^^^^^^^^^^
[00:58:41]    |
[00:58:41]    |
[00:58:41]    = help: cast through a raw pointer first
[00:58:41] 
[00:58:41] error[E0606]: casting `&[i32]` as `i16` is invalid
[00:58:41]    |
[00:58:41]    |
[00:58:41] LL |     a as i16; //~ ERROR casting `&[i32]` as `i16` is invalid
[00:58:41]    |
[00:58:41]    = help: cast through a raw pointer first
[00:58:41] 
[00:58:41] 
[00:58:41] error[E0606]: casting `&[i32]` as `u32` is invalid
[00:58:41]    |
[00:58:41]    |
[00:58:41] LL |     a as u32; //~ ERROR casting `&[i32]` as `u32` is invalid
[00:58:41]    |
[00:58:41]    = help: cast through a raw pointer first
[00:58:41] 
[00:58:41] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[00:58:41] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[00:58:41]   --> /checkout/src/test/ui/fat-ptr-cast.rs:14:5
[00:58:41]    |
[00:58:41] LL |     b as usize; //~ ERROR non-primitive cast
[00:58:41]    |     ^^^^^^^^^^
[00:58:41]    |
[00:58:41]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:58:41] 
[00:58:41] error[E0606]: casting `*const [i32]` as `usize` is invalid
[00:58:41]    |
[00:58:41] LL |     p as usize;
[00:58:41]    |     ^^^^^^^^^^
[00:58:41]    |
[00:58:41]    |
[00:58:41]    = help: cast through a thin pointer first
[00:58:41] 
[00:58:41] error[E0607]: cannot cast thin pointer `*const i32` to fat pointer `*const [i32]`
[00:58:41]    |
[00:58:41]    |
[00:58:41] LL |     q as *const [i32]; //~ ERROR cannot cast
[00:58:41] 
[00:58:41] error: aborting due to 9 previous errors
[00:58:41] 
[00:58:41] Some errors have detailed explanations: E0277, E0308, E0605, E0606, E0607.
---
[00:58:41] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:41] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:41] 
[00:58:41] 
[00:58:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:41] 
[00:58:41] 
[00:58:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:41] Build completed unsuccessfully in 0:54:02
---
travis_time:end:06e2daaf:start=1560605408197126352,finish=1560605408204037993,duration=6911641
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2150e942
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
