plain
2020-03-20T16:56:06.8576901Z ========================== Starting Command Output ===========================
2020-03-20T16:56:06.8580906Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ed429fc9-5083-4858-bbac-37d2add72f5f.sh
2020-03-20T16:56:06.8581271Z 
2020-03-20T16:56:06.8584651Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T16:56:06.8601239Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70193/merge to s
2020-03-20T16:56:06.8604170Z Task         : Get sources
2020-03-20T16:56:06.8604455Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T16:56:06.8604687Z Version      : 1.0.0
2020-03-20T16:56:06.8605017Z Author       : Microsoft
---
2020-03-20T16:56:08.1178958Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T16:56:08.1187143Z ##[command]git config gc.auto 0
2020-03-20T16:56:08.1190837Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T16:56:08.1195569Z ##[command]git config --get-all http.proxy
2020-03-20T16:56:08.1204226Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70193/merge:refs/remotes/pull/70193/merge
---
2020-03-20T17:53:03.2791796Z .................................................................................................... 1700/9803
2020-03-20T17:53:06.9175949Z .................................................................................................... 1800/9803
2020-03-20T17:53:16.7827168Z ..........................................................................i......................... 1900/9803
2020-03-20T17:53:22.2863816Z .................................................................................................... 2000/9803
2020-03-20T17:53:29.2228122Z ................................................................iiiii............................... 2100/9803
2020-03-20T17:53:44.9896927Z .................................................................................................... 2300/9803
2020-03-20T17:53:46.9509139Z .................................................................................................... 2400/9803
2020-03-20T17:53:49.5492603Z .................................................................................................... 2500/9803
2020-03-20T17:54:08.4137927Z .................................................................................................... 2600/9803
---
2020-03-20T17:56:38.6684503Z .....................................i...............i.............................................. 5000/9803
2020-03-20T17:56:46.8549497Z .................................................................................................... 5100/9803
2020-03-20T17:56:53.0130344Z ................................................................................i................... 5200/9803
2020-03-20T17:56:58.4040530Z .................................................................................................... 5300/9803
2020-03-20T17:57:08.3160059Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-20T17:57:16.2112926Z i................................................................................................... 5600/9803
2020-03-20T17:57:25.2630002Z .....i.............................................................................................. 5700/9803
2020-03-20T17:57:31.5735406Z ........................................................i........................................... 5800/9803
2020-03-20T17:57:37.7793312Z .................................................................................................... 5900/9803
2020-03-20T17:57:37.7793312Z .................................................................................................... 5900/9803
2020-03-20T17:57:45.4232053Z .................................................................................................... 6000/9803
2020-03-20T17:57:52.5351007Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-20T17:58:11.6078090Z .................................................................................................... 6300/9803
2020-03-20T17:58:17.7541144Z .................................................................................................... 6400/9803
2020-03-20T17:58:17.7541144Z .................................................................................................... 6400/9803
2020-03-20T17:58:21.3971036Z ................................................................................i..ii............... 6500/9803
2020-03-20T17:58:42.5850907Z .................................................................................................... 6700/9803
2020-03-20T17:58:51.2712878Z ...............................................................................i.................... 6800/9803
2020-03-20T17:58:53.2133253Z .................................................................................................... 6900/9803
2020-03-20T17:58:55.0169056Z .................................................................................................... 7000/9803
---
2020-03-20T18:00:30.7779959Z .......................................................F............................................ 7800/9803
2020-03-20T18:00:35.6987501Z .................................................................................................... 7900/9803
2020-03-20T18:00:41.2283863Z ..................................................................i................................. 8000/9803
2020-03-20T18:00:50.3833884Z .................................................................................................... 8100/9803
2020-03-20T18:00:55.6214923Z ...............iiiiiiiiii..i........................................................................ 8200/9803
2020-03-20T18:01:08.6734653Z .................................................................................................... 8400/9803
2020-03-20T18:01:14.3785909Z .................................................................................................... 8500/9803
2020-03-20T18:01:28.3559123Z .................................................................................................... 8600/9803
2020-03-20T18:01:34.3415245Z .................................................................................................... 8700/9803
---
2020-03-20T18:03:18.4241438Z 
2020-03-20T18:03:18.4241588Z 10 LL | #[repr(align(15))]
2020-03-20T18:03:18.4241796Z 11    |        ^^^^^^^^^
2020-03-20T18:03:18.4242054Z 12 
2020-03-20T18:03:18.4242745Z - error[E0589]: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4243286Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4243949Z 15    |
2020-03-20T18:03:18.4244120Z 16 LL | #[repr(align(4294967296))]
2020-03-20T18:03:18.4244261Z 
2020-03-20T18:03:18.4244447Z 17    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4244447Z 17    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4244602Z 18 
2020-03-20T18:03:18.4244827Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4245470Z +    |
2020-03-20T18:03:18.4245470Z +    |
2020-03-20T18:03:18.4245736Z + LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4246185Z + 
2020-03-20T18:03:18.4246185Z + 
2020-03-20T18:03:18.4246426Z 19 error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4247240Z 21    |
2020-03-20T18:03:18.4247334Z 
2020-03-20T18:03:18.4247473Z 28 LL | #[repr(align(15))]
2020-03-20T18:03:18.4247661Z 29    |        ^^^^^^^^^
2020-03-20T18:03:18.4247661Z 29    |        ^^^^^^^^^
2020-03-20T18:03:18.4247798Z 30 
2020-03-20T18:03:18.4248201Z - error[E0589]: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4248535Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4249152Z 33    |
2020-03-20T18:03:18.4249312Z 34 LL | #[repr(align(4294967296))]
2020-03-20T18:03:18.4249465Z 
2020-03-20T18:03:18.4249612Z 35    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4249612Z 35    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4249764Z 36 
2020-03-20T18:03:18.4249997Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4250596Z +    |
2020-03-20T18:03:18.4250596Z +    |
2020-03-20T18:03:18.4250846Z + LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4251282Z + 
2020-03-20T18:03:18.4251282Z + 
2020-03-20T18:03:18.4251515Z 37 error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4252141Z 39    |
2020-03-20T18:03:18.4252236Z 
2020-03-20T18:03:18.4252391Z 46 LL | #[repr(align(15))]
2020-03-20T18:03:18.4252564Z 47    |        ^^^^^^^^^
2020-03-20T18:03:18.4252564Z 47    |        ^^^^^^^^^
2020-03-20T18:03:18.4252871Z 48 
2020-03-20T18:03:18.4253341Z - error[E0589]: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4253696Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4254710Z 51    |
2020-03-20T18:03:18.4254897Z 52 LL | #[repr(align(4294967296))]
2020-03-20T18:03:18.4255032Z 
2020-03-20T18:03:18.4255359Z 53    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4255359Z 53    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4255530Z 54 
2020-03-20T18:03:18.4255758Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4257304Z +    |
2020-03-20T18:03:18.4257304Z +    |
2020-03-20T18:03:18.4257787Z + LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4258261Z + 
2020-03-20T18:03:18.4258261Z + 
2020-03-20T18:03:18.4258543Z 55 error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4259258Z 57    |
2020-03-20T18:03:18.4259383Z 
2020-03-20T18:03:18.4259540Z 64 LL | #[repr(align(15))]
2020-03-20T18:03:18.4259733Z 65    |        ^^^^^^^^^
2020-03-20T18:03:18.4259733Z 65    |        ^^^^^^^^^
2020-03-20T18:03:18.4259888Z 66 
2020-03-20T18:03:18.4260381Z - error[E0589]: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4261338Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4262155Z 69    |
2020-03-20T18:03:18.4262353Z 70 LL | #[repr(align(4294967296))]
2020-03-20T18:03:18.4262518Z 
2020-03-20T18:03:18.4262714Z 71    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4262714Z 71    |        ^^^^^^^^^^^^^^^^^
2020-03-20T18:03:18.4262896Z 72 
2020-03-20T18:03:18.4263310Z - error: aborting due to 12 previous errors
2020-03-20T18:03:18.4263661Z + error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4264790Z +    |
2020-03-20T18:03:18.4264790Z +    |
2020-03-20T18:03:18.4265063Z + LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4265528Z + 
2020-03-20T18:03:18.4265704Z + error: aborting due to 16 previous errors
2020-03-20T18:03:18.4265904Z 74 
2020-03-20T18:03:18.4266338Z 75 For more information about this error, try `rustc --explain E0589`.
2020-03-20T18:03:18.4266338Z 75 For more information about this error, try `rustc --explain E0589`.
2020-03-20T18:03:18.4266561Z 76 
2020-03-20T18:03:18.4266674Z 
2020-03-20T18:03:18.4266764Z 
2020-03-20T18:03:18.4266953Z The actual stderr differed from the expected stderr.
2020-03-20T18:03:18.4267710Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-align/repr-align.stderr
2020-03-20T18:03:18.4268291Z To update references, rerun the tests and pass the `--bless` flag
2020-03-20T18:03:18.4268808Z To only update this specific test, also pass `--test-args repr/repr-align.rs`
2020-03-20T18:03:18.4269202Z error: 1 errors occurred comparing output.
2020-03-20T18:03:18.4269414Z status: exit code: 1
2020-03-20T18:03:18.4269414Z status: exit code: 1
2020-03-20T18:03:18.4271228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-align.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-align" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-align/auxiliary"
2020-03-20T18:03:18.4272541Z ------------------------------------------
2020-03-20T18:03:18.4272706Z 
2020-03-20T18:03:18.4273019Z ------------------------------------------
2020-03-20T18:03:18.4273188Z stderr:
2020-03-20T18:03:18.4273188Z stderr:
2020-03-20T18:03:18.4273520Z ------------------------------------------
2020-03-20T18:03:18.4273910Z error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4274633Z    |
2020-03-20T18:03:18.4274633Z    |
2020-03-20T18:03:18.4274918Z LL | #[repr(align(16.0))] //~ ERROR: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4275431Z 
2020-03-20T18:03:18.4275431Z 
2020-03-20T18:03:18.4275639Z error[E0589]: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4276328Z    |
2020-03-20T18:03:18.4276328Z    |
2020-03-20T18:03:18.4276595Z LL | #[repr(align(15))] //~ ERROR: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4276998Z 
2020-03-20T18:03:18.4276998Z 
2020-03-20T18:03:18.4277221Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4278069Z    |
2020-03-20T18:03:18.4278069Z    |
2020-03-20T18:03:18.4281159Z LL | #[repr(align(4294967296))] //~ ERROR: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4281941Z 
2020-03-20T18:03:18.4281941Z 
2020-03-20T18:03:18.4282150Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4284091Z    |
2020-03-20T18:03:18.4284091Z    |
2020-03-20T18:03:18.4285246Z LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4285689Z 
2020-03-20T18:03:18.4285689Z 
2020-03-20T18:03:18.4285906Z error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4286699Z    |
2020-03-20T18:03:18.4286699Z    |
2020-03-20T18:03:18.4286984Z LL | #[repr(align(16.0))] //~ ERROR: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4287423Z 
2020-03-20T18:03:18.4287423Z 
2020-03-20T18:03:18.4287639Z error[E0589]: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4288315Z    |
2020-03-20T18:03:18.4288315Z    |
2020-03-20T18:03:18.4288579Z LL | #[repr(align(15))] //~ ERROR: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4289007Z 
2020-03-20T18:03:18.4289007Z 
2020-03-20T18:03:18.4289407Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4290094Z    |
2020-03-20T18:03:18.4290094Z    |
2020-03-20T18:03:18.4290376Z LL | #[repr(align(4294967296))] //~ ERROR: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4290813Z 
2020-03-20T18:03:18.4290813Z 
2020-03-20T18:03:18.4291039Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4291696Z    |
2020-03-20T18:03:18.4291696Z    |
2020-03-20T18:03:18.4291959Z LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4292357Z 
2020-03-20T18:03:18.4292357Z 
2020-03-20T18:03:18.4292580Z error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4293424Z    |
2020-03-20T18:03:18.4293424Z    |
2020-03-20T18:03:18.4293709Z LL | #[repr(align(16.0))] //~ ERROR: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4295877Z 
2020-03-20T18:03:18.4295877Z 
2020-03-20T18:03:18.4296089Z error[E0589]: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4296873Z    |
2020-03-20T18:03:18.4296873Z    |
2020-03-20T18:03:18.4297137Z LL | #[repr(align(15))] //~ ERROR: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4297745Z 
2020-03-20T18:03:18.4297745Z 
2020-03-20T18:03:18.4298090Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4299251Z    |
2020-03-20T18:03:18.4299251Z    |
2020-03-20T18:03:18.4299553Z LL | #[repr(align(4294967296))] //~ ERROR: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4300135Z 
2020-03-20T18:03:18.4300135Z 
2020-03-20T18:03:18.4300359Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4301109Z    |
2020-03-20T18:03:18.4301109Z    |
2020-03-20T18:03:18.4301363Z LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4301958Z 
2020-03-20T18:03:18.4301958Z 
2020-03-20T18:03:18.4302207Z error[E0589]: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4302901Z    |
2020-03-20T18:03:18.4302901Z    |
2020-03-20T18:03:18.4303225Z LL | #[repr(align(16.0))] //~ ERROR: invalid `repr(align)` attribute: not an unsuffixed integer
2020-03-20T18:03:18.4303828Z 
2020-03-20T18:03:18.4303828Z 
2020-03-20T18:03:18.4304032Z error[E0589]: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4304706Z    |
2020-03-20T18:03:18.4304706Z    |
2020-03-20T18:03:18.4304969Z LL | #[repr(align(15))] //~ ERROR: invalid `repr(align)` attribute: not a power of two
2020-03-20T18:03:18.4305381Z 
2020-03-20T18:03:18.4305381Z 
2020-03-20T18:03:18.4305583Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4306442Z    |
2020-03-20T18:03:18.4306442Z    |
2020-03-20T18:03:18.4306720Z LL | #[repr(align(4294967296))] //~ ERROR: invalid `repr(align)` attribute: larger than 2^29
2020-03-20T18:03:18.4307182Z 
2020-03-20T18:03:18.4307182Z 
2020-03-20T18:03:18.4307391Z error[E0589]: invalid `repr(align)` attribute: larger than 4096
2020-03-20T18:03:18.4308070Z    |
2020-03-20T18:03:18.4308070Z    |
2020-03-20T18:03:18.4308306Z LL | #[repr(align(536870912))] // ok: this is the largest accepted alignment
2020-03-20T18:03:18.4308886Z 
2020-03-20T18:03:18.4309045Z error: aborting due to 16 previous errors
2020-03-20T18:03:18.4309186Z 
2020-03-20T18:03:18.4309572Z For more information about this error, try `rustc --explain E0589`.
---
2020-03-20T18:03:18.4312343Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-20T18:03:18.4312715Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-20T18:03:18.4312922Z 
2020-03-20T18:03:18.4313006Z 
2020-03-20T18:03:18.4316275Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-20T18:03:18.4318618Z 
2020-03-20T18:03:18.4318704Z 
2020-03-20T18:03:18.4319183Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-20T18:03:18.4319522Z Build completed unsuccessfully in 1:01:56
2020-03-20T18:03:18.4319522Z Build completed unsuccessfully in 1:01:56
2020-03-20T18:03:18.4344346Z == clock drift check ==
2020-03-20T18:03:18.4368341Z   local time: Fri Mar 20 18:03:18 UTC 2020
2020-03-20T18:03:18.5269701Z   network time: Fri, 20 Mar 2020 18:03:18 GMT
2020-03-20T18:03:18.5276775Z == end clock drift check ==
2020-03-20T18:03:18.9981873Z 
2020-03-20T18:03:19.0046788Z ##[error]Bash exited with code '1'.
2020-03-20T18:03:19.0061771Z ##[section]Finishing: Run build
2020-03-20T18:03:19.0107488Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70193/merge to s
2020-03-20T18:03:19.0112133Z Task         : Get sources
2020-03-20T18:03:19.0112444Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T18:03:19.0112706Z Version      : 1.0.0
2020-03-20T18:03:19.0112891Z Author       : Microsoft
2020-03-20T18:03:19.0112891Z Author       : Microsoft
2020-03-20T18:03:19.0113210Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T18:03:19.0113548Z ==============================================================================
2020-03-20T18:03:19.3403092Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T18:03:19.3447900Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70193/merge to s
2020-03-20T18:03:19.3529955Z Cleaning up task key
2020-03-20T18:03:19.3531050Z Start cleaning up orphan processes.
2020-03-20T18:03:19.3795059Z Terminate orphan process: pid (3964) (python)
2020-03-20T18:03:19.3836031Z ##[section]Finishing: Finalize Job
