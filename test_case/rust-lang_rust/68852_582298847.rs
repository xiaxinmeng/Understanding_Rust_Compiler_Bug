plain
2020-02-05T08:36:23.5808991Z 
2020-02-05T08:36:23.5809085Z 1 error[E0106]: missing lifetime specifier
2020-02-05T08:36:23.5809337Z 2   --> $DIR/issue-63388-2.rs:12:10
2020-02-05T08:36:23.5809415Z 3    |
2020-02-05T08:36:23.5809640Z + LL |         foo: &dyn Foo, bar: &'a dyn Foo
2020-02-05T08:36:23.5810076Z 4 LL |     ) -> &dyn Foo
2020-02-05T08:36:23.5810305Z 5    |          ^ help: consider using the named lifetime: `&'a`
2020-02-05T08:36:23.5810387Z 6    |
2020-02-05T08:36:23.5810418Z 
2020-02-05T08:36:23.5810418Z 
2020-02-05T08:36:23.5810448Z 
2020-02-05T08:36:23.5810522Z The actual stderr differed from the expected stderr.
2020-02-05T08:36:23.5810841Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll/issue-63388-2.nll.stderr
2020-02-05T08:36:23.5811116Z To update references, rerun the tests and pass the `--bless` flag
2020-02-05T08:36:23.5811404Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-2.rs`
2020-02-05T08:36:23.5811521Z error: 1 errors occurred comparing output.
2020-02-05T08:36:23.5811599Z status: exit code: 1
2020-02-05T08:36:23.5811599Z status: exit code: 1
2020-02-05T08:36:23.5812510Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2.nll/auxiliary" "-A" "unused"
2020-02-05T08:36:23.5813186Z ------------------------------------------
2020-02-05T08:36:23.5813250Z 
2020-02-05T08:36:23.5813453Z ------------------------------------------
2020-02-05T08:36:23.5813538Z stderr:
2020-02-05T08:36:23.5813538Z stderr:
2020-02-05T08:36:23.5813736Z ------------------------------------------
2020-02-05T08:36:23.5813825Z error[E0106]: missing lifetime specifier
2020-02-05T08:36:23.5814063Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-2.rs:12:10
2020-02-05T08:36:23.5814148Z    |
2020-02-05T08:36:23.5814373Z LL |         foo: &dyn Foo, bar: &'a dyn Foo //~ ERROR cannot infer
2020-02-05T08:36:23.5814604Z    |              --------       -----------
2020-02-05T08:36:23.5814827Z LL |     ) -> &dyn Foo //~ ERROR missing lifetime specifier
2020-02-05T08:36:23.5815078Z    |          ^ help: consider using the named lifetime: `&'a`
2020-02-05T08:36:23.5815456Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `foo` or `bar`
2020-02-05T08:36:23.5815540Z 
2020-02-05T08:36:23.5815594Z error: aborting due to previous error
2020-02-05T08:36:23.5815632Z 
---
2020-02-05T08:36:23.5848164Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-05T08:36:23.5848430Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-05T08:36:23.5868137Z 
2020-02-05T08:36:23.5868214Z 
2020-02-05T08:36:23.5870169Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-02-05T08:36:23.5870745Z 
2020-02-05T08:36:23.5870779Z 
2020-02-05T08:36:23.5883610Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-05T08:36:23.5883699Z Build completed unsuccessfully in 1:44:03
2020-02-05T08:36:23.5883699Z Build completed unsuccessfully in 1:44:03
2020-02-05T08:36:23.5935667Z == clock drift check ==
2020-02-05T08:36:23.5949592Z   local time: Wed Feb  5 08:36:23 UTC 2020
2020-02-05T08:36:24.1167756Z   network time: Wed, 05 Feb 2020 08:36:24 GMT
2020-02-05T08:36:24.1171815Z == end clock drift check ==
2020-02-05T08:36:24.5248440Z 
2020-02-05T08:36:24.5352648Z ##[error]Bash exited with code '1'.
2020-02-05T08:36:24.5416284Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-05T08:36:24.5418819Z ==============================================================================
2020-02-05T08:36:24.5418896Z Task         : Get sources
2020-02-05T08:36:24.5418990Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
