plain
2019-11-12T19:29:08.5607818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T19:29:08.5822266Z ##[command]git config gc.auto 0
2019-11-12T19:29:08.5905195Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T19:29:08.5989444Z ##[command]git config --get-all http.proxy
2019-11-12T19:29:08.6157055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64588/merge:refs/remotes/pull/64588/merge
---
2019-11-12T20:30:07.5603374Z .................................................................................................... 1500/9246
2019-11-12T20:30:13.9769718Z .................................................................................................... 1600/9246
2019-11-12T20:30:22.3561725Z .................................................................................................... 1700/9246
2019-11-12T20:30:32.1651472Z ...........i........................................................................................ 1800/9246
2019-11-12T20:30:38.9688168Z ...............................................................................................iiiii 1900/9246
2019-11-12T20:31:01.0740391Z .................................................................................................... 2100/9246
2019-11-12T20:31:03.5636340Z .................................................................................................... 2200/9246
2019-11-12T20:31:06.1485556Z .................................................................................................... 2300/9246
2019-11-12T20:31:12.3510498Z .................................................................................................... 2400/9246
---
2019-11-12T20:34:14.2188147Z ............................................................................................i....... 4700/9246
2019-11-12T20:34:21.0458436Z ........i........................................................................................... 4800/9246
2019-11-12T20:34:30.7820414Z .................................................................................................... 4900/9246
2019-11-12T20:34:36.1865361Z .................................................................................................... 5000/9246
2019-11-12T20:34:46.9313702Z ...............................................................................................ii.ii 5100/9246
2019-11-12T20:34:56.3864363Z ..............................i..................................................................... 5300/9246
2019-11-12T20:35:03.0267586Z .................................................................................................... 5400/9246
2019-11-12T20:35:13.8337094Z .............................................................................i...................... 5500/9246
2019-11-12T20:35:21.7714872Z .................................................................................................... 5600/9246
2019-11-12T20:35:21.7714872Z .................................................................................................... 5600/9246
2019-11-12T20:35:28.6521999Z .................................................................................................... 5700/9246
2019-11-12T20:35:38.8771441Z ..............................................................ii...i..ii...........i................ 5800/9246
2019-11-12T20:36:01.9798726Z .................................................................................................... 6000/9246
2019-11-12T20:36:10.1337189Z .................................................................................................... 6100/9246
2019-11-12T20:36:10.1337189Z .................................................................................................... 6100/9246
2019-11-12T20:36:14.4157121Z .................................................................................i..ii.............. 6200/9246
2019-11-12T20:36:42.4545135Z .................................................................................................... 6400/9246
2019-11-12T20:36:47.3424906Z ..................................................i................................................. 6500/9246
2019-11-12T20:36:49.6351216Z .................................................................................................... 6600/9246
2019-11-12T20:36:52.1057219Z ..................................i................................................................. 6700/9246
---
2019-11-12T20:42:14.7450298Z  finished in 5.938
2019-11-12T20:42:14.7677010Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:42:14.9532602Z 
2019-11-12T20:42:14.9532835Z running 156 tests
2019-11-12T20:42:18.2975114Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-12T20:42:19.9454412Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-12T20:42:19.9455817Z 
2019-11-12T20:42:19.9461141Z  finished in 5.178
2019-11-12T20:42:19.9650706Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:42:20.1242032Z 
---
2019-11-12T20:42:22.0727033Z  finished in 2.107
2019-11-12T20:42:22.0921932Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:42:22.2532313Z 
2019-11-12T20:42:22.2533029Z running 9 tests
2019-11-12T20:42:22.2533860Z iiiiiiiii
2019-11-12T20:42:22.2534254Z 
2019-11-12T20:42:22.2534495Z  finished in 0.161
2019-11-12T20:42:22.2732301Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:42:22.4498399Z 
---
2019-11-12T20:42:41.9794990Z  finished in 19.706
2019-11-12T20:42:42.0023646Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:42:42.1715137Z 
2019-11-12T20:42:42.1715371Z running 123 tests
2019-11-12T20:43:06.8780144Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-12T20:43:11.7596233Z i.i.i......iii.i.....ii
2019-11-12T20:43:11.7597873Z 
2019-11-12T20:43:11.7600286Z  finished in 29.757
2019-11-12T20:43:11.7609542Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T20:43:11.7611780Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-12T20:50:02.3511916Z running 56 tests
2019-11-12T20:50:04.9441381Z ............................................F...........
2019-11-12T20:50:04.9441773Z failures:
2019-11-12T20:50:04.9441830Z 
2019-11-12T20:50:04.9442455Z ---- [pretty] pretty/raw-address-of.rs stdout ----
2019-11-12T20:50:04.9442727Z error: pretty-printed source does not typecheck
2019-11-12T20:50:04.9442942Z status: exit code: 1
2019-11-12T20:50:04.9442942Z status: exit code: 1
2019-11-12T20:50:04.9443708Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-codegen" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/raw-address-of/raw-address-of.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/raw-address-of/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
2019-11-12T20:50:04.9444056Z ------------------------------------------
2019-11-12T20:50:04.9444089Z 
2019-11-12T20:50:04.9444315Z ------------------------------------------
2019-11-12T20:50:04.9444358Z stderr:
2019-11-12T20:50:04.9444358Z stderr:
2019-11-12T20:50:04.9444560Z ------------------------------------------
2019-11-12T20:50:04.9444623Z error[E0744]: cannot take address of a temporary
2019-11-12T20:50:04.9444810Z  --> <anon>:4:35
2019-11-12T20:50:04.9444851Z   |
2019-11-12T20:50:04.9444891Z 4 | const PTR: *const () = &raw const ();
2019-11-12T20:50:04.9444952Z   |                                   ^^ temporary value
2019-11-12T20:50:04.9445218Z error[E0744]: cannot take address of a temporary
2019-11-12T20:50:04.9445218Z error[E0744]: cannot take address of a temporary
2019-11-12T20:50:04.9445473Z  --> <anon>:5:35
2019-11-12T20:50:04.9445513Z   |
2019-11-12T20:50:04.9445553Z 5 | static MUT_PTR: () = { &raw const (); };
2019-11-12T20:50:04.9445614Z   |                                   ^^ temporary value
2019-11-12T20:50:04.9445787Z error[E0744]: cannot take address of a temporary
2019-11-12T20:50:04.9445787Z error[E0744]: cannot take address of a temporary
2019-11-12T20:50:04.9446017Z   --> <anon>:11:38
2019-11-12T20:50:04.9446077Z    |
2019-11-12T20:50:04.9446119Z 11 |     let parens = unsafe { *(&raw mut (x * y)) };
2019-11-12T20:50:04.9446165Z    |                                      ^^^^^^^ temporary value
2019-11-12T20:50:04.9446250Z error: aborting due to 3 previous errors
2019-11-12T20:50:04.9446276Z 
2019-11-12T20:50:04.9446509Z For more information about this error, try `rustc --explain E0744`.
2019-11-12T20:50:04.9446541Z 
2019-11-12T20:50:04.9446541Z 
2019-11-12T20:50:04.9446773Z ------------------------------------------
2019-11-12T20:50:04.9446918Z 
2019-11-12T20:50:04.9446942Z 
2019-11-12T20:50:04.9446965Z 
2019-11-12T20:50:04.9447022Z failures:
2019-11-12T20:50:04.9447740Z     [pretty] pretty/raw-address-of.rs
2019-11-12T20:50:04.9448320Z test result: FAILED. 55 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-12T20:50:04.9448391Z 
2019-11-12T20:50:04.9450810Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T20:50:04.9452439Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T20:50:04.9452439Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T20:50:04.9452545Z 
2019-11-12T20:50:04.9454082Z 
2019-11-12T20:50:04.9456290Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T20:50:04.9456575Z 
2019-11-12T20:50:04.9456625Z 
2019-11-12T20:50:04.9464233Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T20:50:04.9465042Z Build completed unsuccessfully in 1:14:18
2019-11-12T20:50:04.9465042Z Build completed unsuccessfully in 1:14:18
2019-11-12T20:50:04.9516062Z == clock drift check ==
2019-11-12T20:50:04.9532277Z   local time: Tue Nov 12 20:50:04 UTC 2019
2019-11-12T20:50:05.1191499Z   network time: Tue, 12 Nov 2019 20:50:05 GMT
2019-11-12T20:50:05.1197772Z == end clock drift check ==
2019-11-12T20:50:07.5819581Z 
2019-11-12T20:50:07.5963458Z ##[error]Bash exited with code '1'.
2019-11-12T20:50:07.6004973Z ##[section]Starting: Checkout
2019-11-12T20:50:07.6006956Z ==============================================================================
2019-11-12T20:50:07.6007016Z Task         : Get sources
2019-11-12T20:50:07.6007068Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
