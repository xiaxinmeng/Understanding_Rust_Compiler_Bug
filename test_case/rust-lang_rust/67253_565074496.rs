plain
2019-12-12T14:53:39.5870119Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T14:53:39.6058201Z ##[command]git config gc.auto 0
2019-12-12T14:53:40.3640119Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T14:53:40.3651363Z ##[command]git config --get-all http.proxy
2019-12-12T14:53:40.3662061Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67253/merge:refs/remotes/pull/67253/merge
---
2019-12-12T15:54:06.6131825Z .................................................................................................... 1600/9364
2019-12-12T15:54:11.0912680Z .................................................................................................... 1700/9364
2019-12-12T15:54:23.3258182Z ................................................................i................................... 1800/9364
2019-12-12T15:54:31.1680364Z .................................................................................................... 1900/9364
2019-12-12T15:54:46.0645298Z .................................................iiiii.............................................. 2000/9364
2019-12-12T15:54:56.6532199Z .................................................................................................... 2200/9364
2019-12-12T15:54:59.3016108Z .................................................................................................... 2300/9364
2019-12-12T15:55:02.7817530Z .................................................................................................... 2400/9364
2019-12-12T15:55:25.5561176Z .................................................................................................... 2500/9364
---
2019-12-12T15:58:00.2708867Z .................................................................................................... 4700/9364
2019-12-12T15:58:05.5263531Z .........................................................i...............i.......................... 4800/9364
2019-12-12T15:58:13.5790992Z .................................................................................................... 4900/9364
2019-12-12T15:58:22.1556249Z .................................................................................................... 5000/9364
2019-12-12T15:58:27.5844912Z .i.................................................................................................. 5100/9364
2019-12-12T15:58:38.0505710Z ...................................................................ii.ii...........i................ 5200/9364
2019-12-12T15:58:47.1735858Z ...i................................................................................................ 5400/9364
2019-12-12T15:58:57.6689757Z .................................................................................................... 5500/9364
2019-12-12T15:59:04.0597622Z .................................................i.................................................. 5600/9364
2019-12-12T15:59:11.3507447Z .................................................................................................... 5700/9364
2019-12-12T15:59:11.3507447Z .................................................................................................... 5700/9364
2019-12-12T15:59:21.8577565Z .................................................................................................... 5800/9364
2019-12-12T15:59:32.1519936Z .....................................ii...i..ii...........i......................................... 5900/9364
2019-12-12T15:59:51.8561607Z .................................................................................................... 6100/9364
2019-12-12T16:00:00.3538379Z .................................................................................................... 6200/9364
2019-12-12T16:00:00.3538379Z .................................................................................................... 6200/9364
2019-12-12T16:00:10.5204063Z .............................................................i..ii.................................. 6300/9364
2019-12-12T16:00:38.9125387Z .................................................................................................... 6500/9364
2019-12-12T16:00:41.0806708Z .................................i.................................................................. 6600/9364
2019-12-12T16:00:43.4397848Z .................................................................................................... 6700/9364
2019-12-12T16:00:45.8718585Z ........................i........................................................................... 6800/9364
---
2019-12-12T16:02:25.6346613Z .................................................................................................... 7400/9364
2019-12-12T16:02:30.6628561Z .................................................................................................... 7500/9364
2019-12-12T16:02:37.4380788Z .................................................................................................... 7600/9364
2019-12-12T16:02:46.1457849Z .................................................................................................... 7700/9364
2019-12-12T16:02:54.2161911Z ...........................................iiii..................................................... 7800/9364
2019-12-12T16:03:08.9920674Z .................................................................................................... 8000/9364
2019-12-12T16:03:17.7053736Z .................................................................................................... 8100/9364
2019-12-12T16:03:31.6542626Z .................................................................................................... 8200/9364
2019-12-12T16:03:40.3964764Z .................................................................................................... 8300/9364
---
2019-12-12T16:06:02.8978382Z  finished in 6.335
2019-12-12T16:06:02.9158468Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:06:03.0970519Z 
2019-12-12T16:06:03.0970842Z running 166 tests
2019-12-12T16:06:06.2637594Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-12T16:06:08.4156232Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-12T16:06:08.4159642Z 
2019-12-12T16:06:08.4163257Z  finished in 5.499
2019-12-12T16:06:08.4370854Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:06:08.6039857Z 
---
2019-12-12T16:06:10.5753354Z  finished in 2.138
2019-12-12T16:06:10.5962028Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:06:10.7626136Z 
2019-12-12T16:06:10.7628033Z running 9 tests
2019-12-12T16:06:10.7628905Z iiiiiiiii
2019-12-12T16:06:10.7629263Z 
2019-12-12T16:06:10.7629327Z  finished in 0.166
2019-12-12T16:06:10.7820253Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:06:10.9653630Z 
---
2019-12-12T16:06:30.9830745Z  finished in 20.201
2019-12-12T16:06:31.0065464Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:06:31.1881298Z 
2019-12-12T16:06:31.1882418Z running 124 tests
2019-12-12T16:06:57.5455236Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-12T16:07:01.8500452Z .i.iii.....iiiiii.....ii
2019-12-12T16:07:01.8502131Z 
2019-12-12T16:07:01.8506019Z  finished in 30.844
2019-12-12T16:07:01.8516186Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-12T16:07:01.8517324Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-12T16:13:30.5925659Z ------------------------------------------
2019-12-12T16:13:30.5925715Z error: unknown start of token: `
2019-12-12T16:13:30.5926128Z   --> <doctest>:21:23
2019-12-12T16:13:30.5926190Z    |
2019-12-12T16:13:30.5926340Z 21 | There are a number of `debug_*` methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5926447Z    |
2019-12-12T16:13:30.5926796Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5926851Z    |
2019-12-12T16:13:30.5926851Z    |
2019-12-12T16:13:30.5927140Z 21 | There are a number of 'debug_*` methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5927251Z 
2019-12-12T16:13:30.5927298Z error: unknown start of token: `
2019-12-12T16:13:30.5927538Z   --> <doctest>:21:31
2019-12-12T16:13:30.5927621Z    |
2019-12-12T16:13:30.5927621Z    |
2019-12-12T16:13:30.5927674Z 21 | There are a number of `debug_*` methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5927795Z    |
2019-12-12T16:13:30.5928102Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5928158Z    |
2019-12-12T16:13:30.5928158Z    |
2019-12-12T16:13:30.5928463Z 21 | There are a number of `debug_*' methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5928553Z 
2019-12-12T16:13:30.5928598Z error: unknown start of token: `
2019-12-12T16:13:30.5928892Z   --> <doctest>:21:45
2019-12-12T16:13:30.5928940Z    |
2019-12-12T16:13:30.5928940Z    |
2019-12-12T16:13:30.5928994Z 21 | There are a number of `debug_*` methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5929118Z    |
2019-12-12T16:13:30.5929772Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5929856Z    |
2019-12-12T16:13:30.5929856Z    |
2019-12-12T16:13:30.5930149Z 21 | There are a number of `debug_*` methods on ['Formatter`] to help you with manual
2019-12-12T16:13:30.5930255Z 
2019-12-12T16:13:30.5930326Z error: unknown start of token: `
2019-12-12T16:13:30.5930553Z   --> <doctest>:21:55
2019-12-12T16:13:30.5930602Z    |
2019-12-12T16:13:30.5930602Z    |
2019-12-12T16:13:30.5930675Z 21 | There are a number of `debug_*` methods on [`Formatter`] to help you with manual
2019-12-12T16:13:30.5930783Z    |
2019-12-12T16:13:30.5931094Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5931148Z    |
2019-12-12T16:13:30.5931148Z    |
2019-12-12T16:13:30.5931437Z 21 | There are a number of `debug_*` methods on [`Formatter'] to help you with manual
2019-12-12T16:13:30.5931562Z 
2019-12-12T16:13:30.5931608Z error: unknown start of token: `
2019-12-12T16:13:30.5931835Z   --> <doctest>:22:27
2019-12-12T16:13:30.5931902Z    |
2019-12-12T16:13:30.5931902Z    |
2019-12-12T16:13:30.5931962Z 22 | implementations, such as [`debug_struct`][debug_struct].
2019-12-12T16:13:30.5932077Z    |
2019-12-12T16:13:30.5932374Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5932429Z    |
2019-12-12T16:13:30.5932429Z    |
2019-12-12T16:13:30.5932734Z 22 | implementations, such as ['debug_struct`][debug_struct].
2019-12-12T16:13:30.5932841Z 
2019-12-12T16:13:30.5932886Z error: unknown start of token: `
2019-12-12T16:13:30.5933130Z   --> <doctest>:22:40
2019-12-12T16:13:30.5933179Z    |
2019-12-12T16:13:30.5933179Z    |
2019-12-12T16:13:30.5933229Z 22 | implementations, such as [`debug_struct`][debug_struct].
2019-12-12T16:13:30.5933488Z    |
2019-12-12T16:13:30.5933816Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5933871Z    |
2019-12-12T16:13:30.5933871Z    |
2019-12-12T16:13:30.5934321Z 22 | implementations, such as [`debug_struct'][debug_struct].
2019-12-12T16:13:30.5934743Z 
2019-12-12T16:13:30.5934794Z error: unknown start of token: `
2019-12-12T16:13:30.5935109Z   --> <doctest>:24:1
2019-12-12T16:13:30.5935159Z    |
---
2019-12-12T16:13:30.5936540Z    |       ^
2019-12-12T16:13:30.5936585Z    |
2019-12-12T16:13:30.5936880Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5936952Z    |
2019-12-12T16:13:30.5937230Z 24 | `Debug' implementations using either `derive` or the debug builder API
2019-12-12T16:13:30.5937334Z 
2019-12-12T16:13:30.5937380Z error: unknown start of token: `
2019-12-12T16:13:30.5937602Z   --> <doctest>:24:38
2019-12-12T16:13:30.5937651Z    |
2019-12-12T16:13:30.5937651Z    |
2019-12-12T16:13:30.5937771Z 24 | `Debug` implementations using either `derive` or the debug builder API
2019-12-12T16:13:30.5937825Z    |                                      ^
2019-12-12T16:13:30.5937882Z    |
2019-12-12T16:13:30.5938254Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5938307Z    |
2019-12-12T16:13:30.5938588Z 24 | `Debug` implementations using either 'derive` or the debug builder API
2019-12-12T16:13:30.5938703Z 
2019-12-12T16:13:30.5938747Z error: unknown start of token: `
2019-12-12T16:13:30.5938972Z   --> <doctest>:24:45
2019-12-12T16:13:30.5939040Z    |
2019-12-12T16:13:30.5939040Z    |
2019-12-12T16:13:30.5939091Z 24 | `Debug` implementations using either `derive` or the debug builder API
2019-12-12T16:13:30.5939544Z    |                                             ^
2019-12-12T16:13:30.5939700Z    |
2019-12-12T16:13:30.5940068Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5940123Z    |
2019-12-12T16:13:30.5940417Z 24 | `Debug` implementations using either `derive' or the debug builder API
2019-12-12T16:13:30.5940523Z 
2019-12-12T16:13:30.5940569Z error: unknown start of token: `
2019-12-12T16:13:30.5940815Z   --> <doctest>:25:5
2019-12-12T16:13:30.5940863Z    |
2019-12-12T16:13:30.5940863Z    |
2019-12-12T16:13:30.5941155Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5941275Z    |
2019-12-12T16:13:30.5941566Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5941639Z    |
2019-12-12T16:13:30.5941639Z    |
2019-12-12T16:13:30.5941922Z 25 | on ['Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5942007Z 
2019-12-12T16:13:30.5942073Z error: unknown start of token: `
2019-12-12T16:13:30.5942293Z   --> <doctest>:25:15
2019-12-12T16:13:30.5942342Z    |
2019-12-12T16:13:30.5942342Z    |
2019-12-12T16:13:30.5942639Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5942944Z    |
2019-12-12T16:13:30.5943269Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5943343Z    |
2019-12-12T16:13:30.5943343Z    |
2019-12-12T16:13:30.5943717Z 25 | on [`Formatter'] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5943834Z 
2019-12-12T16:13:30.5943881Z error: unknown start of token: `
2019-12-12T16:13:30.5944131Z   --> <doctest>:25:68
2019-12-12T16:13:30.5944181Z    |
2019-12-12T16:13:30.5944181Z    |
2019-12-12T16:13:30.5944484Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5944596Z    |
2019-12-12T16:13:30.5944907Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5944962Z    |
2019-12-12T16:13:30.5944962Z    |
2019-12-12T16:13:30.5945238Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: '{:#?}`.
2019-12-12T16:13:30.5945368Z 
2019-12-12T16:13:30.5945414Z error: unknown start of token: `
2019-12-12T16:13:30.5945647Z   --> <doctest>:25:74
2019-12-12T16:13:30.5945716Z    |
2019-12-12T16:13:30.5945716Z    |
2019-12-12T16:13:30.5946001Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-12T16:13:30.5946201Z    |
2019-12-12T16:13:30.5946492Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5946545Z    |
2019-12-12T16:13:30.5946545Z    |
2019-12-12T16:13:30.5946845Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}'.
2019-12-12T16:13:30.5946943Z 
2019-12-12T16:13:30.5946999Z error: unknown start of token: `
2019-12-12T16:13:30.5947243Z   --> <doctest>:28:2
2019-12-12T16:13:30.5947291Z    |
2019-12-12T16:13:30.5947291Z    |
2019-12-12T16:13:30.5947342Z 28 | [`Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-12T16:13:30.5947411Z    |  ^
2019-12-12T16:13:30.5947753Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5947830Z    |
2019-12-12T16:13:30.5947830Z    |
2019-12-12T16:13:30.5948089Z 28 | ['Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-12T16:13:30.5948140Z    |  ^
2019-12-12T16:13:30.5948239Z error: unknown start of token: `
2019-12-12T16:13:30.5948460Z   --> <doctest>:28:12
2019-12-12T16:13:30.5948508Z    |
2019-12-12T16:13:30.5948508Z    |
2019-12-12T16:13:30.5948578Z 28 | [`Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-12T16:13:30.5948671Z    |
2019-12-12T16:13:30.5949318Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5949414Z    |
2019-12-12T16:13:30.5949414Z    |
2019-12-12T16:13:30.5949718Z 28 | [`Formatter']: ../../std/fmt/struct.Formatter.html
2019-12-12T16:13:30.5949824Z 
2019-12-12T16:13:30.5949871Z error: unknown start of token: `
2019-12-12T16:13:30.5950094Z   --> <doctest>:30:22
2019-12-12T16:13:30.5950151Z    |
2019-12-12T16:13:30.5950151Z    |
2019-12-12T16:13:30.5950405Z 30 | Pretty-printing with `#?`:
2019-12-12T16:13:30.5950504Z    |
2019-12-12T16:13:30.5950812Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5950867Z    |
2019-12-12T16:13:30.5950867Z    |
2019-12-12T16:13:30.5951094Z 30 | Pretty-printing with '#?`:
2019-12-12T16:13:30.5951199Z 
2019-12-12T16:13:30.5951246Z error: unknown start of token: `
2019-12-12T16:13:30.5951468Z   --> <doctest>:30:25
2019-12-12T16:13:30.5951536Z    |
2019-12-12T16:13:30.5951536Z    |
2019-12-12T16:13:30.5951764Z 30 | Pretty-printing with `#?`:
2019-12-12T16:13:30.5951991Z    |
2019-12-12T16:13:30.5952340Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-12T16:13:30.5952394Z    |
2019-12-12T16:13:30.5952394Z    |
2019-12-12T16:13:30.5952617Z 30 | Pretty-printing with `#?':
2019-12-12T16:13:30.5952819Z 
2019-12-12T16:13:30.5952868Z warning: could not parse code block as Rust code
2019-12-12T16:13:30.5953170Z    --> /checkout/src/libcore/fmt/mod.rs:448:5
2019-12-12T16:13:30.5953228Z     |
---
2019-12-12T16:13:30.5954880Z 
2019-12-12T16:13:30.5954929Z error: unterminated double quote string
2019-12-12T16:13:30.5955198Z   --> <doctest>:13:24
2019-12-12T16:13:30.5955250Z    |
2019-12-12T16:13:30.5955298Z 13 |       "{"A": 10, "B": 11}"
2019-12-12T16:13:30.5955428Z 14 | |  );
2019-12-12T16:13:30.5955476Z    | |____^
2019-12-12T16:13:30.5955507Z 
2019-12-12T16:13:30.5955555Z 
---
2019-12-12T16:13:30.5956704Z test result: FAILED. 324 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
2019-12-12T16:13:30.5956746Z 
2019-12-12T16:13:30.5962940Z 
2019-12-12T16:13:30.5963022Z 
2019-12-12T16:13:30.5965150Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-12T16:13:30.5965607Z 
2019-12-12T16:13:30.5965663Z 
2019-12-12T16:13:30.5965719Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-12T16:13:30.5965856Z Build completed unsuccessfully in 1:14:04
2019-12-12T16:13:30.5965856Z Build completed unsuccessfully in 1:14:04
2019-12-12T16:13:30.5965940Z src/tools/compiletest/src/main.rs:537:22
2019-12-12T16:13:30.5966002Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-12T16:13:30.5977053Z == clock drift check ==
2019-12-12T16:13:30.6003123Z   local time: Thu Dec 12 16:13:30 UTC 2019
2019-12-12T16:13:31.1487495Z   network time: Thu, 12 Dec 2019 16:13:31 GMT
2019-12-12T16:13:34.1919224Z == end clock drift check ==
2019-12-12T16:13:34.1919332Z 
2019-12-12T16:13:34.2046243Z ##[error]Bash exited with code '1'.
2019-12-12T16:13:34.2096407Z ##[section]Starting: Checkout
2019-12-12T16:13:34.2098283Z ==============================================================================
2019-12-12T16:13:34.2098359Z Task         : Get sources
2019-12-12T16:13:34.2098431Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
