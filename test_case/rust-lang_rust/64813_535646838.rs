plain
2019-09-26T17:59:37.9637659Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T17:59:37.9868389Z ##[command]git config gc.auto 0
2019-09-26T17:59:37.9922204Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T17:59:37.9986550Z ##[command]git config --get-all http.proxy
2019-09-26T17:59:38.0146764Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64813/merge:refs/remotes/pull/64813/merge
---
2019-09-26T19:03:20.3610943Z .................................................................................................... 1500/9049
2019-09-26T19:03:26.4890140Z .................................................................................................... 1600/9049
2019-09-26T19:03:39.3748441Z ..........................................................................i...............i......... 1700/9049
2019-09-26T19:03:46.3159152Z .................................................................................................... 1800/9049
2019-09-26T19:03:55.0523888Z .................................................................iiiii.............................. 1900/9049
2019-09-26T19:04:15.2469770Z .................................................................................................... 2100/9049
2019-09-26T19:04:17.8352377Z .................................................................................................... 2200/9049
2019-09-26T19:04:21.1070378Z .................................................................................................... 2300/9049
2019-09-26T19:04:30.0050910Z .................................................................................................... 2400/9049
---
2019-09-26T19:07:33.6644263Z ........................................................i...............i........................... 4700/9049
2019-09-26T19:07:43.0217443Z .................................................................................................... 4800/9049
2019-09-26T19:07:51.7496479Z .................................................................................................... 4900/9049
2019-09-26T19:07:59.4375203Z .................................................................................................... 5000/9049
2019-09-26T19:08:09.3938110Z ...........................................ii.ii.................................................... 5100/9049
2019-09-26T19:08:19.6226928Z .................................................................................................... 5300/9049
2019-09-26T19:08:30.2425552Z .................................................................................................... 5400/9049
2019-09-26T19:08:37.7308220Z ........i........................................................................................... 5500/9049
2019-09-26T19:08:43.4507520Z .................................................................................................... 5600/9049
2019-09-26T19:08:43.4507520Z .................................................................................................... 5600/9049
2019-09-26T19:08:55.6518023Z .................................................................................................... 5700/9049
2019-09-26T19:09:08.9615894Z ...ii...i..ii...........i........................................................................... 5800/9049
2019-09-26T19:09:31.8130866Z .................................................................................................... 6000/9049
2019-09-26T19:09:38.9258978Z .................................................................................................... 6100/9049
2019-09-26T19:09:38.9258978Z .................................................................................................... 6100/9049
2019-09-26T19:09:53.0879767Z ......i..ii......................................................................................... 6200/9049
2019-09-26T19:10:12.5276988Z ..................................................................i................................. 6400/9049
2019-09-26T19:10:14.7332558Z .................................................................................................... 6500/9049
2019-09-26T19:10:17.2954977Z ......................................i............................................................. 6600/9049
2019-09-26T19:10:21.3779655Z .................................................................................................... 6700/9049
---
2019-09-26T19:11:16.6079973Z .................................................................................................... 7200/9049
2019-09-26T19:11:22.0857656Z .................................................................................................... 7300/9049
2019-09-26T19:11:29.6259459Z .................................................................................................... 7400/9049
2019-09-26T19:11:40.2336157Z .................................................................................................... 7500/9049
2019-09-26T19:11:50.4545709Z ..........................................................................................ii......i. 7600/9049
2019-09-26T19:12:01.8493227Z .................................................................................................... 7800/9049
2019-09-26T19:12:19.1213751Z .................................................................................................... 7900/9049
2019-09-26T19:12:27.8906173Z .................................................................................................... 8000/9049
2019-09-26T19:12:38.1981820Z .................................................................................................... 8100/9049
---
2019-09-26T19:14:29.0280347Z 
2019-09-26T19:14:29.0281057Z ---- [ui] ui/ast-json/ast-json-output.rs stdout ----
2019-09-26T19:14:29.0281119Z diff of stdout:
2019-09-26T19:14:29.0281178Z 
2019-09-26T19:14:29.0282012Z - {"module":{"inner":{"lo":0,"hi":0},"items":[{"ident":{"name":"core","span":{"lo":0,"hi":0}},"attrs":[],"id":0,"node":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"span":{"lo":0,"hi":0},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]}]}],"inline":true},"attrs":[],"span":{"lo":0,"hi":0}}
2019-09-26T19:14:29.0282213Z + {"module":{"inner":{"lo":0,"hi":0},"items":[{"ident":{"name":"core","span":{"lo":0,"hi":0}},"attrs":[],"id":0,"kind":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"span":{"lo":0,"hi":0},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]}]}],"inline":true},"attrs":[],"span":{"lo":0,"hi":0}}
2019-09-26T19:14:29.0282355Z 
2019-09-26T19:14:29.0282387Z 
2019-09-26T19:14:29.0282430Z The actual stdout differed from the expected stdout.
2019-09-26T19:14:29.0282775Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
2019-09-26T19:14:29.0282775Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
2019-09-26T19:14:29.0283026Z To update references, rerun the tests and pass the `--bless` flag
2019-09-26T19:14:29.0283337Z To only update this specific test, also pass `--test-args ast-json/ast-json-output.rs`
2019-09-26T19:14:29.0283418Z error: 1 errors occurred comparing output.
2019-09-26T19:14:29.0283418Z error: 1 errors occurred comparing output.
2019-09-26T19:14:29.0283764Z failed to decode compiler output as json: line: {"module":{"inner":{"lo":218,"hi":236},"items":[{"ident":{"name":"core","span":{"lo":231,"hi":235}},"attrs":[],"id":4294967040,"kind":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":218,"hi":218}},"span":{"lo":218,"hi":236},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":218,"hi":224}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":225,"hi":230}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":231,"hi":235}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":235,"hi":236}}]}]}],"inline":true},"attrs":[],"span":{"lo":218,"hi":236}}
2019-09-26T19:14:29.0284075Z output: {"module":{"inner":{"lo":218,"hi":236},"items":[{"ident":{"name":"core","span":{"lo":231,"hi":235}},"attrs":[],"id":4294967040,"kind":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":218,"hi":218}},"span":{"lo":218,"hi":236},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":218,"hi":224}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":225,"hi":230}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":231,"hi":235}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":235,"hi":236}}]}]}],"inline":true},"attrs":[],"span":{"lo":218,"hi":236}}
2019-09-26T19:14:29.0284494Z thread '[ui] ui/ast-json/ast-json-output.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
2019-09-26T19:14:29.0284611Z 
2019-09-26T19:14:29.0284637Z 
2019-09-26T19:14:29.0284695Z failures:
2019-09-26T19:14:29.0284928Z     [ui] ui/ast-json/ast-json-output.rs
2019-09-26T19:14:29.0284928Z     [ui] ui/ast-json/ast-json-output.rs
2019-09-26T19:14:29.0284960Z 
2019-09-26T19:14:29.0285375Z test result: FAILED. 9010 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2019-09-26T19:14:29.0285425Z 
2019-09-26T19:14:29.0320919Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-26T19:14:29.0335180Z 
2019-09-26T19:14:29.0335720Z 
2019-09-26T19:14:29.0340311Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-26T19:14:29.0340668Z 
2019-09-26T19:14:29.0340721Z 
2019-09-26T19:14:29.0359491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-26T19:14:29.0359574Z Build completed unsuccessfully in 1:07:43
2019-09-26T19:14:29.0359574Z Build completed unsuccessfully in 1:07:43
2019-09-26T19:14:29.0414647Z == clock drift check ==
2019-09-26T19:14:29.0432708Z   local time: Thu Sep 26 19:14:29 UTC 2019
2019-09-26T19:14:29.1414434Z   network time: Thu, 26 Sep 2019 19:14:29 GMT
2019-09-26T19:14:29.1421828Z == end clock drift check ==
2019-09-26T19:14:30.3958292Z ##[error]Bash exited with code '1'.
2019-09-26T19:14:30.4002645Z ##[section]Starting: Checkout
2019-09-26T19:14:30.4004732Z ==============================================================================
2019-09-26T19:14:30.4004784Z Task         : Get sources
2019-09-26T19:14:30.4004844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
