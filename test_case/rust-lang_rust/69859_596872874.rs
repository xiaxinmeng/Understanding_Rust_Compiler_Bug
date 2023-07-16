plain
2020-03-10T01:31:17.8158311Z ========================== Starting Command Output ===========================
2020-03-10T01:31:17.8161785Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d2a031fb-63b5-442d-bf56-95103db49955.sh
2020-03-10T01:31:17.8162093Z 
2020-03-10T01:31:17.8166429Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T01:31:17.8187843Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-10T01:31:17.8191454Z Task         : Get sources
2020-03-10T01:31:17.8192581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T01:31:17.8192923Z Version      : 1.0.0
2020-03-10T01:31:17.8193164Z Author       : Microsoft
---
2020-03-10T01:31:18.9635772Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T01:31:18.9648168Z ##[command]git config gc.auto 0
2020-03-10T01:31:18.9653692Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T01:31:18.9663191Z ##[command]git config --get-all http.proxy
2020-03-10T01:31:18.9676226Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69859/merge:refs/remotes/pull/69859/merge
---
2020-03-10T02:32:42.6693510Z .................................................................................................... 1700/9755
2020-03-10T02:32:47.2980532Z .................................................................................................... 1800/9755
2020-03-10T02:32:59.1331063Z ............................................................i....................................... 1900/9755
2020-03-10T02:33:06.8205721Z .................................................................................................... 2000/9755
2020-03-10T02:33:21.6053499Z ..................................................iiiii............................................. 2100/9755
2020-03-10T02:33:31.8768713Z .................................................................................................... 2300/9755
2020-03-10T02:33:34.1120900Z .................................................................................................... 2400/9755
2020-03-10T02:33:37.3984821Z .................................................................................................... 2500/9755
2020-03-10T02:33:59.2932723Z .................................................................................................... 2600/9755
---
2020-03-10T02:36:42.0014837Z .....................i...............i.............................................................. 5000/9755
2020-03-10T02:36:52.0933921Z .................................................................................................... 5100/9755
2020-03-10T02:36:57.6940039Z ................................................................i................................... 5200/9755
2020-03-10T02:37:03.9965253Z .................................................F.................................................. 5300/9755
2020-03-10T02:37:13.2330710Z .............................................ii.ii........i...i..................................... 5400/9755
2020-03-10T02:37:21.4615850Z .................................................................................................... 5600/9755
2020-03-10T02:37:31.3128527Z .................................................................................................... 5700/9755
2020-03-10T02:37:38.3445253Z ....................................i............................................................... 5800/9755
2020-03-10T02:37:44.5324787Z .................................................................................................... 5900/9755
2020-03-10T02:37:44.5324787Z .................................................................................................... 5900/9755
2020-03-10T02:37:55.3178122Z .................................................................................................... 6000/9755
2020-03-10T02:38:04.4985656Z .............................ii...i..ii...........i................................................. 6100/9755
2020-03-10T02:38:21.6155639Z .................................................................................................... 6300/9755
2020-03-10T02:38:28.2494638Z .................................................................................................... 6400/9755
2020-03-10T02:38:28.2494638Z .................................................................................................... 6400/9755
2020-03-10T02:38:39.7146614Z ............................................................i..ii................................... 6500/9755
2020-03-10T02:39:11.7936707Z .................................................................................................... 6700/9755
2020-03-10T02:39:15.4754166Z ......................................................i............................................. 6800/9755
2020-03-10T02:39:17.6070723Z .................................................................................................... 6900/9755
2020-03-10T02:39:20.2144393Z .....................................................................................i.............. 7000/9755
---
2020-03-10T02:40:58.8512904Z .................................................................................................... 7700/9755
2020-03-10T02:41:02.9280765Z .................................................................................................... 7800/9755
2020-03-10T02:41:08.7294082Z .................................................................................................... 7900/9755
2020-03-10T02:41:15.5928043Z ...................................i................................................................ 8000/9755
2020-03-10T02:41:24.4577237Z ....................................................................................iiiiiiiiii.i.... 8100/9755
2020-03-10T02:41:40.0657004Z ............................i......i................................................................ 8300/9755
2020-03-10T02:41:44.7552406Z .................................................................................................... 8400/9755
2020-03-10T02:41:55.8527308Z .................................................................................................... 8500/9755
2020-03-10T02:42:07.5533650Z .................................................................................................... 8600/9755
---
2020-03-10T02:44:05.5167711Z 
2020-03-10T02:44:05.5168904Z ---- [ui] ui/issues/issue-69602-type-err-during-codegen-ice.rs stdout ----
2020-03-10T02:44:05.5169377Z diff of stderr:
2020-03-10T02:44:05.5169625Z 
2020-03-10T02:44:05.5169922Z 13 LL | impl TraitB for B {
2020-03-10T02:44:05.5170308Z 14    | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
2020-03-10T02:44:05.5171229Z - error: aborting due to 2 previous errors
2020-03-10T02:44:05.5171914Z + error: array lengths can't depend on generic parameters
2020-03-10T02:44:05.5172663Z +   --> $DIR/issue-69602-type-err-during-codegen-ice.rs:21:17
2020-03-10T02:44:05.5173088Z +    |
2020-03-10T02:44:05.5173088Z +    |
2020-03-10T02:44:05.5173401Z + LL |     let _ = [0; B::VALUE];
2020-03-10T02:44:05.5174071Z + 
2020-03-10T02:44:05.5174378Z + error: aborting due to 3 previous errors
2020-03-10T02:44:05.5174699Z 17 
2020-03-10T02:44:05.5175049Z 18 Some errors have detailed explanations: E0046, E0437.
2020-03-10T02:44:05.5175049Z 18 Some errors have detailed explanations: E0046, E0437.
2020-03-10T02:44:05.5175947Z 19 For more information about an error, try `rustc --explain E0046`.
2020-03-10T02:44:05.5176369Z 
2020-03-10T02:44:05.5176574Z 
2020-03-10T02:44:05.5178241Z The actual stderr differed from the expected stderr.
2020-03-10T02:44:05.5180729Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice/issue-69602-type-err-during-codegen-ice.stderr
2020-03-10T02:44:05.5182987Z To update references, rerun the tests and pass the `--bless` flag
2020-03-10T02:44:05.5199674Z To only update this specific test, also pass `--test-args issues/issue-69602-type-err-during-codegen-ice.rs`
2020-03-10T02:44:05.5221752Z error: 1 errors occurred comparing output.
2020-03-10T02:44:05.5222117Z status: exit code: 1
2020-03-10T02:44:05.5222117Z status: exit code: 1
2020-03-10T02:44:05.5224594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice/auxiliary"
2020-03-10T02:44:05.5226967Z ------------------------------------------
2020-03-10T02:44:05.5227301Z 
2020-03-10T02:44:05.5227818Z ------------------------------------------
2020-03-10T02:44:05.5228188Z stderr:
2020-03-10T02:44:05.5228188Z stderr:
2020-03-10T02:44:05.5228706Z ------------------------------------------
2020-03-10T02:44:05.5229144Z error[E0437]: type `M` is not a member of trait `TraitB`
2020-03-10T02:44:05.5231112Z   --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:17:5
2020-03-10T02:44:05.5231590Z    |
2020-03-10T02:44:05.5231981Z LL |     type M   = A; //~ ERROR type `M` is not a member of trait `TraitB`
2020-03-10T02:44:05.5232429Z    |     ^^^^^^^^^^^^^ not a member of trait `TraitB`
2020-03-10T02:44:05.5233117Z error[E0046]: not all trait items implemented, missing: `MyA`
2020-03-10T02:44:05.5233977Z   --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:16:1
2020-03-10T02:44:05.5235083Z    |
2020-03-10T02:44:05.5235083Z    |
2020-03-10T02:44:05.5235281Z LL |     type MyA: TraitA;
2020-03-10T02:44:05.5235784Z    |     ----------------- `MyA` from trait
2020-03-10T02:44:05.5235983Z ...
2020-03-10T02:44:05.5236280Z LL | impl TraitB for B { //~ ERROR not all trait items implemented, missing: `MyA`
2020-03-10T02:44:05.5236641Z    | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
2020-03-10T02:44:05.5237243Z error: array lengths can't depend on generic parameters
2020-03-10T02:44:05.5237829Z   --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:21:17
2020-03-10T02:44:05.5238117Z    |
2020-03-10T02:44:05.5238117Z    |
2020-03-10T02:44:05.5238321Z LL |     let _ = [0; B::VALUE];
2020-03-10T02:44:05.5238697Z 
2020-03-10T02:44:05.5238888Z error: aborting due to 3 previous errors
2020-03-10T02:44:05.5239074Z 
2020-03-10T02:44:05.5239298Z Some errors have detailed explanations: E0046, E0437.
---
2020-03-10T02:44:05.5245854Z test result: FAILED. 9697 passed; 1 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-10T02:44:05.5246146Z 
2020-03-10T02:44:05.5246239Z 
2020-03-10T02:44:05.5246330Z 
2020-03-10T02:44:05.5250137Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-10T02:44:05.5252853Z 
2020-03-10T02:44:05.5252952Z 
2020-03-10T02:44:05.5253204Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-10T02:44:05.5253534Z Build completed unsuccessfully in 1:06:56
2020-03-10T02:44:05.5253534Z Build completed unsuccessfully in 1:06:56
2020-03-10T02:44:05.5254168Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-10T02:44:05.5254598Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-10T02:44:05.5287738Z == clock drift check ==
2020-03-10T02:44:05.5303619Z   local time: Tue Mar 10 02:44:05 UTC 2020
2020-03-10T02:44:05.8140553Z   network time: Tue, 10 Mar 2020 02:44:05 GMT
2020-03-10T02:44:05.8143292Z == end clock drift check ==
2020-03-10T02:44:06.2393404Z 
2020-03-10T02:44:06.2471518Z ##[error]Bash exited with code '1'.
2020-03-10T02:44:06.2485618Z ##[section]Finishing: Run build
2020-03-10T02:44:06.2545937Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-10T02:44:06.2550724Z Task         : Get sources
2020-03-10T02:44:06.2551092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T02:44:06.2551406Z Version      : 1.0.0
2020-03-10T02:44:06.2551624Z Author       : Microsoft
2020-03-10T02:44:06.2551624Z Author       : Microsoft
2020-03-10T02:44:06.2551986Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T02:44:06.2552392Z ==============================================================================
2020-03-10T02:44:06.5891657Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T02:44:06.5940700Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-10T02:44:06.6028643Z Cleaning up task key
2020-03-10T02:44:06.6029878Z Start cleaning up orphan processes.
2020-03-10T02:44:06.6212976Z Terminate orphan process: pid (3779) (python)
2020-03-10T02:44:06.6423906Z ##[section]Finishing: Finalize Job
