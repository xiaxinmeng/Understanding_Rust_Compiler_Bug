plain
[00:55:59] 56 
[00:55:59] 
[00:55:59] 
[00:55:59] The actual stderr differed from the expected stderr.
[00:55:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/feature-gate-async-await.stderr
[00:55:59] To update references, rerun the tests and pass the `--bless` flag
[00:55:59] To only update this specific test, also pass `--test-args feature-gates/feature-gate-async-await.rs`
[00:55:59] error: 1 errors occurred comparing output.
[00:55:59] status: exit code: 1
[00:55:59] status: exit code: 1
[00:55:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-async-await.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/auxiliary" "-A" "unused"
[00:55:59] ------------------------------------------
[00:55:59] 
[00:55:59] ------------------------------------------
[00:55:59] stderr:
[00:55:59] stderr:
[00:55:59] ------------------------------------------
[00:55:59] error[E0706]: trait fns cannot be declared `async`
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     async fn foo(); //~ ERROR trait fns cannot be declared `async`
[00:55:59] 
[00:55:59] 
[00:55:59] error[E0658]: async fn is unstable
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     async fn foo() {} //~ ERROR async fn is unstable
[00:55:59]    |
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = help: add #![feature(async_await)] to the crate attributes to enable
[00:55:59] 
[00:55:59] error[E0658]: async fn is unstable
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     async fn foo(); //~ ERROR trait fns cannot be declared `async`
[00:55:59]    |
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = help: add #![feature(async_await)] to the crate attributes to enable
[00:55:59] 
[00:55:59] error[E0658]: async fn is unstable
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL | async fn foo() {} //~ ERROR async fn is unstable
[00:55:59]    |
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = help: add #![feature(async_await)] to the crate attributes to enable
[00:55:59] 
[00:55:59] error[E0658]: async blocks are unstable
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     let _ = async {}; //~ ERROR async blocks are unstable
[00:55:59]    |
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = help: add #![feature(async_await)] to the crate attributes to enable
[00:55:59] 
[00:55:59] error[E0658]: async closures are unstable
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     let _ = async || {}; //~ ERROR async closures are unstable
[00:55:59]    |
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[00:55:59]    = help: add #![feature(async_await)] to the crate attributes to enable
[00:55:59] error: aborting due to 6 previous errors
[00:55:59] 
[00:55:59] For more information about this error, try `rustc --explain E0658`.
[00:55:59] 
---
[00:55:59] 
[00:55:59] 16 
[00:55:59] 17 error: aborting due to 3 previous errors
[00:55:59] 18 
[00:55:59] - Some errors occurred: E0425, E0601.
[00:55:59] + Some errors have detailed explanations: E0425, E0601.
[00:55:59] 21 
[00:55:59] 
[00:55:59] 
[00:55:59] The actual stderr differed from the expected stderr.
[00:55:59] The actual stderr differed from the expected stderr.
[00:55:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/issue-60057.stderr
[00:55:59] To update references, rerun the tests and pass the `--bless` flag
[00:55:59] To only update this specific test, also pass `--test-args issues/issue-60057.rs`
[00:55:59] error: 1 errors occurred comparing output.
[00:55:59] status: exit code: 1
[00:55:59] status: exit code: 1
[00:55:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60057.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/auxiliary" "-A" "unused"
[00:55:59] ------------------------------------------
[00:55:59] 
[00:55:59] ------------------------------------------
[00:55:59] stderr:
[00:55:59] stderr:
[00:55:59] ------------------------------------------
[00:55:59] error[E0425]: cannot find value `banana` in this scope
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
[00:55:59]    |                     ^^^^^^ a field by this name exists in `Self`
[00:55:59] 
[00:55:59] error[E0425]: cannot find value `banana` in this scope
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
[00:55:59]    |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
[00:55:59] 
[00:55:59] error[E0601]: `main` function not found in crate `issue_60057`
[00:55:59]    |
[00:55:59]    = note: consider adding a `main` function to `/checkout/src/test/ui/issues/issue-60057.rs`
[00:55:59] error: aborting due to 3 previous errors
[00:55:59] 
[00:55:59] Some errors have detailed explanations: E0425, E0601.
[00:55:59] For more information about an error, try `rustc --explain E0425`.
---
[00:55:59] 
[00:55:59] 20 
[00:55:59] 21 error: aborting due to 2 previous errors
[00:55:59] 22 
[00:55:59] - Some errors occurred: E0223, E0599.
[00:55:59] + Some errors have detailed explanations: E0223, E0599.
[00:55:59] 25 
[00:55:59] 
[00:55:59] 
[00:55:59] The actual stderr differed from the expected stderr.
[00:55:59] The actual stderr differed from the expected stderr.
[00:55:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/suggest-std-when-using-type.stderr
[00:55:59] To update references, rerun the tests and pass the `--bless` flag
[00:55:59] To only update this specific test, also pass `--test-args suggestions/suggest-std-when-using-type.rs`
[00:55:59] error: 1 errors occurred comparing output.
[00:55:59] status: exit code: 1
[00:55:59] status: exit code: 1
[00:55:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-std-when-using-type/auxiliary" "-A" "unused"
[00:55:59] ------------------------------------------
[00:55:59] 
[00:55:59] ------------------------------------------
[00:55:59] stderr:
[00:55:59] stderr:
[00:55:59] ------------------------------------------
[00:55:59] error[E0223]: ambiguous associated type
[00:55:59]   --> /checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs:2:14
[00:55:59]    |
[00:55:59] LL |     let pi = f32::consts::PI; //~ ERROR ambiguous associated type
[00:55:59] help: you are looking for the module in `std`, not the primitive type
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |     let pi = std::f32::consts::PI; //~ ERROR ambiguous associated type
[00:55:59] 
[00:55:59] error[E0599]: no function or associated item named `from_utf8` found for type `str` in the current scope
[00:55:59]   --> /checkout/src/test/ui/suggestions/suggest-std-when-using-type.rs:5:14
[00:55:59]    |
[00:55:59]    |
[00:55:59] LL |         str::from_utf8(bytes) //~ ERROR no function or associated item named `from_utf8` found
[00:55:59]    |              ^^^^^^^^^ function or associated item not found in `str`
[00:55:59] help: you are looking for the module in `std`, not the primitive type
[00:55:59]    |
[00:55:59] LL |         std::str::from_utf8(bytes) //~ ERROR no function or associated item named `from_utf8` found
[00:55:59] 
[00:55:59] error: aborting due to 2 previous errors
[00:55:59] 
[00:55:59] Some errors have detailed explanations: E0223, E0599.
---
[00:55:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[00:55:59] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:55:59] 
[00:55:59] 
[00:55:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:59] 
[00:55:59] 
[00:55:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:55:59] Build completed unsuccessfully in 0:52:55
---
travis_time:end:03c9430e:start=1555637242133738511,finish=1555637242140285191,duration=6546680
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09fc2cb8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:115d5c78
travis_time:start:115d5c78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03691f92
$ dmesg | grep -i kill
