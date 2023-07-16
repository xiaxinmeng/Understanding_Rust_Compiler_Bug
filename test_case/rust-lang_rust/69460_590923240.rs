plain
2020-02-25T14:09:28.4716053Z ========================== Starting Command Output ===========================
2020-02-25T14:09:28.4719465Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8710dde4-028d-4756-9583-d06b9a032718.sh
2020-02-25T14:09:28.4719757Z 
2020-02-25T14:09:28.4723448Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T14:09:28.4744626Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69460/merge to s
2020-02-25T14:09:28.4748409Z Task         : Get sources
2020-02-25T14:09:28.4748738Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T14:09:28.4749052Z Version      : 1.0.0
2020-02-25T14:09:28.4749291Z Author       : Microsoft
---
2020-02-25T14:09:29.4617986Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T14:09:29.4623184Z ##[command]git config gc.auto 0
2020-02-25T14:09:29.4626959Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T14:09:29.4630401Z ##[command]git config --get-all http.proxy
2020-02-25T14:09:29.4636544Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69460/merge:refs/remotes/pull/69460/merge
---
2020-02-25T15:15:01.1192852Z .................................................................................................... 1700/9704
2020-02-25T15:15:05.7091504Z .................................................................................................... 1800/9704
2020-02-25T15:15:17.3275635Z ...........................................i........................................................ 1900/9704
2020-02-25T15:15:25.4277173Z .................................................................................................... 2000/9704
2020-02-25T15:15:39.7715590Z .................................iiiii.............................................................. 2100/9704
2020-02-25T15:15:49.8011823Z .................................................................................................... 2300/9704
2020-02-25T15:15:52.3140096Z .................................................................................................... 2400/9704
2020-02-25T15:15:56.7365976Z .................................................................................................... 2500/9704
2020-02-25T15:16:18.0260006Z .................................................................................................... 2600/9704
---
2020-02-25T15:19:03.0657204Z .........i.......................................................................................... 5000/9704
2020-02-25T15:19:12.3775703Z .................................................................................................... 5100/9704
2020-02-25T15:19:17.1791881Z ....................................i............................................................... 5200/9704
2020-02-25T15:19:27.5178098Z .................................................................................................... 5300/9704
2020-02-25T15:19:33.5262942Z ............ii.ii........i...i...................................................................... 5400/9704
2020-02-25T15:19:42.2002172Z .................................................................................................... 5600/9704
2020-02-25T15:19:53.1992514Z .................................................................................................... 5700/9704
2020-02-25T15:20:00.5152602Z ...i................................................................................................ 5800/9704
2020-02-25T15:20:06.1464748Z .................................................................................................... 5900/9704
2020-02-25T15:20:06.1464748Z .................................................................................................... 5900/9704
2020-02-25T15:20:16.4809133Z ..............................................................................................ii...i 6000/9704
2020-02-25T15:20:28.5693681Z ..ii...........i.................................................................................... 6100/9704
2020-02-25T15:20:45.7685855Z .................................................................................................... 6300/9704
2020-02-25T15:20:52.3553857Z .................................................................................................... 6400/9704
2020-02-25T15:20:52.3553857Z .................................................................................................... 6400/9704
2020-02-25T15:21:09.0223132Z .........................i..ii...................................................................... 6500/9704
2020-02-25T15:21:29.8580186Z .................................................................................................... 6700/9704
2020-02-25T15:21:32.1341828Z .................i.................................................................................. 6800/9704
2020-02-25T15:21:34.3949248Z .................................................................................................... 6900/9704
2020-02-25T15:21:36.7837658Z ...............................................i.................................................... 7000/9704
---
2020-02-25T15:23:19.7657298Z .................................................................................................... 7700/9704
2020-02-25T15:23:25.1607960Z .................................................................................................... 7800/9704
2020-02-25T15:23:32.3187900Z ...........................................................................................i........ 7900/9704
2020-02-25T15:23:41.6264934Z .................................................................................................... 8000/9704
2020-02-25T15:23:49.6066777Z ........................................iiiiiii.i................................................... 8100/9704
2020-02-25T15:24:04.8270081Z .................................................................................................... 8300/9704
2020-02-25T15:24:11.7463757Z .................................................................................................... 8400/9704
2020-02-25T15:24:27.7922454Z .................................................................................................... 8500/9704
2020-02-25T15:24:35.6888287Z .................................................................................................... 8600/9704
---
2020-02-25T15:26:39.8778672Z 
2020-02-25T15:26:39.8782356Z 1 #![warn(anonymous_parameters)]
2020-02-25T15:26:39.8783144Z 2 // Test for the anonymous_parameters deprecation lint (RFC 1685)
2020-02-25T15:26:39.8783846Z 3 
2020-02-25T15:26:39.8784793Z - // build-pass (FIXME(62277): could be check-pass?)
2020-02-25T15:26:39.8788353Z + // check-pass
2020-02-25T15:26:39.8789916Z 5 // edition:2015
2020-02-25T15:26:39.8791703Z 7 
2020-02-25T15:26:39.8799523Z 
2020-02-25T15:26:39.8799949Z 
2020-02-25T15:26:39.8800437Z The actual fixed differed from the expected fixed.
2020-02-25T15:26:39.8800437Z The actual fixed differed from the expected fixed.
2020-02-25T15:26:39.8811441Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-deprecated/anon-params-deprecated.fixed
2020-02-25T15:26:39.8812797Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T15:26:39.8813940Z To only update this specific test, also pass `--test-args anon-params-deprecated.rs`
2020-02-25T15:26:39.8815163Z error: 1 errors occurred comparing output.
2020-02-25T15:26:39.8815685Z status: exit code: 0
2020-02-25T15:26:39.8815685Z status: exit code: 0
2020-02-25T15:26:39.8818269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params-deprecated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-deprecated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-deprecated/auxiliary"
2020-02-25T15:26:39.8830239Z ------------------------------------------
2020-02-25T15:26:39.8830887Z 
2020-02-25T15:26:39.8831795Z ------------------------------------------
2020-02-25T15:26:39.8832409Z stderr:
2020-02-25T15:26:39.8832409Z stderr:
2020-02-25T15:26:39.8833318Z ------------------------------------------
2020-02-25T15:26:39.8835364Z warning: anonymous parameters are deprecated and will be removed in the next edition.
2020-02-25T15:26:39.8836736Z   --> /checkout/src/test/ui/anon-params-deprecated.rs:9:12
2020-02-25T15:26:39.8837146Z    |
2020-02-25T15:26:39.8838478Z LL |     fn foo(i32); //~ WARNING anonymous parameters are deprecated
2020-02-25T15:26:39.8839689Z    |            ^^^ help: try naming the parameter or explicitly ignoring it: `_: i32`
2020-02-25T15:26:39.8840404Z note: the lint level is defined here
2020-02-25T15:26:39.8841171Z   --> /checkout/src/test/ui/anon-params-deprecated.rs:1:9
2020-02-25T15:26:39.8841535Z    |
2020-02-25T15:26:39.8841800Z LL | #![warn(anonymous_parameters)]
2020-02-25T15:26:39.8841800Z LL | #![warn(anonymous_parameters)]
2020-02-25T15:26:39.8842146Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-25T15:26:39.8842770Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2020-02-25T15:26:39.8843981Z    = note: for more information, see issue #41686 <***/issues/41686>
2020-02-25T15:26:39.8844793Z warning: anonymous parameters are deprecated and will be removed in the next edition.
2020-02-25T15:26:39.8845600Z   --> /checkout/src/test/ui/anon-params-deprecated.rs:12:30
2020-02-25T15:26:39.8845963Z    |
2020-02-25T15:26:39.8846270Z LL |     fn bar_with_default_impl(String, String) {}
2020-02-25T15:26:39.8846270Z LL |     fn bar_with_default_impl(String, String) {}
2020-02-25T15:26:39.8846912Z    |                              ^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: String`
2020-02-25T15:26:39.8847440Z    |
2020-02-25T15:26:39.8847968Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2020-02-25T15:26:39.8849077Z    = note: for more information, see issue #41686 <***/issues/41686>
2020-02-25T15:26:39.8849811Z warning: anonymous parameters are deprecated and will be removed in the next edition.
2020-02-25T15:26:39.8850625Z   --> /checkout/src/test/ui/anon-params-deprecated.rs:12:38
2020-02-25T15:26:39.8850984Z    |
2020-02-25T15:26:39.8851289Z LL |     fn bar_with_default_impl(String, String) {}
2020-02-25T15:26:39.8851289Z LL |     fn bar_with_default_impl(String, String) {}
2020-02-25T15:26:39.8851951Z    |                                      ^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: String`
2020-02-25T15:26:39.8852505Z    |
2020-02-25T15:26:39.8853031Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2020-02-25T15:26:39.8854547Z    = note: for more information, see issue #41686 <***/issues/41686>
2020-02-25T15:26:39.8855040Z 
2020-02-25T15:26:39.8855532Z ------------------------------------------
2020-02-25T15:26:39.8855779Z 
2020-02-25T15:26:39.8856083Z 
---
2020-02-25T15:26:39.8859138Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-25T15:26:39.8859836Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-25T15:26:39.8860175Z 
2020-02-25T15:26:39.8860308Z 
2020-02-25T15:26:39.8865415Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-25T15:26:39.8869454Z 
2020-02-25T15:26:39.8869589Z 
2020-02-25T15:26:39.8869987Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-25T15:26:39.8870458Z Build completed unsuccessfully in 1:09:49
2020-02-25T15:26:39.8870458Z Build completed unsuccessfully in 1:09:49
2020-02-25T15:26:39.8902323Z == clock drift check ==
2020-02-25T15:26:39.8926380Z   local time: Tue Feb 25 15:26:39 UTC 2020
2020-02-25T15:26:40.4419048Z   network time: Tue, 25 Feb 2020 15:26:40 GMT
2020-02-25T15:26:40.4423335Z == end clock drift check ==
2020-02-25T15:26:41.0129719Z 
2020-02-25T15:26:41.0220348Z ##[error]Bash exited with code '1'.
2020-02-25T15:26:41.0236819Z ##[section]Finishing: Run build
2020-02-25T15:26:41.0292596Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69460/merge to s
2020-02-25T15:26:41.0298518Z Task         : Get sources
2020-02-25T15:26:41.0298911Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T15:26:41.0299298Z Version      : 1.0.0
2020-02-25T15:26:41.0299550Z Author       : Microsoft
2020-02-25T15:26:41.0299550Z Author       : Microsoft
2020-02-25T15:26:41.0299953Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T15:26:41.0300456Z ==============================================================================
2020-02-25T15:26:41.4201204Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T15:26:41.4253623Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69460/merge to s
2020-02-25T15:26:41.4403788Z Cleaning up task key
2020-02-25T15:26:41.4405447Z Start cleaning up orphan processes.
2020-02-25T15:26:41.4620578Z Terminate orphan process: pid (4373) (python)
2020-02-25T15:26:41.4896011Z ##[section]Finishing: Finalize Job
