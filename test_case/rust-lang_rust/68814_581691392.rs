plain
2020-02-03T23:49:40.8007554Z ========================== Starting Command Output ===========================
2020-02-03T23:49:40.8010475Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6ca53ed3-2685-45c9-843c-e0490d7c4ff6.sh
2020-02-03T23:49:40.8010583Z 
2020-02-03T23:49:40.8013379Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-03T23:49:40.8019046Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68814/merge to s
2020-02-03T23:49:40.8020682Z Task         : Get sources
2020-02-03T23:49:40.8020713Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T23:49:40.8020742Z Version      : 1.0.0
2020-02-03T23:49:40.8020773Z Author       : Microsoft
---
2020-02-03T23:49:42.5639196Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-03T23:49:42.5652631Z ##[command]git config gc.auto 0
2020-02-03T23:49:42.5658921Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-03T23:49:42.5663998Z ##[command]git config --get-all http.proxy
2020-02-03T23:49:42.5673820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68814/merge:refs/remotes/pull/68814/merge
---
2020-02-04T00:42:53.0410167Z .................................................................................................... 1700/9570
2020-02-04T00:42:57.9844809Z .................................................................................................... 1800/9570
2020-02-04T00:43:10.2843115Z ...........................i........................................................................ 1900/9570
2020-02-04T00:43:17.2024019Z .................................................................................................... 2000/9570
2020-02-04T00:43:31.1600219Z .................iiiii.............................................................................. 2100/9570
2020-02-04T00:43:40.9755568Z .................................................................................................... 2300/9570
2020-02-04T00:43:43.4243700Z .................................................................................................... 2400/9570
2020-02-04T00:43:48.2833764Z .................................................................................................... 2500/9570
2020-02-04T00:44:08.2998357Z .................................................................................................... 2600/9570
---
2020-02-04T00:46:41.1377851Z ............................................................i...............i....................... 4900/9570
2020-02-04T00:46:48.0886880Z .................................................................................................... 5000/9570
2020-02-04T00:46:55.5952197Z .................................................................................................... 5100/9570
2020-02-04T00:47:00.0215209Z ...i................................................................................................ 5200/9570
2020-02-04T00:47:10.3802521Z .............................................................................ii.ii........i...i..... 5300/9570
2020-02-04T00:47:18.6403440Z ...............i.................................................................................... 5500/9570
2020-02-04T00:47:27.8635074Z .................................................................................................... 5600/9570
2020-02-04T00:47:34.1423060Z ................................................................i................................... 5700/9570
2020-02-04T00:47:41.0006263Z .................................................................................................... 5800/9570
2020-02-04T00:47:41.0006263Z .................................................................................................... 5800/9570
2020-02-04T00:47:48.1333420Z .................................................................................................... 5900/9570
2020-02-04T00:47:56.5268035Z .......................................................ii...i..ii...........i....................... 6000/9570
2020-02-04T00:48:16.7734392Z .................................................................................................... 6200/9570
2020-02-04T00:48:23.9472035Z .................................................................................................... 6300/9570
2020-02-04T00:48:23.9472035Z .................................................................................................... 6300/9570
2020-02-04T00:48:31.5553221Z ...................................................................................i..ii............ 6400/9570
2020-02-04T00:48:54.7621439Z .................................................................................................... 6600/9570
2020-02-04T00:49:02.8633191Z ...........................................................i........................................ 6700/9570
2020-02-04T00:49:04.9381000Z .................................................................................................... 6800/9570
2020-02-04T00:49:07.1978004Z .............................................................i...................................... 6900/9570
---
2020-02-04T00:50:43.9614813Z .................................................................................................... 7600/9570
2020-02-04T00:50:48.9378489Z .................................................................................................... 7700/9570
2020-02-04T00:50:55.4238653Z .................................................................................................... 7800/9570
2020-02-04T00:51:05.5594662Z .................................................................................................... 7900/9570
2020-02-04T00:51:11.5219864Z ......................iiiiiii.i..................................................................... 8000/9570
2020-02-04T00:51:25.3581059Z .................................................................................................... 8200/9570
2020-02-04T00:51:33.6019208Z .................................................................................................... 8300/9570
2020-02-04T00:51:47.3339296Z .................................................................................................... 8400/9570
2020-02-04T00:51:54.5320249Z .................................................................................................... 8500/9570
---
2020-02-04T00:53:43.0374233Z 
2020-02-04T00:53:43.0375136Z ---- [ui] ui/ast-json/ast-json-output.rs stdout ----
2020-02-04T00:53:43.0375540Z diff of stdout:
2020-02-04T00:53:43.0375864Z 
2020-02-04T00:53:43.0377266Z - {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]},"NonJoint"]]}}],"inline":true},"attrs":[],"span":{"lo":0,"hi":0}}
2020-02-04T00:53:43.0377790Z + {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]},"NonJoint"]]}}],"inline":true},"attrs":[],"span":{"lo":0,"hi":0},"proc_macros":[]}
2020-02-04T00:53:43.0378386Z 
2020-02-04T00:53:43.0378603Z 
2020-02-04T00:53:43.0378844Z The actual stdout differed from the expected stdout.
2020-02-04T00:53:43.0379667Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
2020-02-04T00:53:43.0379667Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
2020-02-04T00:53:43.0380404Z To update references, rerun the tests and pass the `--bless` flag
2020-02-04T00:53:43.0381262Z To only update this specific test, also pass `--test-args ast-json/ast-json-output.rs`
2020-02-04T00:53:43.0381839Z error: 1 errors occurred comparing output.
2020-02-04T00:53:43.0381839Z error: 1 errors occurred comparing output.
2020-02-04T00:53:43.0382236Z failed to decode compiler output as json: line: {"module":{"inner":{"lo":218,"hi":236},"items":[{"attrs":[],"id":4294967040,"span":{"lo":218,"hi":236},"vis":{"node":"Inherited","span":{"lo":218,"hi":218}},"ident":{"name":"core","span":{"lo":231,"hi":235}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":218,"hi":224}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":225,"hi":230}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":231,"hi":235}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":235,"hi":236}}]},"NonJoint"]]}}],"inline":true},"attrs":[],"span":{"lo":218,"hi":236},"proc_macros":[]}
2020-02-04T00:53:43.0382725Z output: {"module":{"inner":{"lo":218,"hi":236},"items":[{"attrs":[],"id":4294967040,"span":{"lo":218,"hi":236},"vis":{"node":"Inherited","span":{"lo":218,"hi":218}},"ident":{"name":"core","span":{"lo":231,"hi":235}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":218,"hi":224}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":225,"hi":230}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":231,"hi":235}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":235,"hi":236}}]},"NonJoint"]]}}],"inline":true},"attrs":[],"span":{"lo":218,"hi":236},"proc_macros":[]}
2020-02-04T00:53:43.0383770Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T00:53:43.0384003Z 
2020-02-04T00:53:43.0384242Z 
2020-02-04T00:53:43.0384477Z failures:
2020-02-04T00:53:43.0384477Z failures:
2020-02-04T00:53:43.0384984Z     [ui] ui/ast-json/ast-json-output.rs
2020-02-04T00:53:43.0386111Z 
2020-02-04T00:53:43.0389122Z test result: FAILED. 9517 passed; 1 failed; 52 ignored; 0 measured; 0 filtered out
2020-02-04T00:53:43.0389380Z 
2020-02-04T00:53:43.0392748Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T00:53:43.0402221Z 
2020-02-04T00:53:43.0402534Z 
2020-02-04T00:53:43.0404794Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T00:53:43.0405470Z 
2020-02-04T00:53:43.0405631Z 
2020-02-04T00:53:43.0437283Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T00:53:43.0437962Z Build completed unsuccessfully in 0:58:37
2020-02-04T00:53:43.0437962Z Build completed unsuccessfully in 0:58:37
2020-02-04T00:53:43.0469102Z == clock drift check ==
2020-02-04T00:53:43.0486815Z   local time: Tue Feb  4 00:53:43 UTC 2020
2020-02-04T00:53:43.6027043Z   network time: Tue, 04 Feb 2020 00:53:43 GMT
2020-02-04T00:53:43.6032531Z == end clock drift check ==
2020-02-04T00:53:44.0076980Z 
2020-02-04T00:53:44.0182827Z ##[error]Bash exited with code '1'.
2020-02-04T00:53:44.0193319Z ##[section]Finishing: Run build
2020-02-04T00:53:44.0217958Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68814/merge to s
2020-02-04T00:53:44.0219948Z Task         : Get sources
2020-02-04T00:53:44.0220008Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T00:53:44.0220064Z Version      : 1.0.0
2020-02-04T00:53:44.0220102Z Author       : Microsoft
2020-02-04T00:53:44.0220102Z Author       : Microsoft
2020-02-04T00:53:44.0220141Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T00:53:44.0220184Z ==============================================================================
2020-02-04T00:53:44.4163471Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T00:53:44.4204372Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68814/merge to s
2020-02-04T00:53:44.4308673Z Cleaning up task key
2020-02-04T00:53:44.4309369Z Start cleaning up orphan processes.
2020-02-04T00:53:44.4404453Z Terminate orphan process: pid (5195) (python)
2020-02-04T00:53:44.4610297Z ##[section]Finishing: Finalize Job
