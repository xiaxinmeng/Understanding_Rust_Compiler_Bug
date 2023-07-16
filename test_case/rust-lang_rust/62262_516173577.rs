plain
2019-07-29T20:18:22.4036312Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T20:18:22.4222682Z ##[command]git config gc.auto 0
2019-07-29T20:18:22.4294318Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T20:18:22.4378262Z ##[command]git config --get-all http.proxy
2019-07-29T20:18:22.4483939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62262/merge:refs/remotes/pull/62262/merge
---
2019-07-29T20:18:56.9483347Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T20:18:56.9483681Z 
2019-07-29T20:18:56.9484267Z   git checkout -b <new-branch-name>
2019-07-29T20:18:56.9484663Z 
2019-07-29T20:18:56.9484980Z HEAD is now at 9747c2722 Merge ec5180415e31fa717eacbe0332a0eea3ab96ca18 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T20:18:56.9725698Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T20:18:56.9728528Z ==============================================================================
2019-07-29T20:18:56.9728598Z Task         : Bash
2019-07-29T20:18:56.9728646Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T21:21:21.5872903Z .................................................................................................... 1400/8805
2019-07-29T21:21:27.5370511Z .................................................................................................... 1500/8805
2019-07-29T21:21:40.3771280Z ................................................................i...............i................... 1600/8805
2019-07-29T21:21:48.4108607Z .................................................................................................... 1700/8805
2019-07-29T21:22:03.9721469Z ..................................................iiiii............................................. 1800/8805
2019-07-29T21:22:15.7315812Z .................................................................................................... 2000/8805
2019-07-29T21:22:18.4287512Z .................................................................................................... 2100/8805
2019-07-29T21:22:22.6103008Z .................................................................................................... 2200/8805
2019-07-29T21:22:29.6555911Z .................................................................................................... 2300/8805
---
2019-07-29T21:26:18.3325567Z .................................................................................................... 5200/8805
2019-07-29T21:26:29.1957156Z .................................................................................................... 5300/8805
2019-07-29T21:26:36.8008965Z ...i................................................................................................ 5400/8805
2019-07-29T21:26:42.2402803Z .................................................................................................... 5500/8805
2019-07-29T21:26:54.8587789Z .................................................................................................ii. 5600/8805
2019-07-29T21:27:09.8982822Z ..i..ii...........i................................................................................. 5700/8805
2019-07-29T21:27:24.6168243Z .................................................................................................... 5900/8805
2019-07-29T21:27:29.4971113Z .................................................................................................i.. 6000/8805
2019-07-29T21:27:44.0860731Z ii.................................................................................................. 6100/8805
2019-07-29T21:28:00.6629754Z .................................................................................................... 6200/8805
---
2019-07-29T21:32:59.1562772Z  finished in 23.391
2019-07-29T21:32:59.1741699Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:32:59.3626850Z 
2019-07-29T21:32:59.3627678Z running 146 tests
2019-07-29T21:33:02.7082567Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-29T21:33:04.5843467Z iii..............i.........iii.i......ii......
2019-07-29T21:33:04.5848209Z 
2019-07-29T21:33:04.5853579Z  finished in 5.411
2019-07-29T21:33:04.6039682Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:33:04.7628304Z 
---
2019-07-29T21:33:06.8131249Z  finished in 2.208
2019-07-29T21:33:06.8316893Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:33:06.9892380Z 
2019-07-29T21:33:06.9892747Z running 9 tests
2019-07-29T21:33:06.9893775Z iiiiiiiii
2019-07-29T21:33:06.9894192Z 
2019-07-29T21:33:06.9894250Z  finished in 0.157
2019-07-29T21:33:07.0090072Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:33:07.1711997Z 
---
2019-07-29T21:33:25.7987079Z  finished in 18.788
2019-07-29T21:33:25.8176784Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:33:25.9863045Z 
2019-07-29T21:33:25.9863843Z running 122 tests
2019-07-29T21:33:50.5940493Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-07-29T21:33:55.3756879Z .i.i......iii.i.....ii
2019-07-29T21:33:55.3757371Z 
2019-07-29T21:33:55.3757447Z  finished in 29.557
2019-07-29T21:33:55.3767654Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T21:33:55.3768021Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-07-29T21:41:35.0326622Z ---- [pretty] pretty/block-disambig.rs stdout ----
2019-07-29T21:41:35.0326915Z 
2019-07-29T21:41:35.0328148Z error: pretty-printed source does not typecheck
2019-07-29T21:41:35.0328593Z status: exit code: 1
2019-07-29T21:41:35.0330170Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-codegen" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/block-disambig/block-disambig.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/block-disambig/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib"
2019-07-29T21:41:35.0331275Z ------------------------------------------
2019-07-29T21:41:35.0332409Z 
2019-07-29T21:41:35.0332940Z ------------------------------------------
2019-07-29T21:41:35.0333134Z stderr:
2019-07-29T21:41:35.0333134Z stderr:
2019-07-29T21:41:35.0333491Z ------------------------------------------
2019-07-29T21:41:35.0333688Z warning: function is never used: `test1`
2019-07-29T21:41:35.0334419Z   --> <anon>:10:1
2019-07-29T21:41:35.0334631Z    |
2019-07-29T21:41:35.0334788Z 10 | fn test1() { let val = &0; { } *val; }
2019-07-29T21:41:35.0336179Z    |
2019-07-29T21:41:35.0336320Z    = note: `#[warn(dead_code)]` on by default
2019-07-29T21:41:35.0336428Z 
2019-07-29T21:41:35.0336547Z warning: function is never used: `test2`
2019-07-29T21:41:35.0336547Z warning: function is never used: `test2`
2019-07-29T21:41:35.0336925Z   --> <anon>:12:1
2019-07-29T21:41:35.0337092Z    |
2019-07-29T21:41:35.0337416Z 12 | fn test2() -> isize { let val = &0; { } *val }
2019-07-29T21:41:35.0337699Z 
2019-07-29T21:41:35.0337699Z 
2019-07-29T21:41:35.0337817Z warning: field is never used: `eax`
2019-07-29T21:41:35.0338123Z   --> <anon>:16:5
2019-07-29T21:41:35.0338279Z    |
2019-07-29T21:41:35.0338393Z 16 |     eax: isize,
2019-07-29T21:41:35.0338624Z 
2019-07-29T21:41:35.0338741Z warning: function is never used: `test3`
2019-07-29T21:41:35.0338741Z warning: function is never used: `test3`
2019-07-29T21:41:35.0339028Z   --> <anon>:19:1
2019-07-29T21:41:35.0339341Z 19 | fn test3() {
2019-07-29T21:41:35.0339452Z    | ^^^^^^^^^^
2019-07-29T21:41:35.0339572Z 
2019-07-29T21:41:35.0339705Z warning: function is never used: `test4`
2019-07-29T21:41:35.0339705Z warning: function is never used: `test4`
2019-07-29T21:41:35.0339996Z   --> <anon>:25:1
2019-07-29T21:41:35.0340166Z    |
2019-07-29T21:41:35.0340505Z 25 | fn test4() -> bool { let regs = &true; if true { } *regs || false }
2019-07-29T21:41:35.0340799Z 
2019-07-29T21:41:35.0340915Z warning: function is never used: `test5`
2019-07-29T21:41:35.0340915Z warning: function is never used: `test5`
2019-07-29T21:41:35.0341204Z   --> <anon>:27:1
2019-07-29T21:41:35.0341373Z    |
2019-07-29T21:41:35.0342032Z 27 | fn test5() -> (isize, isize) { { } (0, 1) }
2019-07-29T21:41:35.0342871Z 
2019-07-29T21:41:35.0343003Z warning: function is never used: `test6`
2019-07-29T21:41:35.0343003Z warning: function is never used: `test6`
2019-07-29T21:41:35.0343392Z   --> <anon>:29:1
2019-07-29T21:41:35.0343589Z    |
2019-07-29T21:41:35.0343968Z 29 | fn test6() -> bool { { } (true || false) && true }
2019-07-29T21:41:35.0344293Z 
2019-07-29T21:41:35.0344425Z warning: function is never used: `test7`
2019-07-29T21:41:35.0344425Z warning: function is never used: `test7`
2019-07-29T21:41:35.0344752Z   --> <anon>:31:1
2019-07-29T21:41:35.0345380Z 31 | fn test7() -> usize {
2019-07-29T21:41:35.0345533Z    | ^^^^^^^^^^^^^^^^^^^
2019-07-29T21:41:35.0345653Z 
2019-07-29T21:41:35.0345768Z warning: function is never used: `test8`
2019-07-29T21:41:35.0345768Z warning: function is never used: `test8`
2019-07-29T21:41:35.0346052Z   --> <anon>:37:1
2019-07-29T21:41:35.0346220Z    |
2019-07-29T21:41:35.0346511Z 37 | fn test8() -> isize {
2019-07-29T21:41:35.0346902Z 
2019-07-29T21:41:35.0347075Z warning: function is never used: `test9`
2019-07-29T21:41:35.0347075Z warning: function is never used: `test9`
2019-07-29T21:41:35.0347423Z   --> <anon>:43:1
2019-07-29T21:41:35.0348272Z 43 | fn test9() {
2019-07-29T21:41:35.0348311Z    | ^^^^^^^^^^
2019-07-29T21:41:35.0348360Z 
2019-07-29T21:41:35.0348432Z warning: function is never used: `test10`
2019-07-29T21:41:35.0348432Z warning: function is never used: `test10`
2019-07-29T21:41:35.0348719Z   --> <anon>:49:1
2019-07-29T21:41:35.0348758Z    |
2019-07-29T21:41:35.0349147Z 49 | fn test10() -> isize {
2019-07-29T21:41:35.0349680Z 
2019-07-29T21:41:35.0349730Z warning: function is never used: `test11`
2019-07-29T21:41:35.0349730Z warning: function is never used: `test11`
2019-07-29T21:41:35.0349969Z   --> <anon>:55:1
2019-07-29T21:41:35.0350022Z    |
2019-07-29T21:41:35.0350229Z 55 | fn test11() -> Vec<isize> { if true { } vec![1 , 2] }
2019-07-29T21:41:35.0350298Z 
2019-07-29T21:41:35.0350350Z error: unused unary operation that must be used
2019-07-29T21:41:35.0350350Z error: unused unary operation that must be used
2019-07-29T21:41:35.0350520Z   --> <anon>:10:32
2019-07-29T21:41:35.0350559Z    |
2019-07-29T21:41:35.0350611Z 10 | fn test1() { let val = &0; { } *val; }
2019-07-29T21:41:35.0350689Z    |
2019-07-29T21:41:35.0350689Z    |
2019-07-29T21:41:35.0350728Z    = note: `#[deny(unused_must_use)]` on by default
2019-07-29T21:41:35.0350992Z error: aborting due to previous error
2019-07-29T21:41:35.0351022Z 
2019-07-29T21:41:35.0351044Z 
2019-07-29T21:41:35.0351285Z ------------------------------------------
2019-07-29T21:41:35.0351285Z ------------------------------------------
2019-07-29T21:41:35.0351314Z 
2019-07-29T21:41:35.0351338Z 
2019-07-29T21:41:35.0351534Z ---- [pretty] pretty/unary-op-disambig.rs stdout ----
2019-07-29T21:41:35.0351561Z 
2019-07-29T21:41:35.0352220Z error: pretty-printed source does not typecheck
2019-07-29T21:41:35.0352279Z status: exit code: 1
2019-07-29T21:41:35.0353053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-codegen" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/unary-op-disambig/unary-op-disambig.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/unary-op-disambig/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib"
2019-07-29T21:41:35.0353392Z ------------------------------------------
2019-07-29T21:41:35.0353446Z 
2019-07-29T21:41:35.0353668Z ------------------------------------------
2019-07-29T21:41:35.0353713Z stderr:
2019-07-29T21:41:35.0353713Z stderr:
2019-07-29T21:41:35.0353936Z ------------------------------------------
2019-07-29T21:41:35.0353984Z warning: function is never used: `f`
2019-07-29T21:41:35.0354173Z  --> <anon>:5:1
2019-07-29T21:41:35.0354275Z 5 | fn f() { }
2019-07-29T21:41:35.0354318Z   | ^^^^^^
2019-07-29T21:41:35.0354357Z   |
2019-07-29T21:41:35.0354417Z   = note: `#[warn(dead_code)]` on by default
2019-07-29T21:41:35.0354417Z   = note: `#[warn(dead_code)]` on by default
2019-07-29T21:41:35.0354447Z 
2019-07-29T21:41:35.0354491Z warning: function is never used: `block_semi`
2019-07-29T21:41:35.0354683Z  --> <anon>:7:1
2019-07-29T21:41:35.0354740Z   |
2019-07-29T21:41:35.0354954Z 7 | fn block_semi() -> isize { { f() }; -1 }
2019-07-29T21:41:35.0355046Z 
2019-07-29T21:41:35.0355046Z 
2019-07-29T21:41:35.0355275Z warning: function is never used: `block_nosemi`
2019-07-29T21:41:35.0355554Z  --> <anon>:9:1
2019-07-29T21:41:35.0355592Z   |
2019-07-29T21:41:35.0355796Z 9 | fn block_nosemi() -> isize { ({ 0 }) - 1 }
2019-07-29T21:41:35.0355862Z 
2019-07-29T21:41:35.0355862Z 
2019-07-29T21:41:35.0355900Z warning: function is never used: `if_semi`
2019-07-29T21:41:35.0356080Z   --> <anon>:11:1
2019-07-29T21:41:35.0356118Z    |
2019-07-29T21:41:35.0356321Z 11 | fn if_semi() -> isize { if true { f() } else { f() }; -1 }
2019-07-29T21:41:35.0356409Z 
2019-07-29T21:41:35.0356409Z 
2019-07-29T21:41:35.0356447Z warning: function is never used: `if_nosemi`
2019-07-29T21:41:35.0356630Z   --> <anon>:13:1
2019-07-29T21:41:35.0356670Z    |
2019-07-29T21:41:35.0356874Z 13 | fn if_nosemi() -> isize { (if true { 0 } else { 0 }) - 1 }
2019-07-29T21:41:35.0356964Z 
2019-07-29T21:41:35.0356964Z 
2019-07-29T21:41:35.0357009Z warning: function is never used: `alt_semi`
2019-07-29T21:41:35.0357179Z   --> <anon>:15:1
2019-07-29T21:41:35.0357215Z    |
2019-07-29T21:41:35.0357445Z 15 | fn alt_semi() -> isize { match true { true => { f() } _ => { } }; -1 }
2019-07-29T21:41:35.0357514Z 
2019-07-29T21:41:35.0357514Z 
2019-07-29T21:41:35.0357566Z warning: function is never used: `alt_no_semi`
2019-07-29T21:41:35.0357732Z   --> <anon>:17:1
2019-07-29T21:41:35.0357769Z    |
2019-07-29T21:41:35.0357989Z 17 | fn alt_no_semi() -> isize { (match true { true => { 0 } _ => { 1 } }) - 1 }
2019-07-29T21:41:35.0358071Z 
2019-07-29T21:41:35.0358071Z 
2019-07-29T21:41:35.0358109Z warning: function is never used: `stmt`
2019-07-29T21:41:35.0358290Z   --> <anon>:19:1
2019-07-29T21:41:35.0358327Z    |
2019-07-29T21:41:35.0358505Z 19 | fn stmt() { { f() }; -1; }
2019-07-29T21:41:35.0358581Z 
2019-07-29T21:41:35.0358711Z error: unused unary operation that must be used
2019-07-29T21:41:35.0358711Z error: unused unary operation that must be used
2019-07-29T21:41:35.0358964Z   --> <anon>:19:22
2019-07-29T21:41:35.0359020Z    |
2019-07-29T21:41:35.0359197Z 19 | fn stmt() { { f() }; -1; }
2019-07-29T21:41:35.0359273Z    |
2019-07-29T21:41:35.0359273Z    |
2019-07-29T21:41:35.0359326Z    = note: `#[deny(unused_must_use)]` on by default
2019-07-29T21:41:35.0359390Z error: aborting due to previous error
2019-07-29T21:41:35.0359415Z 
2019-07-29T21:41:35.0359451Z 
2019-07-29T21:41:35.0359637Z ------------------------------------------
---
2019-07-29T21:41:35.0361337Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T21:41:35.0361522Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T21:41:35.0369348Z 
2019-07-29T21:41:35.0369411Z 
2019-07-29T21:41:35.0371111Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T21:41:35.0371574Z 
2019-07-29T21:41:35.0371620Z 
2019-07-29T21:41:35.0375924Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T21:41:35.0376006Z Build completed unsuccessfully in 1:16:15
2019-07-29T21:41:35.0376006Z Build completed unsuccessfully in 1:16:15
2019-07-29T21:41:37.4348457Z ##[error]Bash exited with code '1'.
2019-07-29T21:41:37.4383808Z ##[section]Starting: Checkout
2019-07-29T21:41:37.4386276Z ==============================================================================
2019-07-29T21:41:37.4386336Z Task         : Get sources
2019-07-29T21:41:37.4386404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
