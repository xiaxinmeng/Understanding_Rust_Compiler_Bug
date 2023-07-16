plain
2019-08-24T15:31:31.1020985Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T15:31:31.1205613Z ##[command]git config gc.auto 0
2019-08-24T15:31:31.1287711Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T15:31:31.1360759Z ##[command]git config --get-all http.proxy
2019-08-24T15:31:31.1503552Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63853/merge:refs/remotes/pull/63853/merge
---
2019-08-24T15:32:05.5479042Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T15:32:05.5480377Z 
2019-08-24T15:32:05.5481877Z   git checkout -b <new-branch-name>
2019-08-24T15:32:05.5483391Z 
2019-08-24T15:32:05.5484756Z HEAD is now at 190fcc267 Merge 9cb7414a20f67755b70225334dd0d179bd1a3863 into 478464570e60523adc6d303577d1782229ca1f93
2019-08-24T15:32:05.5639593Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T15:32:05.5641988Z ==============================================================================
2019-08-24T15:32:05.5642067Z Task         : Bash
2019-08-24T15:32:05.5642107Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T16:33:19.0270898Z .................................................................................................... 1500/8953
2019-08-24T16:33:24.4355993Z .................................................................................................... 1600/8953
2019-08-24T16:33:36.7793041Z ..............................................i...............i..................................... 1700/8953
2019-08-24T16:33:44.7499847Z .................................................................................................... 1800/8953
2019-08-24T16:33:58.5749149Z ......................................iiiii......................................................... 1900/8953
2019-08-24T16:34:08.9006134Z .................................................................................................... 2100/8953
2019-08-24T16:34:11.3230230Z .................................................................................................... 2200/8953
2019-08-24T16:34:15.3652692Z .................................................................................................... 2300/8953
2019-08-24T16:34:22.7708984Z .................................................................................................... 2400/8953
---
2019-08-24T16:37:16.6500417Z ..........................i...............i......................................................... 4700/8953
2019-08-24T16:37:28.7304531Z .................................................................................................... 4800/8953
2019-08-24T16:37:35.9016039Z .................................................................................................... 4900/8953
2019-08-24T16:37:45.6649024Z .................................................................................................... 5000/8953
2019-08-24T16:37:51.1446857Z .......ii.ii........................................................................................ 5100/8953
2019-08-24T16:38:04.9463590Z .................................................................................................... 5300/8953
2019-08-24T16:38:12.1019560Z ...............................................................i.................................... 5400/8953
2019-08-24T16:38:19.2917394Z .................................................................................................... 5500/8953
2019-08-24T16:38:26.6602966Z .................................................................................................... 5600/8953
2019-08-24T16:38:26.6602966Z .................................................................................................... 5600/8953
2019-08-24T16:38:37.8235472Z .........................................................ii...i..ii...........i..................... 5700/8953
2019-08-24T16:39:00.0955409Z .................................................................................................... 5900/8953
2019-08-24T16:39:05.1238227Z .................................................................................................... 6000/8953
2019-08-24T16:39:05.1238227Z .................................................................................................... 6000/8953
2019-08-24T16:39:12.3305629Z ..........................................................i..ii..................................... 6100/8953
2019-08-24T16:39:40.6385058Z .................................................................................................... 6300/8953
2019-08-24T16:39:42.8669379Z ....i............................................................................................... 6400/8953
2019-08-24T16:39:45.0981718Z ............................................................................i....................... 6500/8953
2019-08-24T16:39:47.8191345Z .................................................................................................... 6600/8953
---
2019-08-24T16:43:41.4779465Z ..............................................................i..................................... 8900/8953
2019-08-24T16:43:47.5342448Z .....................................................
2019-08-24T16:43:47.5342684Z failures:
2019-08-24T16:43:47.5372935Z 
2019-08-24T16:43:47.5373484Z ---- [ui] ui/ast-json/ast-json-output.rs stdout ----
2019-08-24T16:43:47.5373600Z 
2019-08-24T16:43:47.5373600Z 
2019-08-24T16:43:47.5374844Z - {"module":{"inner":{"lo":93,"hi":111},"items":[{"ident":{"name":"core","span":{"lo":106,"hi":110}},"attrs":[],"id":4294967040,"node":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":93,"hi":93}},"span":{"lo":93,"hi":111},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":93,"hi":99}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":100,"hi":105}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":106,"hi":110}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":110,"hi":111}}]}]}],"inline":true},"attrs":[],"span":{"lo":93,"hi":111}}
2019-08-24T16:43:47.5375062Z + {"module":{"inner":{"lo":178,"hi":196},"items":[{"ident":{"name":"core","span":{"lo":191,"hi":195}},"attrs":[],"id":4294967040,"node":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":178,"hi":178}},"span":{"lo":178,"hi":196},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":178,"hi":184}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":185,"hi":190}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":191,"hi":195}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":195,"hi":196}}]}]}],"inline":true},"attrs":[],"span":{"lo":178,"hi":196}}
2019-08-24T16:43:47.5375408Z 
2019-08-24T16:43:47.5375435Z 
2019-08-24T16:43:47.5375490Z The actual stdout differed from the expected stdout.
2019-08-24T16:43:47.5375490Z The actual stdout differed from the expected stdout.
2019-08-24T16:43:47.5375880Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
2019-08-24T16:43:47.5376147Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T16:43:47.5376466Z To only update this specific test, also pass `--test-args ast-json/ast-json-output.rs`
2019-08-24T16:43:47.5376574Z error: 1 errors occurred comparing output.
2019-08-24T16:43:47.5376574Z error: 1 errors occurred comparing output.
2019-08-24T16:43:47.5376717Z failed to decode compiler output as json: line: {"module":{"inner":{"lo":178,"hi":196},"items":[{"ident":{"name":"core","span":{"lo":191,"hi":195}},"attrs":[],"id":4294967040,"node":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":178,"hi":178}},"span":{"lo":178,"hi":196},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":178,"hi":184}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":185,"hi":190}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":191,"hi":195}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":195,"hi":196}}]}]}],"inline":true},"attrs":[],"span":{"lo":178,"hi":196}}
2019-08-24T16:43:47.5377025Z output: {"module":{"inner":{"lo":178,"hi":196},"items":[{"ident":{"name":"core","span":{"lo":191,"hi":195}},"attrs":[],"id":4294967040,"node":{"variant":"ExternCrate","fields":[null]},"vis":{"node":"Inherited","span":{"lo":178,"hi":178}},"span":{"lo":178,"hi":196},"tokens":[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":178,"hi":184}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":185,"hi":190}}]},{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":191,"hi":195}}]},{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":195,"hi":196}}]}]}],"inline":true},"attrs":[],"span":{"lo":178,"hi":196}}
2019-08-24T16:43:47.5377463Z thread '[ui] ui/ast-json/ast-json-output.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
2019-08-24T16:43:47.5377595Z 
2019-08-24T16:43:47.5377624Z 
2019-08-24T16:43:47.5377667Z failures:
2019-08-24T16:43:47.5377667Z failures:
2019-08-24T16:43:47.5377924Z     [ui] ui/ast-json/ast-json-output.rs
2019-08-24T16:43:47.5378248Z test result: FAILED. 8915 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out
2019-08-24T16:43:47.5378284Z 
2019-08-24T16:43:47.5407082Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-24T16:43:47.5422585Z 
2019-08-24T16:43:47.5422585Z 
2019-08-24T16:43:47.5422667Z 
2019-08-24T16:43:47.5428994Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-24T16:43:47.5431820Z 
2019-08-24T16:43:47.5432089Z 
2019-08-24T16:43:47.5440046Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-24T16:43:47.5440757Z Build completed unsuccessfully in 1:05:37
2019-08-24T16:43:47.5440757Z Build completed unsuccessfully in 1:05:37
2019-08-24T16:43:47.5495630Z == clock drift check ==
2019-08-24T16:43:47.5512092Z   local time: Sat Aug 24 16:43:47 UTC 2019
2019-08-24T16:43:47.7047312Z   network time: Sat, 24 Aug 2019 16:43:47 GMT
2019-08-24T16:43:47.7047477Z == end clock drift check ==
2019-08-24T16:43:48.4694813Z ##[error]Bash exited with code '1'.
2019-08-24T16:43:48.4735140Z ##[section]Starting: Checkout
2019-08-24T16:43:48.4736840Z ==============================================================================
2019-08-24T16:43:48.4736893Z Task         : Get sources
2019-08-24T16:43:48.4736962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
