plain
2019-09-30T09:08:37.2635137Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T09:08:37.2870443Z ##[command]git config gc.auto 0
2019-09-30T09:08:37.2930096Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T09:08:37.3007664Z ##[command]git config --get-all http.proxy
2019-09-30T09:08:37.3156775Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64914/merge:refs/remotes/pull/64914/merge
---
2019-09-30T10:10:16.0306454Z .................................................................................................... 1500/9078
2019-09-30T10:10:22.6108600Z .................................................................................................... 1600/9078
2019-09-30T10:10:32.3999328Z ...................................................................................................i 1700/9078
2019-09-30T10:10:40.9331682Z ..............i..................................................................................... 1800/9078
2019-09-30T10:10:47.9006650Z .........................................................................................iiiii...... 1900/9078
2019-09-30T10:11:10.1828495Z .................................................................................................... 2100/9078
2019-09-30T10:11:12.5860254Z .................................................................................................... 2200/9078
2019-09-30T10:11:15.1740146Z .................................................................................................... 2300/9078
2019-09-30T10:11:21.7264492Z .................................................................................................... 2400/9078
---
2019-09-30T10:14:20.1166850Z .............................................................................i...............i...... 4700/9078
2019-09-30T10:14:28.6985137Z .................................................................................................... 4800/9078
2019-09-30T10:14:38.5065379Z .................................................................................................... 4900/9078
2019-09-30T10:14:44.5744534Z .................................................................................................... 5000/9078
2019-09-30T10:14:55.5415107Z .................................................................ii.ii.............................. 5100/9078
2019-09-30T10:15:05.1291094Z .................................................................................................... 5300/9078
2019-09-30T10:15:14.6812604Z .................................................................................................... 5400/9078
2019-09-30T10:15:22.4661823Z ...............................i.................................................................... 5500/9078
2019-09-30T10:15:28.7682676Z .................................................................................................... 5600/9078
2019-09-30T10:15:28.7682676Z .................................................................................................... 5600/9078
2019-09-30T10:15:40.8067764Z .................................................................................................... 5700/9078
2019-09-30T10:15:52.7068154Z ..........................ii...i..ii...........i.................................................... 5800/9078
2019-09-30T10:16:14.3813758Z .................................................................................................... 6000/9078
2019-09-30T10:16:22.6585832Z .................................................................................................... 6100/9078
2019-09-30T10:16:22.6585832Z .................................................................................................... 6100/9078
2019-09-30T10:16:37.6892544Z .............................i..ii.................................................................. 6200/9078
2019-09-30T10:16:56.9651499Z ........................................................................................i........... 6400/9078
2019-09-30T10:16:59.1532955Z .................................................................................................... 6500/9078
2019-09-30T10:17:01.4889818Z ............................................................i....................................... 6600/9078
2019-09-30T10:17:04.4916693Z .................................................................................................... 6700/9078
---
2019-09-30T10:21:08.7969422Z 
2019-09-30T10:21:08.7970258Z ---- [ui] ui/borrowck/issue-64453.rs stdout ----
2019-09-30T10:21:08.7970658Z diff of stderr:
2019-09-30T10:21:08.7970930Z 
2019-09-30T10:21:08.7971226Z 5    |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `std::string::String`, which does not implement the `Copy` trait
2019-09-30T10:21:08.7971726Z 7 error[E0019]: static contains unimplemented expression type
2019-09-30T10:21:08.7972369Z -   --> $DIR/issue-64453.rs:4:39
2019-09-30T10:21:08.7973156Z +   --> $DIR/issue-64453.rs:4:31
2019-09-30T10:21:08.7973418Z 9    |
2019-09-30T10:21:08.7973418Z 9    |
2019-09-30T10:21:08.7973594Z 10 LL | static settings_dir: String = format!("");
2019-09-30T10:21:08.7974229Z +    |                               ^^^^^^^^^^^
2019-09-30T10:21:08.7974455Z +    |
2019-09-30T10:21:08.7974908Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-30T10:21:08.7975178Z 12 
2019-09-30T10:21:08.7975178Z 12 
2019-09-30T10:21:08.7975358Z 13 error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-30T10:21:08.7975738Z 14   --> $DIR/issue-64453.rs:4:31
2019-09-30T10:21:08.7975951Z 
2019-09-30T10:21:08.7976111Z 
2019-09-30T10:21:08.7976281Z The actual stderr differed from the expected stderr.
2019-09-30T10:21:08.7976716Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/issue-64453.stderr
2019-09-30T10:21:08.7977143Z To update references, rerun the tests and pass the `--bless` flag
2019-09-30T10:21:08.7977999Z To only update this specific test, also pass `--test-args borrowck/issue-64453.rs`
2019-09-30T10:21:08.7978612Z error: 1 errors occurred comparing output.
2019-09-30T10:21:08.7978860Z status: exit code: 1
2019-09-30T10:21:08.7978860Z status: exit code: 1
2019-09-30T10:21:08.7980088Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-64453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/auxiliary" "-A" "unused"
2019-09-30T10:21:08.7981030Z ------------------------------------------
2019-09-30T10:21:08.7981454Z 
2019-09-30T10:21:08.7981872Z ------------------------------------------
2019-09-30T10:21:08.7982302Z stderr:
2019-09-30T10:21:08.7982302Z stderr:
2019-09-30T10:21:08.7982709Z ------------------------------------------
2019-09-30T10:21:08.7983137Z error[E0507]: cannot move out of static item `settings_dir`
2019-09-30T10:21:08.7983705Z   --> /checkout/src/test/ui/borrowck/issue-64453.rs:15:37
2019-09-30T10:21:08.7983957Z    |
2019-09-30T10:21:08.7984143Z LL |     let settings_data = from_string(settings_dir);
2019-09-30T10:21:08.7984356Z    |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `std::string::String`, which does not implement the `Copy` trait
2019-09-30T10:21:08.7984709Z error[E0019]: static contains unimplemented expression type
2019-09-30T10:21:08.7985082Z   --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
2019-09-30T10:21:08.7985331Z    |
2019-09-30T10:21:08.7985331Z    |
2019-09-30T10:21:08.7985501Z LL | static settings_dir: String = format!("");
2019-09-30T10:21:08.7985855Z    |
2019-09-30T10:21:08.7986291Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-30T10:21:08.7986522Z 
2019-09-30T10:21:08.7986739Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-30T10:21:08.7986739Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-30T10:21:08.7987145Z   --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
2019-09-30T10:21:08.7987408Z    |
2019-09-30T10:21:08.7987589Z LL | static settings_dir: String = format!("");
2019-09-30T10:21:08.7988453Z    |
2019-09-30T10:21:08.7989049Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-30T10:21:08.7989341Z 
2019-09-30T10:21:08.7989608Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-30T10:21:08.7989608Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-30T10:21:08.7990104Z   --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
2019-09-30T10:21:08.7990416Z    |
2019-09-30T10:21:08.7990667Z LL | static settings_dir: String = format!("");
2019-09-30T10:21:08.7991107Z    |
2019-09-30T10:21:08.7991796Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-30T10:21:08.7992029Z 
2019-09-30T10:21:08.7992219Z error: aborting due to 4 previous errors
---
2019-09-30T10:21:08.8006322Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-30T10:21:08.8006861Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-30T10:21:08.8019620Z 
2019-09-30T10:21:08.8020031Z 
2019-09-30T10:21:08.8025815Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-30T10:21:08.8026240Z 
2019-09-30T10:21:08.8026264Z 
2019-09-30T10:21:08.8032029Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-30T10:21:08.8032386Z Build completed unsuccessfully in 1:05:26
2019-09-30T10:21:08.8032386Z Build completed unsuccessfully in 1:05:26
2019-09-30T10:21:08.8077570Z == clock drift check ==
2019-09-30T10:21:08.8091649Z   local time: Mon Sep 30 10:21:08 UTC 2019
2019-09-30T10:21:09.0740958Z   network time: Mon, 30 Sep 2019 10:21:09 GMT
2019-09-30T10:21:09.0741086Z == end clock drift check ==
2019-09-30T10:21:10.3631504Z ##[error]Bash exited with code '1'.
2019-09-30T10:21:10.3676582Z ##[section]Starting: Checkout
2019-09-30T10:21:10.3679672Z ==============================================================================
2019-09-30T10:21:10.3679738Z Task         : Get sources
2019-09-30T10:21:10.3679806Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
