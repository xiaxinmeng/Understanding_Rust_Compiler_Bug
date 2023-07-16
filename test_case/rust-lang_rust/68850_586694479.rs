plain
2020-02-16T10:01:50.8263203Z ========================== Starting Command Output ===========================
2020-02-16T10:01:50.8267640Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/663b4850-085d-49dc-afa8-be015fc60955.sh
2020-02-16T10:01:50.8267849Z 
2020-02-16T10:01:50.8271925Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T10:01:50.8278519Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-16T10:01:50.8280229Z Task         : Get sources
2020-02-16T10:01:50.8280319Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T10:01:50.8280357Z Version      : 1.0.0
2020-02-16T10:01:50.8280393Z Author       : Microsoft
---
2020-02-16T10:01:54.1504711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T10:01:54.1518455Z ##[command]git config gc.auto 0
2020-02-16T10:01:54.1523510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T10:01:54.1527835Z ##[command]git config --get-all http.proxy
2020-02-16T10:01:54.1535615Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68850/merge:refs/remotes/pull/68850/merge
---
2020-02-16T11:05:26.3781929Z .................................................................................................... 1700/9651
2020-02-16T11:05:31.5089114Z .................................................................................................... 1800/9651
2020-02-16T11:05:44.0227662Z ..................................i................................................................. 1900/9651
2020-02-16T11:05:52.2061220Z .................................................................................................... 2000/9651
2020-02-16T11:06:07.5916751Z ........................iiiii....................................................................... 2100/9651
2020-02-16T11:06:18.0047460Z .................................................................................................... 2300/9651
2020-02-16T11:06:20.5592449Z .................................................................................................... 2400/9651
2020-02-16T11:06:25.4246791Z .................................................................................................... 2500/9651
2020-02-16T11:06:47.9338459Z .................................................................................................... 2600/9651
---
2020-02-16T11:10:25.2271081Z .................................................................................................... 5600/9651
2020-02-16T11:10:36.4325492Z .............................F.........................................................i............ 5700/9651
2020-02-16T11:10:44.7787131Z .................................................................................................... 5800/9651
2020-02-16T11:10:50.2524957Z .....................................................................................i.............. 5900/9651
2020-02-16T11:11:00.7197096Z ...............................................................................ii...i..ii........... 6000/9651
2020-02-16T11:11:13.6707400Z i................................................................................................... 6100/9651
2020-02-16T11:11:31.0887934Z .................................................................................................... 6300/9651
2020-02-16T11:11:39.2679692Z .................................................................................................... 6400/9651
2020-02-16T11:11:39.2679692Z .................................................................................................... 6400/9651
2020-02-16T11:11:57.1938940Z .......i..ii........................................................................................ 6500/9651
2020-02-16T11:12:18.5366371Z ...............................................................................................i.... 6700/9651
2020-02-16T11:12:20.8901479Z .................................................................................................... 6800/9651
2020-02-16T11:12:23.2377401Z .................................................................................................... 6900/9651
2020-02-16T11:12:25.8663584Z .....i.............................................................................................. 7000/9651
---
2020-02-16T11:14:08.3416177Z .................................................................................................... 7600/9651
2020-02-16T11:14:13.3037434Z .................................................................................................... 7700/9651
2020-02-16T11:14:19.7888479Z .................................................................................................... 7800/9651
2020-02-16T11:14:26.9722243Z .................................................................................................... 7900/9651
2020-02-16T11:14:37.2219563Z ........................................................................................iiiiiii.i... 8000/9651
2020-02-16T11:14:54.3661395Z ............................i......i................................................................ 8200/9651
2020-02-16T11:14:59.7150657Z .................................................................................................... 8300/9651
2020-02-16T11:15:11.7797793Z .................................................................................................... 8400/9651
2020-02-16T11:15:24.1921196Z .................................................................................................... 8500/9651
---
2020-02-16T11:17:31.3931179Z 
2020-02-16T11:17:31.3931407Z 
2020-02-16T11:17:31.3931724Z The actual stderr differed from the expected stderr.
2020-02-16T11:17:31.3932478Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-prelude-no-accidents/builtin-prelude-no-accidents.stderr
2020-02-16T11:17:31.3933132Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T11:17:31.3934220Z To only update this specific test, also pass `--test-args macros/builtin-prelude-no-accidents.rs`
2020-02-16T11:17:31.3934848Z error: 1 errors occurred comparing output.
2020-02-16T11:17:31.3935510Z status: exit code: 1
2020-02-16T11:17:31.3935510Z status: exit code: 1
2020-02-16T11:17:31.3936791Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/builtin-prelude-no-accidents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-prelude-no-accidents" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-prelude-no-accidents/auxiliary" "-A" "unused"
2020-02-16T11:17:31.3937956Z ------------------------------------------
2020-02-16T11:17:31.3938330Z 
2020-02-16T11:17:31.3938916Z ------------------------------------------
2020-02-16T11:17:31.3939247Z stderr:
2020-02-16T11:17:31.3939247Z stderr:
2020-02-16T11:17:31.3939834Z ------------------------------------------
2020-02-16T11:17:31.3940235Z error[E0433]: failed to resolve: use of undeclared type or module `env`
2020-02-16T11:17:31.3940819Z   --> /checkout/src/test/ui/macros/builtin-prelude-no-accidents.rs:5:5
2020-02-16T11:17:31.3941261Z    |
2020-02-16T11:17:31.3941529Z LL |     env::current_dir; //~ ERROR use of undeclared type or module `env`
2020-02-16T11:17:31.3941811Z    |     ^^^ use of undeclared type or module `env`
2020-02-16T11:17:31.3942309Z error[E0433]: failed to resolve: use of undeclared type or module `panic`
2020-02-16T11:17:31.3943551Z   --> /checkout/src/test/ui/macros/builtin-prelude-no-accidents.rs:6:14
2020-02-16T11:17:31.3943849Z    |
2020-02-16T11:17:31.3944048Z LL |     type A = panic::PanicInfo; //~ ERROR use of undeclared type or module `panic`
2020-02-16T11:17:31.3944048Z LL |     type A = panic::PanicInfo; //~ ERROR use of undeclared type or module `panic`
2020-02-16T11:17:31.3944253Z    |              ^^^^^ use of undeclared type or module `panic`
2020-02-16T11:17:31.3944397Z 
2020-02-16T11:17:31.3944566Z error[E0433]: failed to resolve: use of undeclared type or module `vec`
2020-02-16T11:17:31.3945128Z   --> /checkout/src/test/ui/macros/builtin-prelude-no-accidents.rs:7:14
2020-02-16T11:17:31.3945346Z    |
2020-02-16T11:17:31.3945540Z LL |     type B = vec::Vec<u8>; //~ ERROR use of undeclared type or module `vec`
2020-02-16T11:17:31.3945868Z    |              |
2020-02-16T11:17:31.3946047Z    |              use of undeclared type or module `vec`
2020-02-16T11:17:31.3946239Z    |              help: a struct with a similar name exists (notice the capitalization): `Vec`
2020-02-16T11:17:31.3946385Z 
---
2020-02-16T11:17:31.3948907Z diff of stderr:
2020-02-16T11:17:31.3949050Z 
2020-02-16T11:17:31.3949489Z 2   --> $DIR/type-ascription-instead-of-path.rs:2:9
2020-02-16T11:17:31.3949696Z 3    |
2020-02-16T11:17:31.3949860Z 4 LL |     std:io::stdin();
2020-02-16T11:17:31.3950300Z -    |         ^^ use of undeclared type or module `io`
2020-02-16T11:17:31.3950669Z +    |         |
2020-02-16T11:17:31.3950850Z +    |         use of undeclared type or module `io`
2020-02-16T11:17:31.3951020Z +    |         help: a builtin type with a similar name exists: `i8`
2020-02-16T11:17:31.3951240Z 6 
2020-02-16T11:17:31.3951240Z 6 
2020-02-16T11:17:31.3951464Z 7 error[E0423]: expected value, found crate `std`
2020-02-16T11:17:31.3951899Z 8   --> $DIR/type-ascription-instead-of-path.rs:2:5
2020-02-16T11:17:31.3952137Z 
2020-02-16T11:17:31.3952296Z 
2020-02-16T11:17:31.3952463Z The actual stderr differed from the expected stderr.
2020-02-16T11:17:31.3954188Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path/type-ascription-instead-of-path.stderr
2020-02-16T11:17:31.3954788Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T11:17:31.3955324Z To only update this specific test, also pass `--test-args suggestions/type-ascription-instead-of-path.rs`
2020-02-16T11:17:31.3955716Z error: 1 errors occurred comparing output.
2020-02-16T11:17:31.3955880Z status: exit code: 1
2020-02-16T11:17:31.3955880Z status: exit code: 1
2020-02-16T11:17:31.3957326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path/auxiliary" "-A" "unused"
2020-02-16T11:17:31.3958146Z ------------------------------------------
2020-02-16T11:17:31.3958342Z 
2020-02-16T11:17:31.3958758Z ------------------------------------------
2020-02-16T11:17:31.3958984Z stderr:
2020-02-16T11:17:31.3958984Z stderr:
2020-02-16T11:17:31.3959395Z ------------------------------------------
2020-02-16T11:17:31.3962113Z error[E0433]: failed to resolve: use of undeclared type or module `io`
2020-02-16T11:17:31.3962814Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs:2:9
2020-02-16T11:17:31.3963057Z    |
2020-02-16T11:17:31.3963329Z LL |     std:io::stdin();
2020-02-16T11:17:31.3963655Z    |         |
2020-02-16T11:17:31.3963837Z    |         use of undeclared type or module `io`
2020-02-16T11:17:31.3964018Z    |         help: a builtin type with a similar name exists: `i8`
2020-02-16T11:17:31.3964803Z 
2020-02-16T11:17:31.3964803Z 
2020-02-16T11:17:31.3965013Z error[E0423]: expected value, found crate `std`
2020-02-16T11:17:31.3965542Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs:2:5
2020-02-16T11:17:31.3965764Z    |
2020-02-16T11:17:31.3965950Z LL |     std:io::stdin();
2020-02-16T11:17:31.3966401Z    |     ^^^- help: maybe you meant to write a path separator here: `::`
2020-02-16T11:17:31.3966806Z    |     not a value
2020-02-16T11:17:31.3966946Z 
2020-02-16T11:17:31.3967111Z error: aborting due to 2 previous errors
2020-02-16T11:17:31.3967271Z 
---
2020-02-16T11:17:31.3971569Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T11:17:31.3971806Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T11:17:31.3983205Z 
2020-02-16T11:17:31.3983795Z 
2020-02-16T11:17:31.3986978Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T11:17:31.3987418Z 
2020-02-16T11:17:31.3987472Z 
2020-02-16T11:17:31.3993599Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T11:17:31.3993888Z Build completed unsuccessfully in 1:08:43
2020-02-16T11:17:31.3993888Z Build completed unsuccessfully in 1:08:43
2020-02-16T11:17:31.4051636Z == clock drift check ==
2020-02-16T11:17:31.4075228Z   local time: Sun Feb 16 11:17:31 UTC 2020
2020-02-16T11:17:31.5638000Z   network time: Sun, 16 Feb 2020 11:17:31 GMT
2020-02-16T11:17:31.5642539Z == end clock drift check ==
2020-02-16T11:17:32.1156090Z 
2020-02-16T11:17:32.1257557Z ##[error]Bash exited with code '1'.
2020-02-16T11:17:32.1269573Z ##[section]Finishing: Run build
2020-02-16T11:17:32.1289619Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-16T11:17:32.1291515Z Task         : Get sources
2020-02-16T11:17:32.1291576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T11:17:32.1291759Z Version      : 1.0.0
2020-02-16T11:17:32.1291800Z Author       : Microsoft
2020-02-16T11:17:32.1291800Z Author       : Microsoft
2020-02-16T11:17:32.1291845Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T11:17:32.1291915Z ==============================================================================
2020-02-16T11:17:32.5685062Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T11:17:32.5728242Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-16T11:17:32.5856094Z Cleaning up task key
2020-02-16T11:17:32.5856936Z Start cleaning up orphan processes.
2020-02-16T11:17:32.5977939Z Terminate orphan process: pid (4380) (python)
2020-02-16T11:17:32.6547559Z ##[section]Finishing: Finalize Job
