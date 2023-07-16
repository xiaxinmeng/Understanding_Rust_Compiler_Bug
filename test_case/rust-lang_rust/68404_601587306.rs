plain
2020-03-20T07:32:47.5670192Z ========================== Starting Command Output ===========================
2020-03-20T07:32:47.5674340Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68128b7c-28ad-422f-ae11-ea856eb433aa.sh
2020-03-20T07:32:47.5674731Z 
2020-03-20T07:32:47.5679466Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T07:32:47.5699063Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T07:32:47.5702428Z Task         : Get sources
2020-03-20T07:32:47.5702712Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T07:32:47.5702991Z Version      : 1.0.0
2020-03-20T07:32:47.5703197Z Author       : Microsoft
---
2020-03-20T07:32:48.8817704Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T07:32:48.8823523Z ##[command]git config gc.auto 0
2020-03-20T07:32:48.8827271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T07:32:48.8830448Z ##[command]git config --get-all http.proxy
2020-03-20T07:32:48.8836850Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
---
2020-03-20T08:25:28.0091142Z .................................................................................................... 1700/9803
2020-03-20T08:25:32.0636538Z .................................................................................................... 1800/9803
2020-03-20T08:25:42.4903019Z ..........................................................................i......................... 1900/9803
2020-03-20T08:25:48.3898364Z .................................................................................................... 2000/9803
2020-03-20T08:25:55.8502879Z ................................................................iiiii............................... 2100/9803
2020-03-20T08:26:12.3067707Z .................................................................................................... 2300/9803
2020-03-20T08:26:14.4234952Z .................................................................................................... 2400/9803
2020-03-20T08:26:17.1948746Z .................................................................................................... 2500/9803
2020-03-20T08:26:36.2042454Z .................................................................................................... 2600/9803
---
2020-03-20T08:29:05.7328467Z .....................................i...............i.............................................. 5000/9803
2020-03-20T08:29:14.3142833Z .................................................................................................... 5100/9803
2020-03-20T08:29:20.4737932Z ................................................................................i................... 5200/9803
2020-03-20T08:29:25.7903333Z .................................................................................................... 5300/9803
2020-03-20T08:29:35.3360234Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-20T08:29:42.9722205Z i................................................................................................... 5600/9803
2020-03-20T08:29:51.9369055Z .....i.............................................................................................. 5700/9803
2020-03-20T08:29:57.9782687Z ........................................................i........................................... 5800/9803
2020-03-20T08:30:04.2093101Z .................................................................................................... 5900/9803
2020-03-20T08:30:04.2093101Z .................................................................................................... 5900/9803
2020-03-20T08:30:11.9198110Z .................................................................................................... 6000/9803
2020-03-20T08:30:19.5125721Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-20T08:30:39.3427441Z .................................................................................................... 6300/9803
2020-03-20T08:30:43.9494758Z .................................................................................................... 6400/9803
2020-03-20T08:30:43.9494758Z .................................................................................................... 6400/9803
2020-03-20T08:30:47.8166102Z ................................................................................i..ii............... 6500/9803
2020-03-20T08:31:09.4836444Z .................................................................................................... 6700/9803
2020-03-20T08:31:18.6085469Z ...............................................................................i.................... 6800/9803
2020-03-20T08:31:20.6545827Z .................................................................................................... 6900/9803
2020-03-20T08:31:22.7139852Z .................................................................................................... 7000/9803
---
2020-03-20T08:33:01.7395793Z .................................................................................................... 7800/9803
2020-03-20T08:33:07.0242081Z .................................................................................................... 7900/9803
2020-03-20T08:33:12.7986621Z ..................................................................i................................. 8000/9803
2020-03-20T08:33:22.5339948Z .................................................................................................... 8100/9803
2020-03-20T08:33:27.7804515Z ...............iiiiiiiiii.i......................................................................... 8200/9803
2020-03-20T08:33:41.0988026Z .................................................................................................... 8400/9803
2020-03-20T08:33:46.9982805Z .................................................................................................... 8500/9803
2020-03-20T08:34:01.3514287Z .................................................................................................... 8600/9803
2020-03-20T08:34:07.6078707Z .................................................................................................... 8700/9803
---
2020-03-20T08:35:56.0876235Z 
2020-03-20T08:35:56.0877647Z ---- [ui] ui/feature-gates/feature-gate-asm.rs stdout ----
2020-03-20T08:35:56.0877934Z diff of stderr:
2020-03-20T08:35:56.0878258Z 
2020-03-20T08:35:56.0878428Z 13 LL |         llvm_asm!("");
2020-03-20T08:35:56.0878825Z 15    |
2020-03-20T08:35:56.0878825Z 15    |
2020-03-20T08:35:56.0879540Z -    = note: see issue #29722 <***/issues/29722> for more information
2020-03-20T08:35:56.0880254Z +    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0880638Z 17    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0881112Z 19 error: aborting due to 2 previous errors
2020-03-20T08:35:56.0881291Z 
2020-03-20T08:35:56.0881389Z 
2020-03-20T08:35:56.0881593Z The actual stderr differed from the expected stderr.
2020-03-20T08:35:56.0881593Z The actual stderr differed from the expected stderr.
2020-03-20T08:35:56.0882301Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/feature-gate-asm.stderr
2020-03-20T08:35:56.0882953Z To update references, rerun the tests and pass the `--bless` flag
2020-03-20T08:35:56.0883552Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm.rs`
2020-03-20T08:35:56.0884019Z error: 1 errors occurred comparing output.
2020-03-20T08:35:56.0884255Z status: exit code: 1
2020-03-20T08:35:56.0884255Z status: exit code: 1
2020-03-20T08:35:56.0886512Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/auxiliary"
2020-03-20T08:35:56.0888144Z ------------------------------------------
2020-03-20T08:35:56.0888317Z 
2020-03-20T08:35:56.0888674Z ------------------------------------------
2020-03-20T08:35:56.0888893Z stderr:
2020-03-20T08:35:56.0888893Z stderr:
2020-03-20T08:35:56.0889258Z ------------------------------------------
2020-03-20T08:35:56.0889924Z error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T08:35:56.0890866Z    |
2020-03-20T08:35:56.0890866Z    |
2020-03-20T08:35:56.0891110Z LL |         asm!(""); //~ ERROR inline assembly is not stable enough
2020-03-20T08:35:56.0891538Z    |
2020-03-20T08:35:56.0891538Z    |
2020-03-20T08:35:56.0892085Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0892467Z    = help: add `#![feature(asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0892689Z 
2020-03-20T08:35:56.0893478Z error[E0658]: use of unstable library feature 'llvm_asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T08:35:56.0894434Z    |
2020-03-20T08:35:56.0894434Z    |
2020-03-20T08:35:56.0894686Z LL |         llvm_asm!(""); //~ ERROR inline assembly is not stable enough
2020-03-20T08:35:56.0895225Z    |
2020-03-20T08:35:56.0895225Z    |
2020-03-20T08:35:56.0895774Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0896163Z    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0896576Z error: aborting due to 2 previous errors
2020-03-20T08:35:56.0896745Z 
2020-03-20T08:35:56.0897189Z For more information about this error, try `rustc --explain E0658`.
2020-03-20T08:35:56.0897403Z 
---
2020-03-20T08:35:56.0898821Z 
2020-03-20T08:35:56.0899053Z 13 LL |         println!("{:?}", llvm_asm!(""));
2020-03-20T08:35:56.0899335Z 14    |                          ^^^^^^^^
2020-03-20T08:35:56.0899532Z 15    |
2020-03-20T08:35:56.0900105Z -    = note: see issue #29722 <***/issues/29722> for more information
2020-03-20T08:35:56.0900775Z +    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0901176Z 17    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0901631Z 19 error: aborting due to 2 previous errors
2020-03-20T08:35:56.0901822Z 
2020-03-20T08:35:56.0901917Z 
2020-03-20T08:35:56.0902121Z The actual stderr differed from the expected stderr.
2020-03-20T08:35:56.0902121Z The actual stderr differed from the expected stderr.
2020-03-20T08:35:56.0902812Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/feature-gate-asm2.stderr
2020-03-20T08:35:56.0903475Z To update references, rerun the tests and pass the `--bless` flag
2020-03-20T08:35:56.0904637Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm2.rs`
2020-03-20T08:35:56.0905112Z error: 1 errors occurred comparing output.
2020-03-20T08:35:56.0905355Z status: exit code: 1
2020-03-20T08:35:56.0905355Z status: exit code: 1
2020-03-20T08:35:56.0907361Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/auxiliary"
2020-03-20T08:35:56.0908983Z ------------------------------------------
2020-03-20T08:35:56.0909176Z 
2020-03-20T08:35:56.0909533Z ------------------------------------------
2020-03-20T08:35:56.0909734Z stderr:
2020-03-20T08:35:56.0909734Z stderr:
2020-03-20T08:35:56.0910116Z ------------------------------------------
2020-03-20T08:35:56.0910786Z error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T08:35:56.0911727Z    |
2020-03-20T08:35:56.0911727Z    |
2020-03-20T08:35:56.0912004Z LL |         println!("{:?}", asm!("")); //~ ERROR inline assembly is not stable
2020-03-20T08:35:56.0912523Z    |
2020-03-20T08:35:56.0912523Z    |
2020-03-20T08:35:56.0913072Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0913452Z    = help: add `#![feature(asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0913752Z 
2020-03-20T08:35:56.0914377Z error[E0658]: use of unstable library feature 'llvm_asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T08:35:56.0915327Z    |
2020-03-20T08:35:56.0915327Z    |
2020-03-20T08:35:56.0915663Z LL |         println!("{:?}", llvm_asm!("")); //~ ERROR inline assembly is not stable
2020-03-20T08:35:56.0916207Z    |
2020-03-20T08:35:56.0916207Z    |
2020-03-20T08:35:56.0916758Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T08:35:56.0917129Z    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
2020-03-20T08:35:56.0917568Z error: aborting due to 2 previous errors
2020-03-20T08:35:56.0917737Z 
2020-03-20T08:35:56.0918179Z For more information about this error, try `rustc --explain E0658`.
2020-03-20T08:35:56.0918393Z 
---
2020-03-20T08:35:56.0920807Z test result: FAILED. 9743 passed; 2 failed; 58 ignored; 0 measured; 0 filtered out
2020-03-20T08:35:56.0921072Z 
2020-03-20T08:35:56.0926871Z 
2020-03-20T08:35:56.0927048Z 
2020-03-20T08:35:56.0931849Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-20T08:35:56.0934561Z 
2020-03-20T08:35:56.0934661Z 
2020-03-20T08:35:56.0935171Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-20T08:35:56.0935557Z Build completed unsuccessfully in 0:58:31
2020-03-20T08:35:56.0935557Z Build completed unsuccessfully in 0:58:31
2020-03-20T08:35:56.0936138Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-20T08:35:56.0936549Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-20T08:35:56.0977989Z == clock drift check ==
2020-03-20T08:35:56.0995647Z   local time: Fri Mar 20 08:35:56 UTC 2020
2020-03-20T08:35:56.3942688Z   network time: Fri, 20 Mar 2020 08:35:56 GMT
2020-03-20T08:35:56.3949063Z == end clock drift check ==
2020-03-20T08:35:56.7949121Z 
2020-03-20T08:35:56.8011667Z ##[error]Bash exited with code '1'.
2020-03-20T08:35:56.8026073Z ##[section]Finishing: Run build
2020-03-20T08:35:56.8077305Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T08:35:56.8082464Z Task         : Get sources
2020-03-20T08:35:56.8082806Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T08:35:56.8083134Z Version      : 1.0.0
2020-03-20T08:35:56.8083362Z Author       : Microsoft
2020-03-20T08:35:56.8083362Z Author       : Microsoft
2020-03-20T08:35:56.8083721Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T08:35:56.8084293Z ==============================================================================
2020-03-20T08:35:57.1519978Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T08:35:57.1559916Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T08:35:57.1660549Z Cleaning up task key
2020-03-20T08:35:57.1661865Z Start cleaning up orphan processes.
2020-03-20T08:35:57.1954927Z Terminate orphan process: pid (3678) (python)
2020-03-20T08:35:57.1992395Z ##[section]Finishing: Finalize Job
