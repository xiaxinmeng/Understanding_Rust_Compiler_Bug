plain
2020-01-30T22:43:45.7167102Z ========================== Starting Command Output ===========================
2020-01-30T22:43:45.7443956Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/55e68f40-843c-4fbe-8e1e-b2044e42419f.sh
2020-01-30T22:43:45.7443996Z 
2020-01-30T22:43:45.7446226Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-30T22:43:45.7452164Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68681/merge to s
2020-01-30T22:43:45.7453875Z Task         : Get sources
2020-01-30T22:43:45.7453951Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T22:43:45.7453986Z Version      : 1.0.0
2020-01-30T22:43:45.7454021Z Author       : Microsoft
---
2020-01-30T22:43:46.5306557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-30T22:43:46.5386200Z ##[command]git config gc.auto 0
2020-01-30T22:43:46.5462846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-30T22:43:46.5517360Z ##[command]git config --get-all http.proxy
2020-01-30T22:43:46.5655401Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68681/merge:refs/remotes/pull/68681/merge
---
2020-01-30T23:31:38.4633258Z .................................................................................................... 1700/9559
2020-01-30T23:31:42.1319091Z .................................................................................................... 1800/9559
2020-01-30T23:31:51.4838445Z .........................i.......................................................................... 1900/9559
2020-01-30T23:31:56.5972076Z .................................................................................................... 2000/9559
2020-01-30T23:32:07.0559238Z ...............iiiii................................................................................ 2100/9559
2020-01-30T23:32:14.3044887Z .................................................................................................... 2300/9559
2020-01-30T23:32:16.0635559Z .................................................................................................... 2400/9559
2020-01-30T23:32:19.8502239Z .................................................................................................... 2500/9559
2020-01-30T23:32:34.7272180Z .................................................................................................... 2600/9559
---
2020-01-30T23:34:31.6519047Z .................................................................................................... 4800/9559
2020-01-30T23:34:35.5949954Z ..........................................................i...............i......................... 4900/9559
2020-01-30T23:34:41.5932214Z .................................................................................................... 5000/9559
2020-01-30T23:34:47.5037471Z .................................................................................................... 5100/9559
2020-01-30T23:34:51.2177028Z .i.................................................................................................. 5200/9559
2020-01-30T23:34:59.2622794Z ..........................................................................ii.ii........i...i........ 5300/9559
2020-01-30T23:35:05.7151140Z ............i....................................................................................... 5500/9559
2020-01-30T23:35:13.0516006Z .................................................................................................... 5600/9559
2020-01-30T23:35:17.8310964Z .............................................................i...................................... 5700/9559
2020-01-30T23:35:23.2811584Z .................................................................................................... 5800/9559
2020-01-30T23:35:23.2811584Z .................................................................................................... 5800/9559
2020-01-30T23:35:29.3312818Z .................................................................................................... 5900/9559
2020-01-30T23:35:35.7318318Z ....................................................ii...i..ii...........i.......................... 6000/9559
2020-01-30T23:35:50.4429400Z .................................................................................................... 6200/9559
2020-01-30T23:35:53.3752506Z .................................................................................................... 6300/9559
2020-01-30T23:35:56.6697447Z .................................................................................i.ii............... 6400/9559
2020-01-30T23:36:06.7096160Z .................................................................................................... 6500/9559
---
2020-01-30T23:37:37.3500860Z .................................................................................................... 7600/9559
2020-01-30T23:37:41.2800887Z .................................................................................................... 7700/9559
2020-01-30T23:37:46.3094238Z .................................................................................................... 7800/9559
2020-01-30T23:37:54.2224791Z .................................................................................................... 7900/9559
2020-01-30T23:37:59.0054284Z ............iiiiiii.i............................................................................... 8000/9559
2020-01-30T23:38:09.5881185Z .................................................................................................... 8200/9559
2020-01-30T23:38:17.4410432Z .................................................................................................... 8300/9559
2020-01-30T23:38:27.5059637Z .................................................................................................... 8400/9559
2020-01-30T23:38:32.3759584Z .................................................................................................... 8500/9559
---
2020-01-30T23:39:56.6393643Z 
2020-01-30T23:39:56.6394211Z ---- [ui] ui/suggestions/type-ascription-instead-of-path-2.rs stdout ----
2020-01-30T23:39:56.6394337Z diff of stderr:
2020-01-30T23:39:56.6394450Z 
2020-01-30T23:39:56.6394695Z - error: expected `::`, found `(`
2020-01-30T23:39:56.6394804Z + error: expected one of `::` or `:`, found `(`
2020-01-30T23:39:56.6395173Z 3    |
2020-01-30T23:39:56.6395173Z 3    |
2020-01-30T23:39:56.6395271Z 4 LL |     vec![Ok(2)].into_iter().collect:<Result<Vec<_>,_>>()?;
2020-01-30T23:39:56.6395638Z -    |                                    -                  ^ expected `::`
2020-01-30T23:39:56.6395931Z +    |                                    -                  ^ expected one of `::` or `:`
2020-01-30T23:39:56.6396061Z 6    |                                    |
2020-01-30T23:39:56.6396164Z 7    |                                    help: maybe write a path separator here: `::`
2020-01-30T23:39:56.6396164Z 7    |                                    help: maybe write a path separator here: `::`
2020-01-30T23:39:56.6396283Z 8    |
2020-01-30T23:39:56.6396363Z 
2020-01-30T23:39:56.6396445Z 
2020-01-30T23:39:56.6396649Z The actual stderr differed from the expected stderr.
2020-01-30T23:39:56.6397011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2/type-ascription-instead-of-path-2.stderr
2020-01-30T23:39:56.6397274Z To update references, rerun the tests and pass the `--bless` flag
2020-01-30T23:39:56.6397574Z To only update this specific test, also pass `--test-args suggestions/type-ascription-instead-of-path-2.rs`
2020-01-30T23:39:56.6397924Z error: 1 errors occurred comparing output.
2020-01-30T23:39:56.6398035Z status: exit code: 1
2020-01-30T23:39:56.6398035Z status: exit code: 1
2020-01-30T23:39:56.6398890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-path-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2/auxiliary" "-A" "unused"
2020-01-30T23:39:56.6399436Z ------------------------------------------
2020-01-30T23:39:56.6399543Z 
2020-01-30T23:39:56.6399833Z ------------------------------------------
2020-01-30T23:39:56.6399952Z stderr:
2020-01-30T23:39:56.6399952Z stderr:
2020-01-30T23:39:56.6400229Z ------------------------------------------
2020-01-30T23:39:56.6400354Z error: expected one of `::` or `:`, found `(`
2020-01-30T23:39:56.6400799Z    |
2020-01-30T23:39:56.6400799Z    |
2020-01-30T23:39:56.6400914Z LL |     vec![Ok(2)].into_iter().collect:<Result<Vec<_>,_>>()?;
2020-01-30T23:39:56.6401312Z    |                                    -                  ^ expected one of `::` or `:`
2020-01-30T23:39:56.6402017Z    |                                    help: maybe write a path separator here: `::`
2020-01-30T23:39:56.6402148Z    |
2020-01-30T23:39:56.6402148Z    |
2020-01-30T23:39:56.6402264Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-01-30T23:39:56.6402809Z    = note: for more information, see ***/issues/23416
2020-01-30T23:39:56.6403060Z error: aborting due to previous error
2020-01-30T23:39:56.6403155Z 
2020-01-30T23:39:56.6403263Z 
2020-01-30T23:39:56.6403533Z ------------------------------------------
---
2020-01-30T23:39:56.6408513Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-01-30T23:39:56.6408716Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-30T23:39:56.6418098Z 
2020-01-30T23:39:56.6418710Z 
2020-01-30T23:39:56.6422200Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-30T23:39:56.6423028Z 
2020-01-30T23:39:56.6423061Z 
2020-01-30T23:39:56.6434649Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-30T23:39:56.6435355Z Build completed unsuccessfully in 0:51:09
2020-01-30T23:39:56.6435355Z Build completed unsuccessfully in 0:51:09
2020-01-30T23:39:56.6486359Z == clock drift check ==
2020-01-30T23:39:56.6503832Z   local time: Thu Jan 30 23:39:56 UTC 2020
2020-01-30T23:39:57.3229885Z   network time: Thu, 30 Jan 2020 23:39:56 GMT
2020-01-30T23:39:57.3230018Z == end clock drift check ==
2020-01-30T23:39:57.3746772Z 
2020-01-30T23:39:57.3812683Z ##[error]Bash exited with code '1'.
2020-01-30T23:39:57.3821789Z ##[section]Finishing: Run build
2020-01-30T23:39:57.3843546Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68681/merge to s
2020-01-30T23:39:57.3845180Z Task         : Get sources
2020-01-30T23:39:57.3845220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T23:39:57.3845274Z Version      : 1.0.0
2020-01-30T23:39:57.3845314Z Author       : Microsoft
2020-01-30T23:39:57.3845314Z Author       : Microsoft
2020-01-30T23:39:57.3845364Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-30T23:39:57.3845407Z ==============================================================================
2020-01-30T23:39:57.7093366Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-30T23:39:57.7128617Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68681/merge to s
2020-01-30T23:39:57.7247033Z Cleaning up task key
2020-01-30T23:39:57.7247622Z Start cleaning up orphan processes.
2020-01-30T23:39:57.7331566Z Terminate orphan process: pid (3525) (python)
2020-01-30T23:39:57.7509183Z ##[section]Finishing: Finalize Job
