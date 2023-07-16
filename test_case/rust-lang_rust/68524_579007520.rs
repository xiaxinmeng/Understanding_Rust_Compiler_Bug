plain
2020-01-27T22:27:47.7960471Z ========================== Starting Command Output ===========================
2020-01-27T22:27:47.7962291Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0ebd008a-a3f3-48bf-a328-b2323127bb58.sh
2020-01-27T22:27:47.7962328Z 
2020-01-27T22:27:47.7964931Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T22:27:47.7971495Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-27T22:27:47.7973201Z Task         : Get sources
2020-01-27T22:27:47.7973239Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T22:27:47.7973333Z Version      : 1.0.0
2020-01-27T22:27:47.7973370Z Author       : Microsoft
---
2020-01-27T22:27:48.7792518Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T22:27:48.7806263Z ##[command]git config gc.auto 0
2020-01-27T22:27:48.7811544Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T22:27:48.7814142Z ##[command]git config --get-all http.proxy
2020-01-27T22:27:48.7823439Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-27T23:22:44.2968268Z .................................................................................................... 1700/9561
2020-01-27T23:22:49.6755315Z .................................................................................................... 1800/9561
2020-01-27T23:23:02.5915660Z ........................i........................................................................... 1900/9561
2020-01-27T23:23:09.5077774Z .................................................................................................... 2000/9561
2020-01-27T23:23:24.6604483Z ..............iiiii................................................................................. 2100/9561
2020-01-27T23:23:34.9105828Z .................................................................................................... 2300/9561
2020-01-27T23:23:37.4473244Z .................................................................................................... 2400/9561
2020-01-27T23:23:42.9019087Z .................................................................................................... 2500/9561
2020-01-27T23:24:04.3733755Z .................................................................................................... 2600/9561
---
2020-01-27T23:26:52.2714415Z ...............................................................i...............i.................... 4900/9561
2020-01-27T23:27:00.2181483Z .................................................................................................... 5000/9561
2020-01-27T23:27:08.3147321Z .................................................................................................... 5100/9561
2020-01-27T23:27:13.1897024Z ......i............................................................................................. 5200/9561
2020-01-27T23:27:24.6111639Z ...............................................................................ii.ii........i...i... 5300/9561
2020-01-27T23:27:33.3721499Z .................i.................................................................................. 5500/9561
2020-01-27T23:27:42.8435659Z .................................................................................................... 5600/9561
2020-01-27T23:27:49.9254930Z ..................................................................i................................. 5700/9561
2020-01-27T23:27:57.3150397Z .................................................................................................... 5800/9561
2020-01-27T23:27:57.3150397Z .................................................................................................... 5800/9561
2020-01-27T23:28:04.7174285Z .................................................................................................... 5900/9561
2020-01-27T23:28:14.2071732Z .........................................................ii...i..ii...........i..................... 6000/9561
2020-01-27T23:28:36.3128084Z .................................................................................................... 6200/9561
2020-01-27T23:28:44.1758236Z .................................................................................................... 6300/9561
2020-01-27T23:28:44.1758236Z .................................................................................................... 6300/9561
2020-01-27T23:28:52.5602261Z .....................................................................................i..ii.......... 6400/9561
2020-01-27T23:29:17.7834689Z .................................................................................................... 6600/9561
2020-01-27T23:29:26.7144436Z .............................................................i...................................... 6700/9561
2020-01-27T23:29:28.9147919Z .................................................................................................... 6800/9561
2020-01-27T23:29:31.2444311Z ............................................................i....................................... 6900/9561
---
2020-01-27T23:31:13.9413816Z .................................................................................................... 7600/9561
2020-01-27T23:31:19.2722142Z .................................................................................................... 7700/9561
2020-01-27T23:31:26.0978145Z .................................................................................................... 7800/9561
2020-01-27T23:31:36.6991095Z .................................................................................................... 7900/9561
2020-01-27T23:31:42.9912796Z ................iiiiiii............................................................................. 8000/9561
2020-01-27T23:31:57.8923213Z .................................................................................................... 8200/9561
2020-01-27T23:32:08.0565025Z .................................................................................................... 8300/9561
2020-01-27T23:32:21.6970998Z .................................................................................................... 8400/9561
2020-01-27T23:32:28.7183416Z .................................................................................................... 8500/9561
---
2020-01-27T23:34:28.4809752Z 12 
2020-01-27T23:34:28.4809964Z 
2020-01-27T23:34:28.4810204Z 
2020-01-27T23:34:28.4810449Z The actual stderr differed from the expected stderr.
2020-01-27T23:34:28.4811078Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref/retain-resume-ref.stderr
2020-01-27T23:34:28.4811714Z To update references, rerun the tests and pass the `--bless` flag
2020-01-27T23:34:28.4812419Z To only update this specific test, also pass `--test-args generator/retain-resume-ref.rs`
2020-01-27T23:34:28.4814487Z error: 1 errors occurred comparing output.
2020-01-27T23:34:28.4818676Z status: exit code: 1
2020-01-27T23:34:28.4818676Z status: exit code: 1
2020-01-27T23:34:28.4820395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/retain-resume-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref/auxiliary" "-A" "unused"
2020-01-27T23:34:28.4821662Z ------------------------------------------
2020-01-27T23:34:28.4821953Z 
2020-01-27T23:34:28.4823471Z ------------------------------------------
2020-01-27T23:34:28.4824626Z stderr:
2020-01-27T23:34:28.4824626Z stderr:
2020-01-27T23:34:28.4827163Z ------------------------------------------
2020-01-27T23:34:28.4827477Z error[E0499]: cannot borrow `thing` as mutable more than once at a time
2020-01-27T23:34:28.4828238Z   --> /checkout/src/test/ui/generator/retain-resume-ref.rs:23:25
2020-01-27T23:34:28.4831901Z LL |     gen.as_mut().resume(&mut thing);
2020-01-27T23:34:28.4833822Z    |                         ---------- first mutable borrow occurs here
2020-01-27T23:34:28.4834175Z LL |     gen.as_mut().resume(&mut thing);
2020-01-27T23:34:28.4834355Z    |                         ^^^^^^^^^^ second mutable borrow occurs here
2020-01-27T23:34:28.4834355Z    |                         ^^^^^^^^^^ second mutable borrow occurs here
2020-01-27T23:34:28.4834526Z LL |     //~^ cannot borrow `thing` as mutable more than once at a time
2020-01-27T23:34:28.4835281Z    | - first borrow might be used here, when `gen` is dropped and runs the destructor for generator
2020-01-27T23:34:28.4835475Z 
2020-01-27T23:34:28.4835661Z error: aborting due to previous error
2020-01-27T23:34:28.4835803Z 
---
2020-01-27T23:34:28.4839409Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-27T23:34:28.4839648Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-27T23:34:28.4839804Z 
2020-01-27T23:34:28.4839942Z 
2020-01-27T23:34:28.4842690Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-27T23:34:28.4844577Z 
2020-01-27T23:34:28.4844888Z 
2020-01-27T23:34:28.4845130Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-27T23:34:28.4845333Z Build completed unsuccessfully in 1:01:19
2020-01-27T23:34:28.4845333Z Build completed unsuccessfully in 1:01:19
2020-01-27T23:34:28.4904714Z == clock drift check ==
2020-01-27T23:34:28.4924613Z   local time: Mon Jan 27 23:34:28 UTC 2020
2020-01-27T23:34:28.7659412Z   network time: Mon, 27 Jan 2020 23:34:28 GMT
2020-01-27T23:34:28.7660369Z == end clock drift check ==
2020-01-27T23:34:29.1490611Z 
2020-01-27T23:34:29.1606135Z ##[error]Bash exited with code '1'.
2020-01-27T23:34:29.1629798Z ##[section]Finishing: Run build
2020-01-27T23:34:29.1650704Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-27T23:34:29.1652785Z Task         : Get sources
2020-01-27T23:34:29.1652840Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T23:34:29.1652892Z Version      : 1.0.0
2020-01-27T23:34:29.1652959Z Author       : Microsoft
2020-01-27T23:34:29.1652959Z Author       : Microsoft
2020-01-27T23:34:29.1653013Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T23:34:29.1653071Z ==============================================================================
2020-01-27T23:34:29.6153782Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T23:34:29.6196261Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-27T23:34:29.6312889Z Cleaning up task key
2020-01-27T23:34:29.6313770Z Start cleaning up orphan processes.
2020-01-27T23:34:29.6427922Z Terminate orphan process: pid (9960) (python)
2020-01-27T23:34:29.6697195Z ##[section]Finishing: Finalize Job
