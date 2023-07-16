plain
2020-01-06T13:01:32.6489512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T13:01:32.6677788Z ##[command]git config gc.auto 0
2020-01-06T13:01:32.6748501Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T13:01:32.6841172Z ##[command]git config --get-all http.proxy
2020-01-06T13:01:32.6993958Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67930/merge:refs/remotes/pull/67930/merge
---
2020-01-06T13:56:28.9131415Z .................................................................................................... 1500/9477
2020-01-06T13:56:34.8733943Z .................................................................................................... 1600/9477
2020-01-06T13:56:39.8517836Z .................................................................................................... 1700/9477
2020-01-06T13:56:49.2549383Z .................................................................................................... 1800/9477
2020-01-06T13:56:57.6970333Z .i.................................................................................................. 1900/9477
2020-01-06T13:57:04.5752299Z .........................................................................................iiiii...... 2000/9477
2020-01-06T13:57:26.6409551Z .................................................................................................... 2200/9477
2020-01-06T13:57:29.0564607Z .................................................................................................... 2300/9477
2020-01-06T13:57:31.5328124Z .................................................................................................... 2400/9477
2020-01-06T13:57:37.5981076Z .................................................................................................... 2500/9477
---
2020-01-06T14:00:39.2844645Z .....................i...............i........................F.FFF................................. 4900/9477
2020-01-06T14:00:49.2915061Z .................................................................................................... 5000/9477
2020-01-06T14:00:55.6010724Z ..................................................................i................................. 5100/9477
2020-01-06T14:01:03.7770640Z .................................................................................................... 5200/9477
2020-01-06T14:01:11.5588273Z .................................ii.ii...........i.................................................. 5300/9477
2020-01-06T14:01:21.1944797Z .................................................................................................... 5500/9477
2020-01-06T14:01:31.2166312Z .................................................................................................... 5600/9477
2020-01-06T14:01:38.5508323Z .................i.................................................................................. 5700/9477
2020-01-06T14:01:44.8391002Z .................................................................................................... 5800/9477
2020-01-06T14:01:44.8391002Z .................................................................................................... 5800/9477
2020-01-06T14:01:56.1815373Z .................................................................................................... 5900/9477
2020-01-06T14:02:07.5894706Z .......ii...i..ii...........i....................................................................... 6000/9477
2020-01-06T14:02:25.1978524Z .................................................................................................... 6200/9477
2020-01-06T14:02:33.0906254Z .................................................................................................... 6300/9477
2020-01-06T14:02:33.0906254Z .................................................................................................... 6300/9477
2020-01-06T14:02:51.5102177Z ..................................i..ii............................................................. 6400/9477
2020-01-06T14:03:12.1931203Z .................................................................................................... 6600/9477
2020-01-06T14:03:14.3342646Z .........i.......................................................................................... 6700/9477
2020-01-06T14:03:16.6013108Z .................................................................................................... 6800/9477
2020-01-06T14:03:19.2358799Z .........i.......................................................................................... 6900/9477
---
2020-01-06T14:04:57.5905044Z .................................................................................................... 7500/9477
2020-01-06T14:05:01.7546607Z .................................................................................................... 7600/9477
2020-01-06T14:05:07.0919242Z .................................................................................................... 7700/9477
2020-01-06T14:05:17.9207319Z .................................................................................................... 7800/9477
2020-01-06T14:05:26.2034163Z ..............................................iiii.................................................. 7900/9477
2020-01-06T14:05:41.2567780Z .................................................................................................... 8100/9477
2020-01-06T14:05:47.8544416Z .................................................................................................... 8200/9477
2020-01-06T14:06:03.4534906Z .................................................................................................... 8300/9477
2020-01-06T14:06:11.0503588Z .................................................................................................... 8400/9477
---
2020-01-06T14:08:08.9878499Z diff of stderr:
2020-01-06T14:08:08.9878639Z 
2020-01-06T14:08:08.9879044Z 2   --> $DIR/result-as_deref_err.rs:4:28
2020-01-06T14:08:08.9879238Z 3    |
2020-01-06T14:08:08.9879373Z 4 LL |     let _result = &Err(41).as_deref_err();
2020-01-06T14:08:08.9880213Z -    |                            ^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_ok`
2020-01-06T14:08:08.9880679Z 6    |
2020-01-06T14:08:08.9880839Z 7    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9880839Z 7    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9880985Z 8            `{integer} : std::ops::Deref`
2020-01-06T14:08:08.9881244Z 
2020-01-06T14:08:08.9881400Z The actual stderr differed from the expected stderr.
2020-01-06T14:08:08.9881960Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/result-as_deref_err.stderr
2020-01-06T14:08:08.9881960Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/result-as_deref_err.stderr
2020-01-06T14:08:08.9882410Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T14:08:08.9882954Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_err.rs`
2020-01-06T14:08:08.9883306Z error: 1 errors occurred comparing output.
2020-01-06T14:08:08.9883448Z status: exit code: 1
2020-01-06T14:08:08.9883448Z status: exit code: 1
2020-01-06T14:08:08.9884575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err/auxiliary" "-A" "unused"
2020-01-06T14:08:08.9885235Z ------------------------------------------
2020-01-06T14:08:08.9885416Z 
2020-01-06T14:08:08.9885792Z ------------------------------------------
2020-01-06T14:08:08.9885965Z stderr:
2020-01-06T14:08:08.9885965Z stderr:
2020-01-06T14:08:08.9886351Z ------------------------------------------
2020-01-06T14:08:08.9886550Z error[E0599]: no method named `as_deref_err` found for type `std::result::Result<_, {integer}>` in the current scope
2020-01-06T14:08:08.9887221Z    |
2020-01-06T14:08:08.9887221Z    |
2020-01-06T14:08:08.9887359Z LL |     let _result = &Err(41).as_deref_err();
2020-01-06T14:08:08.9887671Z    |
2020-01-06T14:08:08.9887816Z    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9887816Z    = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9887976Z            `{integer} : std::ops::Deref`
2020-01-06T14:08:08.9888272Z error: aborting due to previous error
2020-01-06T14:08:08.9888391Z 
2020-01-06T14:08:08.9888797Z For more information about this error, try `rustc --explain E0599`.
2020-01-06T14:08:08.9888959Z 
---
2020-01-06T14:08:08.9894356Z 4 LL |     let _result = &mut Err(41).as_deref_mut_err();
2020-01-06T14:08:08.9894796Z -    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut_ok`
2020-01-06T14:08:08.9895016Z +    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut`
2020-01-06T14:08:08.9895156Z 6    |
2020-01-06T14:08:08.9895294Z 7    = note: the method `as_deref_mut_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9895484Z 8            `{integer} : std::ops::DerefMut`
2020-01-06T14:08:08.9895715Z 
2020-01-06T14:08:08.9895859Z The actual stderr differed from the expected stderr.
2020-01-06T14:08:08.9896350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/result-as_deref_mut_err.stderr
2020-01-06T14:08:08.9896350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/result-as_deref_mut_err.stderr
2020-01-06T14:08:08.9896817Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T14:08:08.9897337Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_mut_err.rs`
2020-01-06T14:08:08.9897659Z error: 1 errors occurred comparing output.
2020-01-06T14:08:08.9897798Z status: exit code: 1
2020-01-06T14:08:08.9897798Z status: exit code: 1
2020-01-06T14:08:08.9899073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err/auxiliary" "-A" "unused"
2020-01-06T14:08:08.9899995Z ------------------------------------------
2020-01-06T14:08:08.9900243Z 
2020-01-06T14:08:08.9900654Z ------------------------------------------
2020-01-06T14:08:08.9900843Z stderr:
2020-01-06T14:08:08.9900843Z stderr:
2020-01-06T14:08:08.9901200Z ------------------------------------------
2020-01-06T14:08:08.9901390Z error[E0599]: no method named `as_deref_mut_err` found for type `std::result::Result<_, {integer}>` in the current scope
2020-01-06T14:08:08.9902027Z    |
2020-01-06T14:08:08.9902196Z LL |     let _result = &mut Err(41).as_deref_mut_err();
2020-01-06T14:08:08.9902345Z    |                                ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_mut`
2020-01-06T14:08:08.9902516Z    |
2020-01-06T14:08:08.9902516Z    |
2020-01-06T14:08:08.9902670Z    = note: the method `as_deref_mut_err` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9902813Z            `{integer} : std::ops::DerefMut`
2020-01-06T14:08:08.9903061Z error: aborting due to previous error
2020-01-06T14:08:08.9903175Z 
2020-01-06T14:08:08.9903565Z For more information about this error, try `rustc --explain E0599`.
2020-01-06T14:08:08.9903727Z 
2020-01-06T14:08:08.9903727Z 
2020-01-06T14:08:08.9904101Z ------------------------------------------
2020-01-06T14:08:08.9904273Z 
2020-01-06T14:08:08.9904388Z 
2020-01-06T14:08:08.9904782Z ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs stdout ----
2020-01-06T14:08:08.9909285Z diff of stderr:
2020-01-06T14:08:08.9909338Z 
2020-01-06T14:08:08.9909380Z 3    |
2020-01-06T14:08:08.9909426Z 4 LL |     let _result = &mut Ok(42).as_deref_mut_ok();
2020-01-06T14:08:08.9910222Z -    |
2020-01-06T14:08:08.9910519Z -    = note: the method `as_deref_mut_ok` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9910519Z -    = note: the method `as_deref_mut_ok` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9910771Z -            `{integer} : std::ops::DerefMut`
2020-01-06T14:08:08.9910860Z 10 error: aborting due to previous error
2020-01-06T14:08:08.9910919Z 11 
2020-01-06T14:08:08.9910948Z 
2020-01-06T14:08:08.9910973Z 
2020-01-06T14:08:08.9910973Z 
2020-01-06T14:08:08.9911017Z The actual stderr differed from the expected stderr.
2020-01-06T14:08:08.9911382Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok/result-as_deref_mut_ok.stderr
2020-01-06T14:08:08.9912085Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T14:08:08.9912398Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs`
2020-01-06T14:08:08.9912511Z error: 1 errors occurred comparing output.
2020-01-06T14:08:08.9912556Z status: exit code: 1
2020-01-06T14:08:08.9912556Z status: exit code: 1
2020-01-06T14:08:08.9913586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok/auxiliary" "-A" "unused"
2020-01-06T14:08:08.9913944Z ------------------------------------------
2020-01-06T14:08:08.9913978Z 
2020-01-06T14:08:08.9914195Z ------------------------------------------
2020-01-06T14:08:08.9914256Z stderr:
2020-01-06T14:08:08.9914256Z stderr:
2020-01-06T14:08:08.9914468Z ------------------------------------------
2020-01-06T14:08:08.9914525Z error[E0599]: no method named `as_deref_mut_ok` found for type `std::result::Result<{integer}, _>` in the current scope
2020-01-06T14:08:08.9914873Z    |
2020-01-06T14:08:08.9914873Z    |
2020-01-06T14:08:08.9914919Z LL |     let _result = &mut Ok(42).as_deref_mut_ok();
2020-01-06T14:08:08.9915036Z 
2020-01-06T14:08:08.9915078Z error: aborting due to previous error
2020-01-06T14:08:08.9915106Z 
2020-01-06T14:08:08.9915369Z For more information about this error, try `rustc --explain E0599`.
---
2020-01-06T14:08:08.9915952Z ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs stdout ----
2020-01-06T14:08:08.9916002Z diff of stderr:
2020-01-06T14:08:08.9916030Z 
2020-01-06T14:08:08.9916084Z 3    |
2020-01-06T14:08:08.9916128Z 4 LL |     let _result = &Ok(42).as_deref_ok();
2020-01-06T14:08:08.9916370Z -    |
2020-01-06T14:08:08.9916655Z -    = note: the method `as_deref_ok` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9916655Z -    = note: the method `as_deref_ok` exists but the following trait bounds were not satisfied:
2020-01-06T14:08:08.9917015Z -            `{integer} : std::ops::Deref`
2020-01-06T14:08:08.9917132Z 10 error: aborting due to previous error
2020-01-06T14:08:08.9917172Z 11 
2020-01-06T14:08:08.9917269Z 
2020-01-06T14:08:08.9917294Z 
2020-01-06T14:08:08.9917294Z 
2020-01-06T14:08:08.9917356Z The actual stderr differed from the expected stderr.
2020-01-06T14:08:08.9917737Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok/result-as_deref_ok.stderr
2020-01-06T14:08:08.9917990Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T14:08:08.9918348Z To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs`
2020-01-06T14:08:08.9918434Z error: 1 errors occurred comparing output.
2020-01-06T14:08:08.9918498Z status: exit code: 1
2020-01-06T14:08:08.9918498Z status: exit code: 1
2020-01-06T14:08:08.9922257Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok/auxiliary" "-A" "unused"
2020-01-06T14:08:08.9922907Z ------------------------------------------
2020-01-06T14:08:08.9922945Z 
2020-01-06T14:08:08.9923189Z ------------------------------------------
2020-01-06T14:08:08.9923234Z stderr:
2020-01-06T14:08:08.9923234Z stderr:
2020-01-06T14:08:08.9923448Z ------------------------------------------
2020-01-06T14:08:08.9923533Z error[E0599]: no method named `as_deref_ok` found for type `std::result::Result<{integer}, _>` in the current scope
2020-01-06T14:08:08.9923876Z    |
2020-01-06T14:08:08.9923876Z    |
2020-01-06T14:08:08.9923939Z LL |     let _result = &Ok(42).as_deref_ok();
2020-01-06T14:08:08.9924026Z 
2020-01-06T14:08:08.9924087Z error: aborting due to previous error
2020-01-06T14:08:08.9924115Z 
2020-01-06T14:08:08.9924362Z For more information about this error, try `rustc --explain E0599`.
---
2020-01-06T14:08:08.9926419Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-06T14:08:08.9926476Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-06T14:08:08.9926508Z 
2020-01-06T14:08:08.9926532Z 
2020-01-06T14:08:08.9928257Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-06T14:08:08.9928593Z 
2020-01-06T14:08:08.9928622Z 
2020-01-06T14:08:08.9928708Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-06T14:08:08.9928760Z Build completed unsuccessfully in 1:00:08
2020-01-06T14:08:08.9928760Z Build completed unsuccessfully in 1:00:08
2020-01-06T14:08:08.9973794Z == clock drift check ==
2020-01-06T14:08:08.9996330Z   local time: Mon Jan  6 14:08:08 UTC 2020
2020-01-06T14:08:09.2842637Z   network time: Mon, 06 Jan 2020 14:08:09 GMT
2020-01-06T14:08:09.2842765Z == end clock drift check ==
2020-01-06T14:08:10.2712382Z 
2020-01-06T14:08:10.2805050Z ##[error]Bash exited with code '1'.
2020-01-06T14:08:10.2838558Z ##[section]Starting: Checkout
2020-01-06T14:08:10.2840676Z ==============================================================================
2020-01-06T14:08:10.2840756Z Task         : Get sources
2020-01-06T14:08:10.2840804Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
