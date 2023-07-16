plain
2020-03-05T19:07:33.6477477Z ========================== Starting Command Output ===========================
2020-03-05T19:07:33.6479932Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c1ed17ed-38e4-4e85-b40f-bfd8a2553353.sh
2020-03-05T19:07:33.6480216Z 
2020-03-05T19:07:33.6483354Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-05T19:07:33.6501109Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69573/merge to s
2020-03-05T19:07:33.6504111Z Task         : Get sources
2020-03-05T19:07:33.6504427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T19:07:33.6504716Z Version      : 1.0.0
2020-03-05T19:07:33.6504901Z Author       : Microsoft
---
2020-03-05T19:07:34.6472486Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-05T19:07:34.6481522Z ##[command]git config gc.auto 0
2020-03-05T19:07:34.6487839Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-05T19:07:34.6492149Z ##[command]git config --get-all http.proxy
2020-03-05T19:07:34.6497646Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69573/merge:refs/remotes/pull/69573/merge
---
2020-03-05T20:03:39.4812391Z .................................................................................................... 1700/9729
2020-03-05T20:03:43.7389591Z .................................................................................................... 1800/9729
2020-03-05T20:03:54.5422630Z ........................................................i........................................... 1900/9729
2020-03-05T20:04:01.7029754Z .................................................................................................... 2000/9729
2020-03-05T20:04:14.6036357Z ..............................................iiiii................................................. 2100/9729
2020-03-05T20:04:23.9985532Z .................................................................................................... 2300/9729
2020-03-05T20:04:26.1327479Z .................................................................................................... 2400/9729
2020-03-05T20:04:29.3677460Z .................................................................................................... 2500/9729
2020-03-05T20:04:49.4595576Z .................................................................................................... 2600/9729
---
2020-03-05T20:07:17.5707768Z .......i...............i............................................................................ 5000/9729
2020-03-05T20:07:26.7798519Z .................................................................................................... 5100/9729
2020-03-05T20:07:31.3727712Z ..................................................i................................................. 5200/9729
2020-03-05T20:07:39.5051801Z .................................................................................................... 5300/9729
2020-03-05T20:07:46.0929705Z .............................ii.ii........i...i..................................................... 5400/9729
2020-03-05T20:07:53.5903811Z .................................................................................................... 5600/9729
2020-03-05T20:08:02.1772910Z .................................................................................................... 5700/9729
2020-03-05T20:08:08.9195985Z ....................i............................................................................... 5800/9729
2020-03-05T20:08:14.0471336Z .................................................................................................... 5900/9729
2020-03-05T20:08:14.0471336Z .................................................................................................... 5900/9729
2020-03-05T20:08:24.0249313Z .................................................................................................... 6000/9729
2020-03-05T20:08:34.4146313Z ............ii...i..ii...........i.................................................................. 6100/9729
2020-03-05T20:08:49.1142748Z .................................................................................................... 6300/9729
2020-03-05T20:08:52.3861178Z .................................................................................................... 6400/9729
2020-03-05T20:08:52.3861178Z .................................................................................................... 6400/9729
2020-03-05T20:09:03.2742359Z ...........................................i..ii.................................................... 6500/9729
2020-03-05T20:09:24.1503070Z .................................................................................................... 6700/9729
2020-03-05T20:09:26.0361572Z ...................................i................................................................ 6800/9729
2020-03-05T20:09:27.9593199Z .................................................................................................... 6900/9729
2020-03-05T20:09:30.0019213Z .................................................................i.................................. 7000/9729
---
2020-03-05T20:11:01.8565628Z .................................................................................................... 7700/9729
2020-03-05T20:11:06.7144220Z .................................................................................................... 7800/9729
2020-03-05T20:11:11.2944452Z .................................................................................................... 7900/9729
2020-03-05T20:11:19.2696630Z ...........i........................................................................................ 8000/9729
2020-03-05T20:11:27.3719369Z ............................................................iiiiiiiii.i............................. 8100/9729
2020-03-05T20:11:40.6654611Z ...i......i......................................................................................... 8300/9729
2020-03-05T20:11:45.6369736Z .................................................................................................... 8400/9729
2020-03-05T20:11:58.7178354Z .................................................................................................... 8500/9729
2020-03-05T20:12:06.8871649Z .................................................................................................... 8600/9729
---
2020-03-05T20:13:57.8387890Z 
2020-03-05T20:13:57.8388165Z 32 error[E0382]: use of moved value: `m`
2020-03-05T20:13:57.8389060Z 33   --> $DIR/issue-53114-borrow-checks.rs:36:16
2020-03-05T20:13:57.8389320Z 34    |
2020-03-05T20:13:57.8389683Z - 34 |     let m = M;
2020-03-05T20:13:57.8389886Z + LL |     let m = M;
2020-03-05T20:13:57.8390687Z 36    |         - move occurs because `m` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8391199Z - 35 |     drop(m);
2020-03-05T20:13:57.8391395Z + LL |     drop(m);
2020-03-05T20:13:57.8391788Z 38    |          - value moved here
2020-03-05T20:13:57.8392356Z - 36 |     if let _ = m { } // #53114: should eventually be accepted too
2020-03-05T20:13:57.8392741Z + LL |     if let _ = m { } // #53114: should eventually be accepted too
2020-03-05T20:13:57.8393081Z 40    |                ^ value used here after move
2020-03-05T20:13:57.8393520Z 42 error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8393712Z 
2020-03-05T20:13:57.8394126Z 43   --> $DIR/issue-53114-borrow-checks.rs:41:22
2020-03-05T20:13:57.8394375Z 44    |
2020-03-05T20:13:57.8394375Z 44    |
2020-03-05T20:13:57.8394736Z - 40 |     if let (_x, _) = mm { }
2020-03-05T20:13:57.8394988Z + LL |     if let (_x, _) = mm { }
2020-03-05T20:13:57.8395426Z 46    |             -- value moved here
2020-03-05T20:13:57.8395849Z - 41 |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8396090Z + LL |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8396398Z 48    |                      ^^ value used here after partial move
2020-03-05T20:13:57.8396642Z 49    |
2020-03-05T20:13:57.8396946Z 50    = note: move occurs because `mm.0` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8397441Z 52 error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8397950Z 53   --> $DIR/issue-53114-borrow-checks.rs:43:21
2020-03-05T20:13:57.8398188Z 54    |
2020-03-05T20:13:57.8398188Z 54    |
2020-03-05T20:13:57.8398564Z - 41 |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8398808Z + LL |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8399233Z 56    |                -- value moved here
2020-03-05T20:13:57.8399596Z - 42 |
2020-03-05T20:13:57.8399963Z - 43 |     if let (_, _) = mm { }
2020-03-05T20:13:57.8400156Z + LL |
2020-03-05T20:13:57.8400361Z + LL |     if let (_, _) = mm { }
2020-03-05T20:13:57.8400652Z 59    |                     ^^ value used here after partial move
2020-03-05T20:13:57.8400895Z 60    |
2020-03-05T20:13:57.8401198Z 61    = note: move occurs because `mm.1` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8401582Z 
2020-03-05T20:13:57.8401791Z The actual stderr differed from the expected stderr.
2020-03-05T20:13:57.8402725Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-borrow-checks/issue-53114-borrow-checks.stderr
2020-03-05T20:13:57.8402725Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-borrow-checks/issue-53114-borrow-checks.stderr
2020-03-05T20:13:57.8403435Z To update references, rerun the tests and pass the `--bless` flag
2020-03-05T20:13:57.8404069Z To only update this specific test, also pass `--test-args binding/issue-53114-borrow-checks.rs`
2020-03-05T20:13:57.8404547Z error: 1 errors occurred comparing output.
2020-03-05T20:13:57.8404877Z status: exit code: 1
2020-03-05T20:13:57.8404877Z status: exit code: 1
2020-03-05T20:13:57.8406941Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-borrow-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-borrow-checks" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-borrow-checks/auxiliary"
2020-03-05T20:13:57.8408629Z ------------------------------------------
2020-03-05T20:13:57.8408807Z 
2020-03-05T20:13:57.8409176Z ------------------------------------------
2020-03-05T20:13:57.8409401Z stderr:
2020-03-05T20:13:57.8409401Z stderr:
2020-03-05T20:13:57.8409775Z ------------------------------------------
2020-03-05T20:13:57.8410058Z error[E0382]: use of moved value: `m`
2020-03-05T20:13:57.8410629Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:22:11
2020-03-05T20:13:57.8410902Z    |
2020-03-05T20:13:57.8411062Z LL |     let m = M;
2020-03-05T20:13:57.8411611Z    |         - move occurs because `m` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8411930Z LL |     drop(m);
2020-03-05T20:13:57.8412310Z    |          - value moved here
2020-03-05T20:13:57.8412647Z LL |     match m { _ => { } } // #53114: should eventually be accepted too
2020-03-05T20:13:57.8412974Z    |           ^ value used here after move
2020-03-05T20:13:57.8413352Z error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8413915Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:27:11
2020-03-05T20:13:57.8414179Z    |
2020-03-05T20:13:57.8414179Z    |
2020-03-05T20:13:57.8414370Z LL |     match mm { (_x, _) => { } }
2020-03-05T20:13:57.8414815Z    |                 -- value moved here
2020-03-05T20:13:57.8415077Z LL |     match mm { (_, _y) => { } }
2020-03-05T20:13:57.8415352Z    |           ^^ value used here after partial move
2020-03-05T20:13:57.8415579Z    |
2020-03-05T20:13:57.8415874Z    = note: move occurs because `mm.0` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8416340Z error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8416904Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:29:11
2020-03-05T20:13:57.8417168Z    |
2020-03-05T20:13:57.8417168Z    |
2020-03-05T20:13:57.8417354Z LL |     match mm { (_, _y) => { } }
2020-03-05T20:13:57.8417812Z    |                    -- value moved here
2020-03-05T20:13:57.8418059Z LL |     //~^ ERROR [E0382]
2020-03-05T20:13:57.8418285Z LL |     match mm { (_, _) => { } }
2020-03-05T20:13:57.8418572Z    |           ^^ value used here after partial move
2020-03-05T20:13:57.8418785Z    |
2020-03-05T20:13:57.8419077Z    = note: move occurs because `mm.1` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8419562Z error[E0382]: use of moved value: `m`
2020-03-05T20:13:57.8420119Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:36:16
2020-03-05T20:13:57.8420399Z    |
2020-03-05T20:13:57.8420399Z    |
2020-03-05T20:13:57.8420559Z LL |     let m = M;
2020-03-05T20:13:57.8421096Z    |         - move occurs because `m` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8421412Z LL |     drop(m);
2020-03-05T20:13:57.8421801Z    |          - value moved here
2020-03-05T20:13:57.8422106Z LL |     if let _ = m { } // #53114: should eventually be accepted too
2020-03-05T20:13:57.8422492Z    |                ^ value used here after move
2020-03-05T20:13:57.8422899Z error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8423463Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:41:22
2020-03-05T20:13:57.8423745Z    |
2020-03-05T20:13:57.8423745Z    |
2020-03-05T20:13:57.8423928Z LL |     if let (_x, _) = mm { }
2020-03-05T20:13:57.8424403Z    |             -- value moved here
2020-03-05T20:13:57.8424648Z LL |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8424917Z    |                      ^^ value used here after partial move
2020-03-05T20:13:57.8425138Z    |
2020-03-05T20:13:57.8425495Z    = note: move occurs because `mm.0` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8425960Z error[E0382]: use of moved value: `mm`
2020-03-05T20:13:57.8426512Z   --> /checkout/src/test/ui/binding/issue-53114-borrow-checks.rs:43:21
2020-03-05T20:13:57.8426795Z    |
2020-03-05T20:13:57.8426795Z    |
2020-03-05T20:13:57.8426975Z LL |     if let (_, _y) = mm { }
2020-03-05T20:13:57.8427396Z    |                -- value moved here
2020-03-05T20:13:57.8427653Z LL |     //~^ ERROR [E0382]
2020-03-05T20:13:57.8427873Z LL |     if let (_, _) = mm { }
2020-03-05T20:13:57.8428153Z    |                     ^^ value used here after partial move
2020-03-05T20:13:57.8428398Z    |
2020-03-05T20:13:57.8428689Z    = note: move occurs because `mm.1` has type `M`, which does not implement the `Copy` trait
2020-03-05T20:13:57.8429142Z error: aborting due to 6 previous errors
2020-03-05T20:13:57.8429331Z 
2020-03-05T20:13:57.8429776Z For more information about this error, try `rustc --explain E0382`.
2020-03-05T20:13:57.8430000Z 
---
2020-03-05T20:13:57.8456788Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-05T20:13:57.8457216Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-05T20:13:57.8466589Z 
2020-03-05T20:13:57.8467660Z 
2020-03-05T20:13:57.8476075Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-05T20:13:57.8478795Z 
2020-03-05T20:13:57.8478896Z 
2020-03-05T20:13:57.8493357Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-05T20:13:57.8493711Z Build completed unsuccessfully in 0:59:43
2020-03-05T20:13:57.8493711Z Build completed unsuccessfully in 0:59:43
2020-03-05T20:13:57.8547090Z == clock drift check ==
2020-03-05T20:13:57.8569870Z   local time: Thu Mar  5 20:13:57 UTC 2020
2020-03-05T20:13:58.1533721Z   network time: Thu, 05 Mar 2020 20:13:58 GMT
2020-03-05T20:13:58.1540266Z == end clock drift check ==
2020-03-05T20:13:58.7682403Z 
2020-03-05T20:13:58.7746102Z ##[error]Bash exited with code '1'.
2020-03-05T20:13:58.7759805Z ##[section]Finishing: Run build
2020-03-05T20:13:58.7813229Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69573/merge to s
2020-03-05T20:13:58.7818348Z Task         : Get sources
2020-03-05T20:13:58.7818691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T20:13:58.7819025Z Version      : 1.0.0
2020-03-05T20:13:58.7819250Z Author       : Microsoft
2020-03-05T20:13:58.7819250Z Author       : Microsoft
2020-03-05T20:13:58.7819607Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-05T20:13:58.7820025Z ==============================================================================
2020-03-05T20:13:59.1138307Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-05T20:13:59.1181243Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69573/merge to s
2020-03-05T20:13:59.1268016Z Cleaning up task key
2020-03-05T20:13:59.1269292Z Start cleaning up orphan processes.
2020-03-05T20:13:59.1443047Z Terminate orphan process: pid (4152) (python)
2020-03-05T20:13:59.1676081Z ##[section]Finishing: Finalize Job
