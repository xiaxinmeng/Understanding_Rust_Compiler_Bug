plain
2019-08-23T06:04:32.3040595Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-23T06:04:32.3252881Z ##[command]git config gc.auto 0
2019-08-23T06:04:32.3321055Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-23T06:04:32.3389493Z ##[command]git config --get-all http.proxy
2019-08-23T06:04:32.3540858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63827/merge:refs/remotes/pull/63827/merge
---
2019-08-23T06:05:07.1115575Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-23T06:05:07.1115610Z 
2019-08-23T06:05:07.1115833Z   git checkout -b <new-branch-name>
2019-08-23T06:05:07.1115867Z 
2019-08-23T06:05:07.1115939Z HEAD is now at 6014e0a03 Merge 2661f79224445d71ef5b061666c94680717305fd into a71e32e4078f3a8c1ebc2a731e23ac3da3ef73a1
2019-08-23T06:05:07.1279943Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-23T06:05:07.1283163Z ==============================================================================
2019-08-23T06:05:07.1283224Z Task         : Bash
2019-08-23T06:05:07.1283288Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-23T07:09:41.9278714Z .................................................................................................... 1500/8950
2019-08-23T07:09:47.6208164Z .................................................................................................... 1600/8950
2019-08-23T07:10:00.7473982Z ...........................................i...............i........................................ 1700/8950
2019-08-23T07:10:09.2025978Z .................................................................................................... 1800/8950
2019-08-23T07:10:23.9726087Z ...................................iiiii............................................................ 1900/8950
2019-08-23T07:10:35.0854332Z .................................................................................................... 2100/8950
2019-08-23T07:10:37.8067092Z .................................................................................................... 2200/8950
2019-08-23T07:10:42.4310835Z .................................................................................................... 2300/8950
2019-08-23T07:10:49.7666618Z .................................................................................................... 2400/8950
---
2019-08-23T07:13:55.5503518Z .......................i...............i............................................................ 4700/8950
2019-08-23T07:14:07.8904942Z .................................................................................................... 4800/8950
2019-08-23T07:14:14.1888414Z .................................................................................................... 4900/8950
2019-08-23T07:14:25.5593296Z .................................................................................................... 5000/8950
2019-08-23T07:14:30.9913716Z ....ii.ii........................................................................................... 5100/8950
2019-08-23T07:14:46.2659182Z .................................................................................................... 5300/8950
2019-08-23T07:14:53.2386411Z ............................................................i....................................... 5400/8950
2019-08-23T07:15:00.4741260Z .................................................................................................... 5500/8950
2019-08-23T07:15:08.5937270Z .................................................................................................... 5600/8950
2019-08-23T07:15:08.5937270Z .................................................................................................... 5600/8950
2019-08-23T07:15:19.7074469Z ......................................................ii...i..ii...........i........................ 5700/8950
2019-08-23T07:15:42.1435058Z .................................................................................................... 5900/8950
2019-08-23T07:15:47.2240065Z .................................................................................................... 6000/8950
2019-08-23T07:15:47.2240065Z .................................................................................................... 6000/8950
2019-08-23T07:15:54.7617116Z .......................................................i..ii........................................ 6100/8950
2019-08-23T07:16:22.6667604Z .................................................................................................... 6300/8950
2019-08-23T07:16:24.9428998Z .i.................................................................................................. 6400/8950
2019-08-23T07:16:27.2640674Z .........................................................................i.......................... 6500/8950
2019-08-23T07:16:30.3354334Z .................................................................................................... 6600/8950
---
2019-08-23T07:21:19.1423398Z  finished in 21.475
2019-08-23T07:21:19.1608419Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:21:19.3322579Z 
2019-08-23T07:21:19.3323393Z running 149 tests
2019-08-23T07:21:22.7459900Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-08-23T07:21:24.7518673Z ..iiii..............i.........iii.i......ii......
2019-08-23T07:21:24.7520047Z 
2019-08-23T07:21:24.7523545Z  finished in 5.591
2019-08-23T07:21:24.7724635Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:21:24.9370181Z 
---
2019-08-23T07:21:27.0857021Z  finished in 2.313
2019-08-23T07:21:27.1043867Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:21:27.2786449Z 
2019-08-23T07:21:27.2786794Z running 9 tests
2019-08-23T07:21:27.2787673Z iiiiiiiii
2019-08-23T07:21:27.2788161Z 
2019-08-23T07:21:27.2790864Z  finished in 0.174
2019-08-23T07:21:27.3009683Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:21:27.4688267Z 
---
2019-08-23T07:21:45.8897229Z  finished in 18.589
2019-08-23T07:21:45.9089443Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:21:46.0742757Z 
2019-08-23T07:21:46.0743114Z running 122 tests
2019-08-23T07:22:11.3055961Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-23T07:22:16.1356428Z .i.i......iii.i.....ii
2019-08-23T07:22:16.1357150Z 
2019-08-23T07:22:16.1361707Z  finished in 30.227
2019-08-23T07:22:16.1370231Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-23T07:22:16.1370577Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-23T07:29:51.7908207Z 
2019-08-23T07:29:51.7908282Z failures:
2019-08-23T07:29:51.7908335Z 
2019-08-23T07:29:51.7908643Z ---- /checkout/src/test/rustdoc/async-move-doctest.rs -  (line 8) stdout ----
2019-08-23T07:29:51.7909219Z thread '/checkout/src/test/rustdoc/async-move-doctest.rs -  (line 8)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputw5Ridd.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7909349Z 
2019-08-23T07:29:51.7909394Z 
2019-08-23T07:29:51.7909439Z failures:
2019-08-23T07:29:51.7909702Z     /checkout/src/test/rustdoc/async-move-doctest.rs -  (line 8)
---
2019-08-23T07:29:51.7913029Z 
2019-08-23T07:29:51.7913092Z failures:
2019-08-23T07:29:51.7913121Z 
2019-08-23T07:29:51.7913398Z ---- /checkout/src/test/rustdoc/comment-in-doctest.rs -  (line 10) stdout ----
2019-08-23T07:29:51.7913947Z thread '/checkout/src/test/rustdoc/comment-in-doctest.rs -  (line 10)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputXcxC49.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7914088Z 
2019-08-23T07:29:51.7914118Z 
2019-08-23T07:29:51.7914178Z failures:
2019-08-23T07:29:51.7914453Z     /checkout/src/test/rustdoc/comment-in-doctest.rs -  (line 10)
---
2019-08-23T07:29:51.7915769Z ---- [rustdoc] rustdoc/doctest-manual-crate-name.rs stdout ----
2019-08-23T07:29:51.7915806Z 
2019-08-23T07:29:51.7915850Z error: rustdoc failed!
2019-08-23T07:29:51.7915916Z status: exit code: 101
2019-08-23T07:29:51.7916575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name" "/checkout/src/test/rustdoc/doctest-manual-crate-name.rs" "--test"
2019-08-23T07:29:51.7916926Z ------------------------------------------
2019-08-23T07:29:51.7916963Z 
2019-08-23T07:29:51.7917025Z running 1 test
2019-08-23T07:29:51.7917309Z test /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) ... FAILED
2019-08-23T07:29:51.7917309Z test /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) ... FAILED
2019-08-23T07:29:51.7917350Z 
2019-08-23T07:29:51.7917396Z failures:
2019-08-23T07:29:51.7917441Z 
2019-08-23T07:29:51.7918092Z ---- /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) stdout ----
2019-08-23T07:29:51.7918796Z thread '/checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputMIy9X6.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7918965Z 
2019-08-23T07:29:51.7918997Z 
2019-08-23T07:29:51.7919058Z failures:
2019-08-23T07:29:51.7919369Z     /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3)
---
2019-08-23T07:29:51.7922615Z test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3) ... FAILED
2019-08-23T07:29:51.7922656Z 
2019-08-23T07:29:51.7922735Z failures:
2019-08-23T07:29:51.7922765Z 
2019-08-23T07:29:51.7923042Z ---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22) stdout ----
2019-08-23T07:29:51.7923587Z thread '/checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputWKSkw4.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7923738Z 
2019-08-23T07:29:51.7924030Z ---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3) stdout ----
2019-08-23T07:29:51.7924030Z ---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3) stdout ----
2019-08-23T07:29:51.7924582Z thread '/checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputPM2hEs.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7924700Z 
2019-08-23T07:29:51.7924743Z failures:
2019-08-23T07:29:51.7925009Z     /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22)
2019-08-23T07:29:51.7925275Z     /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3)
---
2019-08-23T07:29:51.7926564Z ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
2019-08-23T07:29:51.7926630Z 
2019-08-23T07:29:51.7926765Z error: rustdoc failed!
2019-08-23T07:29:51.7926818Z status: exit code: 101
2019-08-23T07:29:51.7929174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
2019-08-23T07:29:51.7929562Z ------------------------------------------
2019-08-23T07:29:51.7929618Z 
2019-08-23T07:29:51.7929668Z running 1 test
2019-08-23T07:29:51.7929943Z test /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) ... FAILED
2019-08-23T07:29:51.7929943Z test /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) ... FAILED
2019-08-23T07:29:51.7930002Z 
2019-08-23T07:29:51.7930048Z failures:
2019-08-23T07:29:51.7930077Z 
2019-08-23T07:29:51.7930360Z ---- /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) stdout ----
2019-08-23T07:29:51.7931117Z thread '/checkout/src/test/rustdoc/edition-flag.rs - main (line 4)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputC1ybz5.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7931262Z 
2019-08-23T07:29:51.7931290Z 
2019-08-23T07:29:51.7931333Z failures:
2019-08-23T07:29:51.7931593Z     /checkout/src/test/rustdoc/edition-flag.rs - main (line 4)
---
2019-08-23T07:29:51.7965583Z ---- [rustdoc] rustdoc/issue-18199.rs stdout ----
2019-08-23T07:29:51.7965621Z 
2019-08-23T07:29:51.7965674Z error: rustdoc failed!
2019-08-23T07:29:51.7965722Z status: exit code: 101
2019-08-23T07:29:51.7966352Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
2019-08-23T07:29:51.7966688Z ------------------------------------------
2019-08-23T07:29:51.7966733Z 
2019-08-23T07:29:51.7966797Z running 1 test
2019-08-23T07:29:51.7967086Z test /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) ... FAILED
2019-08-23T07:29:51.7967086Z test /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) ... FAILED
2019-08-23T07:29:51.7967128Z 
2019-08-23T07:29:51.7967185Z failures:
2019-08-23T07:29:51.7967214Z 
2019-08-23T07:29:51.7967489Z ---- /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) stdout ----
2019-08-23T07:29:51.7968508Z thread '/checkout/src/test/rustdoc/issue-18199.rs - foo (line 5)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInput9pPiud.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7968823Z 
2019-08-23T07:29:51.7968868Z 
2019-08-23T07:29:51.7968914Z failures:
2019-08-23T07:29:51.7969229Z     /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5)
---
2019-08-23T07:29:51.7970751Z ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
2019-08-23T07:29:51.7970788Z 
2019-08-23T07:29:51.7970845Z error: rustdoc failed!
2019-08-23T07:29:51.7970892Z status: exit code: 101
2019-08-23T07:29:51.7971523Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
2019-08-23T07:29:51.7972853Z ------------------------------------------
2019-08-23T07:29:51.7972892Z 
2019-08-23T07:29:51.7972955Z running 1 test
2019-08-23T07:29:51.7973235Z test /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) ... FAILED
2019-08-23T07:29:51.7973235Z test /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) ... FAILED
2019-08-23T07:29:51.7973276Z 
2019-08-23T07:29:51.7973323Z failures:
2019-08-23T07:29:51.7973359Z 
2019-08-23T07:29:51.7973631Z ---- /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) stdout ----
2019-08-23T07:29:51.7974570Z thread '/checkout/src/test/rustdoc/issue-23106.rs - main (line 3)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputnIgfWn.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7974760Z 
2019-08-23T07:29:51.7974788Z 
2019-08-23T07:29:51.7974840Z failures:
2019-08-23T07:29:51.7975119Z     /checkout/src/test/rustdoc/issue-23106.rs - main (line 3)
---
2019-08-23T07:29:51.7976406Z ---- [rustdoc] rustdoc/issue-23744.rs stdout ----
2019-08-23T07:29:51.7976443Z 
2019-08-23T07:29:51.7976489Z error: rustdoc failed!
2019-08-23T07:29:51.7976543Z status: exit code: 101
2019-08-23T07:29:51.7977171Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
2019-08-23T07:29:51.7977520Z ------------------------------------------
2019-08-23T07:29:51.7977557Z 
2019-08-23T07:29:51.7977976Z running 2 tests
2019-08-23T07:29:51.7978350Z test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) ... FAILED
2019-08-23T07:29:51.7978350Z test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) ... FAILED
2019-08-23T07:29:51.7978631Z test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) ... FAILED
2019-08-23T07:29:51.7978674Z 
2019-08-23T07:29:51.7978733Z failures:
2019-08-23T07:29:51.7978762Z 
2019-08-23T07:29:51.7979035Z ---- /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) stdout ----
2019-08-23T07:29:51.7979722Z thread '/checkout/src/test/rustdoc/issue-23744.rs - foo (line 5)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputL4A2Mt.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7979883Z 
2019-08-23T07:29:51.7980201Z ---- /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) stdout ----
2019-08-23T07:29:51.7980201Z ---- /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) stdout ----
2019-08-23T07:29:51.7980747Z thread '/checkout/src/test/rustdoc/issue-23744.rs - foo (line 9)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputBGvQhA.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7980837Z 
2019-08-23T07:29:51.7980892Z failures:
2019-08-23T07:29:51.7981161Z     /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5)
2019-08-23T07:29:51.7981538Z     /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9)
---
2019-08-23T07:29:51.7982869Z ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
2019-08-23T07:29:51.7982914Z 
2019-08-23T07:29:51.7982960Z error: rustdoc failed!
2019-08-23T07:29:51.7983019Z status: exit code: 101
2019-08-23T07:29:51.7983649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
2019-08-23T07:29:51.7983988Z ------------------------------------------
2019-08-23T07:29:51.7984026Z 
2019-08-23T07:29:51.7984075Z running 1 test
2019-08-23T07:29:51.7984365Z test /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) ... FAILED
2019-08-23T07:29:51.7984365Z test /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) ... FAILED
2019-08-23T07:29:51.7984406Z 
2019-08-23T07:29:51.7984452Z failures:
2019-08-23T07:29:51.7984480Z 
2019-08-23T07:29:51.7984768Z ---- /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) stdout ----
2019-08-23T07:29:51.7985587Z thread '/checkout/src/test/rustdoc/issue-25944.rs - main (line 3)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputQn7WRp.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7985767Z 
2019-08-23T07:29:51.7985796Z 
2019-08-23T07:29:51.7985839Z failures:
2019-08-23T07:29:51.7986166Z     /checkout/src/test/rustdoc/issue-25944.rs - main (line 3)
---
2019-08-23T07:29:51.7987459Z ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
2019-08-23T07:29:51.7987939Z 
2019-08-23T07:29:51.7988049Z error: rustdoc failed!
2019-08-23T07:29:51.7988095Z status: exit code: 101
2019-08-23T07:29:51.7988836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
2019-08-23T07:29:51.7989177Z ------------------------------------------
2019-08-23T07:29:51.7989214Z 
2019-08-23T07:29:51.7989263Z running 1 test
2019-08-23T07:29:51.7989547Z test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) ... FAILED
2019-08-23T07:29:51.7989547Z test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) ... FAILED
2019-08-23T07:29:51.7989585Z 
2019-08-23T07:29:51.7989630Z failures:
2019-08-23T07:29:51.7989659Z 
2019-08-23T07:29:51.7990753Z ---- /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) stdout ----
2019-08-23T07:29:51.7991698Z thread '/checkout/src/test/rustdoc/issue-30252.rs - foo (line 3)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputeGSUvN.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.7991855Z 
2019-08-23T07:29:51.7991883Z 
2019-08-23T07:29:51.7991926Z failures:
2019-08-23T07:29:51.7992210Z     /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3)
---
2019-08-23T07:29:51.7994829Z ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
2019-08-23T07:29:51.7994879Z 
2019-08-23T07:29:51.7994925Z error: rustdoc failed!
2019-08-23T07:29:51.7994970Z status: exit code: 101
2019-08-23T07:29:51.7995602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
2019-08-23T07:29:51.7995928Z ------------------------------------------
2019-08-23T07:29:51.7995979Z 
2019-08-23T07:29:51.7996041Z running 5 tests
2019-08-23T07:29:51.7996333Z test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) ... FAILED
2019-08-23T07:29:51.7996333Z test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) ... FAILED
2019-08-23T07:29:51.7996649Z test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) ... FAILED
2019-08-23T07:29:51.7996940Z test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) ... FAILED
2019-08-23T07:29:51.8005257Z test /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15) ... FAILED
2019-08-23T07:29:51.8005677Z test /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) ... FAILED
2019-08-23T07:29:51.8005721Z 
2019-08-23T07:29:51.8005767Z failures:
2019-08-23T07:29:51.8005795Z 
2019-08-23T07:29:51.8006072Z ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) stdout ----
2019-08-23T07:29:51.8006739Z thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputWjVmKu.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8006901Z 
2019-08-23T07:29:51.8007217Z ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) stdout ----
2019-08-23T07:29:51.8007217Z ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) stdout ----
2019-08-23T07:29:51.8008227Z thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputDgmQ2D.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8008570Z ---- /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) stdout ----
2019-08-23T07:29:51.8008570Z ---- /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) stdout ----
2019-08-23T07:29:51.8009341Z thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputSJH62j.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8009871Z ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15) stdout ----
2019-08-23T07:29:51.8009871Z ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15) stdout ----
2019-08-23T07:29:51.8010378Z thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputqAIbpH.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8010703Z ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) stdout ----
2019-08-23T07:29:51.8010703Z ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) stdout ----
2019-08-23T07:29:51.8011200Z thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 10)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputU3v2iR.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8011310Z 
2019-08-23T07:29:51.8011351Z failures:
2019-08-23T07:29:51.8011608Z     /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47)
2019-08-23T07:29:51.8011861Z     /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74)
---
2019-08-23T07:29:51.8013858Z ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
2019-08-23T07:29:51.8013891Z 
2019-08-23T07:29:51.8013931Z error: rustdoc failed!
2019-08-23T07:29:51.8014135Z status: exit code: 101
2019-08-23T07:29:51.8014735Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
2019-08-23T07:29:51.8015153Z ------------------------------------------
2019-08-23T07:29:51.8015197Z 
2019-08-23T07:29:51.8015244Z running 1 test
2019-08-23T07:29:51.8015534Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-23T07:29:51.8015534Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-23T07:29:51.8015571Z 
2019-08-23T07:29:51.8015614Z failures:
2019-08-23T07:29:51.8015640Z 
2019-08-23T07:29:51.8015897Z ---- /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
2019-08-23T07:29:51.8016383Z thread '/checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInput70CpsG.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8016510Z 
2019-08-23T07:29:51.8016536Z 
2019-08-23T07:29:51.8016573Z failures:
2019-08-23T07:29:51.8016932Z     /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6)
---
2019-08-23T07:29:51.8018486Z ---- [rustdoc] rustdoc/issue-48377.rs stdout ----
2019-08-23T07:29:51.8018520Z 
2019-08-23T07:29:51.8018560Z error: rustdoc failed!
2019-08-23T07:29:51.8018601Z status: exit code: 101
2019-08-23T07:29:51.8019178Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
2019-08-23T07:29:51.8019490Z ------------------------------------------
2019-08-23T07:29:51.8019524Z 
2019-08-23T07:29:51.8019569Z running 2 tests
2019-08-23T07:29:51.8019810Z test /checkout/src/test/rustdoc/issue-48377.rs -  (line 10) ... FAILED
2019-08-23T07:29:51.8019810Z test /checkout/src/test/rustdoc/issue-48377.rs -  (line 10) ... FAILED
2019-08-23T07:29:51.8020070Z test /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) ... FAILED
2019-08-23T07:29:51.8020108Z 
2019-08-23T07:29:51.8020148Z failures:
2019-08-23T07:29:51.8020174Z 
2019-08-23T07:29:51.8020429Z ---- /checkout/src/test/rustdoc/issue-48377.rs -  (line 10) stdout ----
2019-08-23T07:29:51.8020919Z thread '/checkout/src/test/rustdoc/issue-48377.rs -  (line 10)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInput5Pix6q.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8021054Z 
2019-08-23T07:29:51.8021295Z ---- /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) stdout ----
2019-08-23T07:29:51.8021295Z ---- /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) stdout ----
2019-08-23T07:29:51.8021793Z thread '/checkout/src/test/rustdoc/issue-48377.rs -  (line 5)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputt7TOOr.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8021873Z 
2019-08-23T07:29:51.8021911Z failures:
2019-08-23T07:29:51.8022156Z     /checkout/src/test/rustdoc/issue-48377.rs -  (line 10)
2019-08-23T07:29:51.8022389Z     /checkout/src/test/rustdoc/issue-48377.rs -  (line 5)
---
2019-08-23T07:29:51.8026403Z 
2019-08-23T07:29:51.8026445Z failures:
2019-08-23T07:29:51.8026487Z 
2019-08-23T07:29:51.8026747Z ---- /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs -  (line 19) stdout ----
2019-08-23T07:29:51.8027257Z thread '/checkout/src/test/rustdoc/issue-54478-demo-allocator.rs -  (line 19)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputZkmnkZ.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8027397Z 
2019-08-23T07:29:51.8027424Z 
2019-08-23T07:29:51.8027475Z failures:
2019-08-23T07:29:51.8028067Z     /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs -  (line 19)
---
2019-08-23T07:29:51.8029479Z ---- [rustdoc] rustdoc/process-termination.rs stdout ----
2019-08-23T07:29:51.8029513Z 
2019-08-23T07:29:51.8030458Z error: rustdoc failed!
2019-08-23T07:29:51.8030525Z status: exit code: 101
2019-08-23T07:29:51.8031175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
2019-08-23T07:29:51.8031490Z ------------------------------------------
2019-08-23T07:29:51.8031524Z 
2019-08-23T07:29:51.8031569Z running 3 tests
2019-08-23T07:29:51.8031859Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2019-08-23T07:29:51.8031859Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2019-08-23T07:29:51.8032140Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
2019-08-23T07:29:51.8032607Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) ... FAILED
2019-08-23T07:29:51.8032667Z 
2019-08-23T07:29:51.8032708Z failures:
2019-08-23T07:29:51.8032734Z 
2019-08-23T07:29:51.8033061Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) stdout ----
2019-08-23T07:29:51.8033601Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputX98apu.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8033735Z 
2019-08-23T07:29:51.8034011Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2019-08-23T07:29:51.8034011Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2019-08-23T07:29:51.8034564Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputOF7bd7.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8035009Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) stdout ----
2019-08-23T07:29:51.8035009Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) stdout ----
2019-08-23T07:29:51.8035551Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/tempRustDocTestInputHgkcAy.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8035635Z 
2019-08-23T07:29:51.8035691Z failures:
2019-08-23T07:29:51.8035965Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)
2019-08-23T07:29:51.8036245Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)
---
2019-08-23T07:29:51.8038280Z ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
2019-08-23T07:29:51.8038320Z 
2019-08-23T07:29:51.8038361Z error: rustdoc failed!
2019-08-23T07:29:51.8038422Z status: exit code: 101
2019-08-23T07:29:51.8039021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
2019-08-23T07:29:51.8039329Z ------------------------------------------
2019-08-23T07:29:51.8039362Z 
2019-08-23T07:29:51.8039407Z running 1 test
2019-08-23T07:29:51.8039407Z running 1 test
2019-08-23T07:29:51.8039673Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) ... FAILED
2019-08-23T07:29:51.8039755Z failures:
2019-08-23T07:29:51.8039780Z 
2019-08-23T07:29:51.8039780Z 
2019-08-23T07:29:51.8040046Z ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) stdout ----
2019-08-23T07:29:51.8040691Z thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/test_option_check/tempRustDocTestInputm7480h.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8040835Z 
2019-08-23T07:29:51.8040864Z 
2019-08-23T07:29:51.8040903Z failures:
2019-08-23T07:29:51.8040903Z failures:
2019-08-23T07:29:51.8041197Z     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6)
2019-08-23T07:29:51.8041280Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-23T07:29:51.8041312Z 
2019-08-23T07:29:51.8041351Z 
2019-08-23T07:29:51.8041565Z ------------------------------------------
---
2019-08-23T07:29:51.8043261Z stdout:
2019-08-23T07:29:51.8043488Z ------------------------------------------
2019-08-23T07:29:51.8043522Z 
2019-08-23T07:29:51.8043580Z running 3 tests
2019-08-23T07:29:51.8043841Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... FAILED
2019-08-23T07:29:51.8044123Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... FAILED
2019-08-23T07:29:51.8044382Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... FAILED
2019-08-23T07:29:51.8044471Z failures:
2019-08-23T07:29:51.8044497Z 
2019-08-23T07:29:51.8044497Z 
2019-08-23T07:29:51.8044750Z ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) stdout ----
2019-08-23T07:29:51.8045285Z thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/test_option_check/tempRustDocTestInput0liJ4L.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8045418Z 
2019-08-23T07:29:51.8045690Z ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) stdout ----
2019-08-23T07:29:51.8045690Z ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) stdout ----
2019-08-23T07:29:51.8046211Z thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/test_option_check/tempRustDocTestInputRgQ43J.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8046537Z ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) stdout ----
2019-08-23T07:29:51.8046537Z ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) stdout ----
2019-08-23T07:29:51.8047146Z thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8)' panicked at 'rustdoc needs to create a tempfile: Custom { kind: Other, error: PathError { path: "/checkout/src/test/rustdoc/test_option_check/tempRustDocTestInputQafxVx.rs", err: Os { code: 30, kind: Other, message: "Read-only file system" } } }', src/libcore/result.rs:1084:5
2019-08-23T07:29:51.8047269Z 
2019-08-23T07:29:51.8047308Z failures:
2019-08-23T07:29:51.8047308Z failures:
2019-08-23T07:29:51.8047600Z     /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)
2019-08-23T07:29:51.8048155Z     /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)
2019-08-23T07:29:51.8048407Z     /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8)
2019-08-23T07:29:51.8048506Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-23T07:29:51.8048539Z 
2019-08-23T07:29:51.8048566Z 
2019-08-23T07:29:51.8049194Z ------------------------------------------
---
2019-08-23T07:29:51.8053806Z test result: FAILED. 299 passed; 17 failed; 2 ignored; 0 measured; 0 filtered out
2019-08-23T07:29:51.8053857Z 
2019-08-23T07:29:51.8053883Z 
2019-08-23T07:29:51.8053908Z 
2019-08-23T07:29:51.8055461Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-23T07:29:51.8055734Z 
2019-08-23T07:29:51.8055849Z 
2019-08-23T07:29:51.8056161Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-23T07:29:51.8056222Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-23T07:29:51.8056222Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-23T07:29:51.8056277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-23T07:29:51.8056340Z Build completed unsuccessfully in 1:17:54
2019-08-23T07:29:51.8056383Z == clock drift check ==
2019-08-23T07:29:51.8056428Z   local time: Fri Aug 23 07:29:51 UTC 2019
2019-08-23T07:29:51.8891571Z   network time: Fri, 23 Aug 2019 07:29:51 GMT
2019-08-23T07:29:51.8892711Z == end clock drift check ==
2019-08-23T07:29:53.9775867Z ##[error]Bash exited with code '1'.
2019-08-23T07:29:53.9857426Z ##[section]Starting: Checkout
2019-08-23T07:29:53.9859796Z ==============================================================================
2019-08-23T07:29:53.9859877Z Task         : Get sources
2019-08-23T07:29:53.9860087Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
