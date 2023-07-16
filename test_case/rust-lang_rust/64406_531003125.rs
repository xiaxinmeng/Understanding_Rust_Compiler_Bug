plain
2019-09-12T19:40:34.8737349Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T19:40:35.7334411Z ##[command]git config gc.auto 0
2019-09-12T19:40:35.7341177Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T19:40:35.7345957Z ##[command]git config --get-all http.proxy
2019-09-12T19:40:35.7349080Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64406/merge:refs/remotes/pull/64406/merge
---
2019-09-12T20:44:07.9517212Z .................................................................................................... 1500/9012
2019-09-12T20:44:13.9147721Z .................................................................................................... 1600/9012
2019-09-12T20:44:26.6534862Z .........................................................i...............i.......................... 1700/9012
2019-09-12T20:44:34.7654815Z .................................................................................................... 1800/9012
2019-09-12T20:44:49.8432815Z ................................................iiiii............................................... 1900/9012
2019-09-12T20:45:00.9307001Z .................................................................................................... 2100/9012
2019-09-12T20:45:03.5700249Z .................................................................................................... 2200/9012
2019-09-12T20:45:07.2502472Z .................................................................................................... 2300/9012
2019-09-12T20:45:15.2497803Z .................................................................................................... 2400/9012
---
2019-09-12T20:48:16.5543522Z ....................................i...............i............................................... 4700/9012
2019-09-12T20:48:27.9463228Z .................................................................................................... 4800/9012
2019-09-12T20:48:34.6026696Z .................................................................................................... 4900/9012
2019-09-12T20:48:45.5971555Z .................................................................................................... 5000/9012
2019-09-12T20:48:51.8553901Z ...................ii.ii............................................................................ 5100/9012
2019-09-12T20:49:02.4211908Z .................................................................................................... 5300/9012
2019-09-12T20:49:12.7195588Z ...................................................................................i................ 5400/9012
2019-09-12T20:49:20.9088740Z .................................................................................................... 5500/9012
2019-09-12T20:49:26.5143572Z .................................................................................................... 5600/9012
2019-09-12T20:49:26.5143572Z .................................................................................................... 5600/9012
2019-09-12T20:49:36.9757602Z ..............................................................................i.i..i..ii...........i 5700/9012
2019-09-12T20:50:02.7153130Z .................................................................................................... 5900/9012
2019-09-12T20:50:11.0268237Z .................................................................................................... 6000/9012
2019-09-12T20:50:11.0268237Z .................................................................................................... 6000/9012
2019-09-12T20:50:15.8746765Z ................................................................................i..ii............... 6100/9012
2019-09-12T20:50:46.9216394Z .................................................................................................... 6300/9012
2019-09-12T20:50:49.1913025Z .......................................i............................................................ 6400/9012
2019-09-12T20:50:51.4291241Z .................................................................................................... 6500/9012
2019-09-12T20:50:53.9891395Z ...........i........................................................................................ 6600/9012
---
2019-09-12T20:55:30.4927214Z  finished in 5.088
2019-09-12T20:55:30.5118967Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-12T20:55:30.6707308Z 
2019-09-12T20:55:30.6707622Z running 150 tests
2019-09-12T20:55:33.9500075Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-12T20:55:35.9478988Z ..iiii..............i.........iii.i.......ii......
2019-09-12T20:55:35.9483386Z 
2019-09-12T20:55:35.9487002Z  finished in 5.437
2019-09-12T20:55:35.9660726Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-12T20:55:36.1218745Z 
---
2019-09-12T20:55:38.2317616Z  finished in 2.265
2019-09-12T20:55:38.2505055Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-12T20:55:38.4085220Z 
2019-09-12T20:55:38.4085587Z running 9 tests
2019-09-12T20:55:38.4086725Z iiiiiiiii
2019-09-12T20:55:38.4087314Z 
2019-09-12T20:55:38.4091038Z  finished in 0.158
2019-09-12T20:55:38.4288641Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-12T20:55:38.5869716Z 
2019-09-12T20:55:38.5869716Z 
2019-09-12T20:55:38.5870340Z running 104 tests
2019-09-12T20:55:55.7421984Z ............................F...........F........................................................... 100/104
2019-09-12T20:55:56.3957871Z ....
2019-09-12T20:55:56.3958068Z failures:
2019-09-12T20:55:56.3958133Z 
2019-09-12T20:55:56.3958439Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-09-12T20:55:56.3958477Z 
2019-09-12T20:55:56.3958722Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-12T20:55:56.3958808Z status: exit code: 1
2019-09-12T20:55:56.3959865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-09-12T20:55:56.3960487Z ------------------------------------------
2019-09-12T20:55:56.3960542Z 
2019-09-12T20:55:56.3960725Z ------------------------------------------
2019-09-12T20:55:56.3960761Z stderr:
2019-09-12T20:55:56.3960761Z stderr:
2019-09-12T20:55:56.3960959Z ------------------------------------------
2019-09-12T20:55:56.3961157Z error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T20:55:56.3961430Z    |
2019-09-12T20:55:56.3961430Z    |
2019-09-12T20:55:56.3961613Z LL | pub extern "rust-intrinsic" fn make_intrinsic() {}
2019-09-12T20:55:56.3963272Z 
2019-09-12T20:55:56.3963330Z error: aborting due to previous error
2019-09-12T20:55:56.3963353Z 
2019-09-12T20:55:56.3963374Z 
2019-09-12T20:55:56.3963374Z 
2019-09-12T20:55:56.3963688Z ------------------------------------------
2019-09-12T20:55:56.3963734Z 
2019-09-12T20:55:56.3963754Z 
2019-09-12T20:55:56.3963944Z ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
2019-09-12T20:55:56.3963971Z 
2019-09-12T20:55:56.3964184Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-12T20:55:56.3964431Z status: exit code: 1
2019-09-12T20:55:56.3965247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
2019-09-12T20:55:56.3966391Z ------------------------------------------
2019-09-12T20:55:56.3966426Z 
2019-09-12T20:55:56.3966661Z ------------------------------------------
2019-09-12T20:55:56.3966706Z stderr:
2019-09-12T20:55:56.3966706Z stderr:
2019-09-12T20:55:56.3966937Z ------------------------------------------
2019-09-12T20:55:56.3967176Z error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T20:55:56.3967505Z    |
2019-09-12T20:55:56.3967505Z    |
2019-09-12T20:55:56.3967727Z LL |     extern "rust-intrinsic" fn method();
2019-09-12T20:55:56.3967806Z 
2019-09-12T20:55:56.3967868Z error: aborting due to previous error
2019-09-12T20:55:56.3967897Z 
2019-09-12T20:55:56.3967923Z 
---
2019-09-12T20:55:56.3969096Z . 102 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-12T20:55:56.3969121Z 
2019-09-12T20:55:56.3971444Z 
2019-09-12T20:55:56.3971511Z 
2019-09-12T20:55:56.3972864Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T20:55:56.3973099Z 
2019-09-12T20:55:56.3973126Z 
2019-09-12T20:55:56.3976780Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-12T20:55:56.3976897Z Build completed unsuccessfully in 1:07:51
2019-09-12T20:55:56.3976897Z Build completed unsuccessfully in 1:07:51
2019-09-12T20:55:56.4032726Z == clock drift check ==
2019-09-12T20:55:56.4045015Z   local time: Thu Sep 12 20:55:56 UTC 2019
2019-09-12T20:55:56.5626990Z   network time: Thu, 12 Sep 2019 20:55:56 GMT
2019-09-12T20:55:56.5627389Z == end clock drift check ==
2019-09-12T20:56:00.7914187Z ##[error]Bash exited with code '1'.
2019-09-12T20:56:00.7968465Z ##[section]Starting: Checkout
2019-09-12T20:56:00.7970777Z ==============================================================================
2019-09-12T20:56:00.7970828Z Task         : Get sources
2019-09-12T20:56:00.7970888Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
