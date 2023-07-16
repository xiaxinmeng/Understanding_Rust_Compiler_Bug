plain
travis_time:end:0bf2987c:start=1560619217754724721,finish=1560619219874405165,duration=2119680444
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:01] .................................................................................................... 1200/5681
[00:58:04] .................................................................................................... 1300/5681
[00:58:06] .................................................................................................... 1400/5681
[00:58:09] .................................................................................................... 1500/5681
[00:58:12] ..........................F......................................................................... 1600/5681
[00:58:18] ..i................................................................................................. 1800/5681
[00:58:22] .................................................................................................... 1900/5681
[00:58:26] .................................................................................................... 2000/5681
[00:58:29] .................................................................................................... 2100/5681
---
[01:00:56] 
[01:00:56] + error[E0277]: the size for values of type `str` cannot be known at compilation time
[01:00:56] +   --> $DIR/fat-ptr-cast.rs:23:32
[01:00:56] +    |
[01:00:56] + LL |     let mut fail: *const str = core::ptr::null();
[01:00:56] +    |
[01:00:56] +    = help: the trait `std::marker::Sized` is not implemented for `str`
[01:00:56] +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:00:56] +    = note: required by `std::ptr::null`
[01:00:56] +    = note: required by `std::ptr::null`
[01:00:56] + 
[01:00:56] 1 error[E0606]: casting `&[i32]` as `usize` is invalid
[01:00:56] 2   --> $DIR/fat-ptr-cast.rs:10:5
[01:00:56] 
[01:00:56] 
[01:00:56] 52 LL |     q as *const [i32];
[01:00:56] 54 
[01:00:56] 54 
[01:00:56] - error[E0606]: casting `usize` as `*mut (dyn Trait + 'static)` is invalid
[01:00:56] -   --> $DIR/fat-ptr-cast.rs:22:41
[01:00:56] -    |
[01:00:56] - LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null();
[01:00:56] + error: aborting due to 8 previous errors
[01:00:56] 60 
[01:00:56] - error[E0606]: casting `usize` as `*const str` is invalid
[01:00:56] -   --> $DIR/fat-ptr-cast.rs:23:32
[01:00:56] -   --> $DIR/fat-ptr-cast.rs:23:32
[01:00:56] -    |
[01:00:56] - LL |     let mut fail: *const str = core::ptr::null();
[01:00:56] - 
[01:00:56] - error: aborting due to 9 previous errors
[01:00:56] - 
[01:00:56] - Some errors have detailed explanations: E0605, E0606, E0607.
[01:00:56] - Some errors have detailed explanations: E0605, E0606, E0607.
[01:00:56] - For more information about an error, try `rustc --explain E0605`.
[01:00:56] + Some errors have detailed explanations: E0277, E0605, E0606, E0607.
[01:00:56] + For more information about an error, try `rustc --explain E0277`.
[01:00:56] 71 
[01:00:56] 
[01:00:56] 
[01:00:56] The actual stderr differed from the expected stderr.
[01:00:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/fat-ptr-cast.stderr
[01:00:56] To update references, rerun the tests and pass the `--bless` flag
[01:00:56] To only update this specific test, also pass `--test-args fat-ptr-cast.rs`
[01:00:56] error: 1 errors occurred comparing output.
[01:00:56] status: exit code: 1
[01:00:56] status: exit code: 1
[01:00:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fat-ptr-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/auxiliary" "-A" "unused"
[01:00:56] ------------------------------------------
[01:00:56] 
[01:00:56] ------------------------------------------
[01:00:56] stderr:
[01:00:56] stderr:
[01:00:56] ------------------------------------------
[01:00:56] error[E0277]: the size for values of type `str` cannot be known at compilation time
[01:00:56]   --> /checkout/src/test/ui/fat-ptr-cast.rs:23:32
[01:00:56]    |
[01:00:56] LL |     let mut fail: *const str = core::ptr::null(); //~ ERROR casting
[01:00:56]    |
[01:00:56]    = help: the trait `std::marker::Sized` is not implemented for `str`
[01:00:56]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:00:56]    = note: required by `std::ptr::null`
[01:00:56]    = note: required by `std::ptr::null`
[01:00:56] 
[01:00:56] error[E0606]: casting `&[i32]` as `usize` is invalid
[01:00:56]    |
[01:00:56] LL |     a as usize; //~ ERROR casting
[01:00:56]    |     ^^^^^^^^^^
[01:00:56]    |
[01:00:56]    |
[01:00:56]    = help: cast through a raw pointer first
[01:00:56] 
[01:00:56] error[E0606]: casting `&[i32]` as `isize` is invalid
[01:00:56]    |
[01:00:56] LL |     a as isize; //~ ERROR casting
[01:00:56]    |     ^^^^^^^^^^
[01:00:56]    |
[01:00:56]    |
[01:00:56]    = help: cast through a raw pointer first
[01:00:56] 
[01:00:56] error[E0606]: casting `&[i32]` as `i16` is invalid
[01:00:56]    |
[01:00:56]    |
[01:00:56] LL |     a as i16; //~ ERROR casting `&[i32]` as `i16` is invalid
[01:00:56]    |
[01:00:56]    = help: cast through a raw pointer first
[01:00:56] 
[01:00:56] 
[01:00:56] error[E0606]: casting `&[i32]` as `u32` is invalid
[01:00:56]    |
[01:00:56]    |
[01:00:56] LL |     a as u32; //~ ERROR casting `&[i32]` as `u32` is invalid
[01:00:56]    |
[01:00:56]    = help: cast through a raw pointer first
[01:00:56] 
[01:00:56] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[01:00:56] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[01:00:56]   --> /checkout/src/test/ui/fat-ptr-cast.rs:14:5
[01:00:56]    |
[01:00:56] LL |     b as usize; //~ ERROR non-primitive cast
[01:00:56]    |     ^^^^^^^^^^
[01:00:56]    |
[01:00:56]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[01:00:56] 
[01:00:56] error[E0606]: casting `*const [i32]` as `usize` is invalid
[01:00:56]    |
[01:00:56] LL |     p as usize;
[01:00:56]    |     ^^^^^^^^^^
[01:00:56]    |
[01:00:56]    |
[01:00:56]    = help: cast through a thin pointer first
[01:00:56] 
[01:00:56] error[E0607]: cannot cast thin pointer `*const i32` to fat pointer `*const [i32]`
[01:00:56]    |
[01:00:56]    |
[01:00:56] LL |     q as *const [i32]; //~ ERROR cannot cast
[01:00:56] 
[01:00:56] error: aborting due to 8 previous errors
[01:00:56] 
[01:00:56] Some errors have detailed explanations: E0277, E0605, E0606, E0607.
---
[01:00:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:56] 
[01:00:56] 
[01:00:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:56] 
[01:00:56] 
[01:00:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:56] Build completed unsuccessfully in 0:56:05
---
166908 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
156504 ./src/llvm-project/clang
149984 ./obj/build/bootstrap/debug/incremental
134712 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
134708 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fd6ynnsorc-60higm-3ekas8ujxant2
127152 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-toohen printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:057adb9e
travis_time:start:057adb9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03f6d614
$ dmesg | grep -i kill
