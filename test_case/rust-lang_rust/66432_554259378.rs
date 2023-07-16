plain
2019-11-15T07:57:59.6647018Z test [ui] ui/parser/issue-5544-a.rs ... ok
2019-11-15T07:57:59.6892347Z test [ui] ui/parser/issue-33418.rs ... ok
2019-11-15T07:57:59.7119121Z test [ui] ui/parser/issue-5544-b.rs ... ok
2019-11-15T07:57:59.7136146Z test [ui] ui/parser/issue-5806.rs ... ok
2019-11-15T07:57:59.7375841Z test [ui] ui/parser/issue-58094-missing-right-square-bracket.rs ... ok
2019-11-15T07:57:59.7791300Z test [ui] ui/parser/issue-59418.rs ... ok
2019-11-15T07:57:59.8160907Z test [ui] ui/parser/issue-62660.rs ... ok
2019-11-15T07:57:59.8327346Z test [ui] ui/parser/issue-62881.rs ... ok
2019-11-15T07:57:59.8558191Z test [ui] ui/parser/issue-62913.rs ... ok
---
2019-11-15T08:10:44.0046600Z test [ui (nll)] ui/parser/issue-41155.rs ... ok
2019-11-15T08:10:44.0536743Z test [ui (nll)] ui/parser/issue-5544-a.rs ... ok
2019-11-15T08:10:44.0537298Z test [ui (nll)] ui/parser/issue-5544-b.rs ... ok
2019-11-15T08:10:44.0794678Z test [ui (nll)] ui/parser/issue-5806.rs ... ok
2019-11-15T08:10:44.0795880Z test [ui (nll)] ui/parser/issue-58094-missing-right-square-bracket.rs ... ok
2019-11-15T08:10:44.1424030Z test [ui (nll)] ui/parser/issue-59418.rs ... ok
2019-11-15T08:10:44.1687599Z test [ui (nll)] ui/parser/issue-62660.rs ... ok
2019-11-15T08:10:44.1967063Z test [ui (nll)] ui/parser/issue-62881.rs ... ok
2019-11-15T08:10:44.2282825Z test [ui (nll)] ui/parser/issue-62913.rs ... ok
---
2019-11-15T08:14:55.2120588Z - error[E0308]: mismatched types
2019-11-15T08:14:55.2121175Z + error: higher-ranked subtype error
2019-11-15T08:14:55.2121757Z 2   --> $DIR/issue-30906.rs:15:5
2019-11-15T08:14:55.2122088Z 3    |
2019-11-15T08:14:55.2122353Z 4 LL |     test(Compose(f, |_| {}));
2019-11-15T08:14:55.2123434Z -    |     ^^^^ one type is more general than the other
2019-11-15T08:14:55.2123990Z -    |
2019-11-15T08:14:55.2123990Z -    |
2019-11-15T08:14:55.2124568Z -    = note: expected type `std::ops::FnOnce<(&'x str,)>`
2019-11-15T08:14:55.2125188Z -               found type `std::ops::FnOnce<(&str,)>`
2019-11-15T08:14:55.2125793Z 9 
2019-11-15T08:14:55.2126044Z 10 error: aborting due to previous error
2019-11-15T08:14:55.2126288Z 11 
2019-11-15T08:14:55.2126484Z 
2019-11-15T08:14:55.2126484Z 
2019-11-15T08:14:55.2127031Z - For more information about this error, try `rustc --explain E0308`.
2019-11-15T08:14:55.2127379Z 13 
2019-11-15T08:14:55.2127586Z 
2019-11-15T08:14:55.2127790Z 
2019-11-15T08:14:55.2128051Z The actual stderr differed from the expected stderr.
2019-11-15T08:14:55.2129108Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/issue-30906.nll/issue-30906.nll.stderr
2019-11-15T08:14:55.2129787Z To update references, rerun the tests and pass the `--bless` flag
2019-11-15T08:14:55.2130472Z To only update this specific test, also pass `--test-args unboxed-closures/issue-30906.rs`
2019-11-15T08:14:55.2131068Z error: 1 errors occurred comparing output.
2019-11-15T08:14:55.2131341Z status: exit code: 1
2019-11-15T08:14:55.2131341Z status: exit code: 1
2019-11-15T08:14:55.2132570Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/issue-30906.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/issue-30906.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/issue-30906.nll/auxiliary" "-A" "unused"
2019-11-15T08:14:55.2133672Z ------------------------------------------
2019-11-15T08:14:55.2133966Z 
2019-11-15T08:14:55.2134499Z ------------------------------------------
2019-11-15T08:14:55.2134841Z stderr:
2019-11-15T08:14:55.2134841Z stderr:
2019-11-15T08:14:55.2135326Z ------------------------------------------
2019-11-15T08:14:55.2136601Z error: higher-ranked subtype error
2019-11-15T08:14:55.2137311Z   --> /checkout/src/test/ui/unboxed-closures/issue-30906.rs:15:5
2019-11-15T08:14:55.2137581Z    |
2019-11-15T08:14:55.2137762Z LL |     test(Compose(f, |_| {})); //~ ERROR: mismatched types
2019-11-15T08:14:55.2138096Z 
2019-11-15T08:14:55.2138274Z error: aborting due to previous error
2019-11-15T08:14:55.2138415Z 
2019-11-15T08:14:55.2138543Z 
---
2019-11-15T08:14:55.2169557Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-15T08:14:55.2170712Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-15T08:14:55.2198752Z 
2019-11-15T08:14:55.2198852Z 
2019-11-15T08:14:55.2201245Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-15T08:14:55.2204080Z 
2019-11-15T08:14:55.2204123Z 
2019-11-15T08:14:55.2204271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-15T08:14:55.2204359Z Build completed unsuccessfully in 1:44:24
2019-11-15T08:14:55.2204359Z Build completed unsuccessfully in 1:44:24
2019-11-15T08:14:55.2257838Z == clock drift check ==
2019-11-15T08:14:55.2271133Z   local time: Fri Nov 15 08:14:55 UTC 2019
2019-11-15T08:14:57.3026719Z   network time: == end clock drift check ==
2019-11-15T08:14:58.1162164Z 
2019-11-15T08:14:58.1298238Z ##[error]Bash exited with code '1'.
2019-11-15T08:14:58.1339511Z ##[section]Starting: Checkout
2019-11-15T08:14:58.1342061Z ==============================================================================
2019-11-15T08:14:58.1342157Z Task         : Get sources
2019-11-15T08:14:58.1342259Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
