plain
2019-12-14T16:15:01.4262856Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T16:15:01.4466433Z ##[command]git config gc.auto 0
2019-12-14T16:15:01.4530270Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T16:15:01.4583265Z ##[command]git config --get-all http.proxy
2019-12-14T16:15:01.4725384Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67253/merge:refs/remotes/pull/67253/merge
---
2019-12-14T17:12:20.8715870Z .................................................................................................... 1600/9375
2019-12-14T17:12:25.0735815Z .................................................................................................... 1700/9375
2019-12-14T17:12:36.6456273Z ................................................................i................................... 1800/9375
2019-12-14T17:12:44.0672540Z .................................................................................................... 1900/9375
2019-12-14T17:12:58.1651626Z .................................................iiiii.............................................. 2000/9375
2019-12-14T17:13:08.0940050Z .................................................................................................... 2200/9375
2019-12-14T17:13:10.4064874Z .................................................................................................... 2300/9375
2019-12-14T17:13:13.6390030Z .................................................................................................... 2400/9375
2019-12-14T17:13:35.0849483Z .................................................................................................... 2500/9375
---
2019-12-14T17:16:00.7334016Z .................................................................................................... 4700/9375
2019-12-14T17:16:05.6164132Z .........................................................i...............i.......................... 4800/9375
2019-12-14T17:16:13.1195288Z .................................................................................................... 4900/9375
2019-12-14T17:16:21.0037212Z .................................................................................................... 5000/9375
2019-12-14T17:16:26.0555774Z .i.................................................................................................. 5100/9375
2019-12-14T17:16:35.9460410Z ...................................................................ii.ii...........i................ 5200/9375
2019-12-14T17:16:44.3852354Z ...i................................................................................................ 5400/9375
2019-12-14T17:16:54.2945143Z .................................................................................................... 5500/9375
2019-12-14T17:17:00.2885511Z .................................................i.................................................. 5600/9375
2019-12-14T17:17:07.1096425Z .................................................................................................... 5700/9375
2019-12-14T17:17:07.1096425Z .................................................................................................... 5700/9375
2019-12-14T17:17:16.9889660Z .................................................................................................... 5800/9375
2019-12-14T17:17:26.4537274Z .....................................ii...i..ii...........i......................................... 5900/9375
2019-12-14T17:17:44.7196446Z .................................................................................................... 6100/9375
2019-12-14T17:17:52.3343722Z .................................................................................................... 6200/9375
2019-12-14T17:17:52.3343722Z .................................................................................................... 6200/9375
2019-12-14T17:18:03.8879466Z .............................................................i..ii.................................. 6300/9375
2019-12-14T17:18:30.0520147Z .................................................................................................... 6500/9375
2019-12-14T17:18:31.9226478Z .................................i.................................................................. 6600/9375
2019-12-14T17:18:34.0412153Z .................................................................................................... 6700/9375
2019-12-14T17:18:36.1915226Z .........................i.......................................................................... 6800/9375
---
2019-12-14T17:20:09.5103541Z .................................................................................................... 7400/9375
2019-12-14T17:20:14.2606070Z .................................................................................................... 7500/9375
2019-12-14T17:20:19.5648644Z .................................................................................................... 7600/9375
2019-12-14T17:20:28.3990382Z .................................................................................................... 7700/9375
2019-12-14T17:20:36.4246092Z ..............................................iiii.................................................. 7800/9375
2019-12-14T17:20:50.3082979Z .................................................................................................... 8000/9375
2019-12-14T17:20:58.2278139Z .................................................................................................... 8100/9375
2019-12-14T17:21:11.7022265Z .................................................................................................... 8200/9375
2019-12-14T17:21:19.3772771Z .................................................................................................... 8300/9375
---
2019-12-14T17:23:35.2979481Z  finished in 6.128
2019-12-14T17:23:35.3164545Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:23:35.4874659Z 
2019-12-14T17:23:35.4874924Z running 166 tests
2019-12-14T17:23:38.4080821Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-14T17:23:40.4436911Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-14T17:23:40.4438556Z 
2019-12-14T17:23:40.4445009Z  finished in 5.128
2019-12-14T17:23:40.4627801Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:23:40.6192335Z 
---
2019-12-14T17:23:42.5101344Z  finished in 2.047
2019-12-14T17:23:42.5295380Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:23:42.6838304Z 
2019-12-14T17:23:42.6839865Z running 9 tests
2019-12-14T17:23:42.6841610Z iiiiiiiii
2019-12-14T17:23:42.6842058Z 
2019-12-14T17:23:42.6843058Z  finished in 0.154
2019-12-14T17:23:42.7016756Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:23:42.8826653Z 
---
2019-12-14T17:24:02.0029536Z  finished in 18.726
2019-12-14T17:24:02.0029891Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:24:02.0029929Z 
2019-12-14T17:24:02.0029986Z running 124 tests
2019-12-14T17:24:25.4468183Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-14T17:24:29.4381760Z .i.iii.....iiiiii.....ii
2019-12-14T17:24:29.4383881Z 
2019-12-14T17:24:29.4384145Z  finished in 27.988
2019-12-14T17:24:29.4391012Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-14T17:24:29.4391571Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-14T17:31:30.1168897Z ------------------------------------------
2019-12-14T17:31:30.1169498Z error: unknown start of token: `
2019-12-14T17:31:30.1170203Z   --> <doctest>:21:46
2019-12-14T17:31:30.1170442Z    |
2019-12-14T17:31:30.1170654Z 21 | There are a number of helper methods on the [`Formatter`] struct to help you with manual
2019-12-14T17:31:30.1171198Z    |
2019-12-14T17:31:30.1172284Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1172540Z    |
2019-12-14T17:31:30.1172540Z    |
2019-12-14T17:31:30.1173131Z 21 | There are a number of helper methods on the ['Formatter`] struct to help you with manual
2019-12-14T17:31:30.1173592Z 
2019-12-14T17:31:30.1173642Z error: unknown start of token: `
2019-12-14T17:31:30.1174085Z   --> <doctest>:21:56
2019-12-14T17:31:30.1174133Z    |
2019-12-14T17:31:30.1174133Z    |
2019-12-14T17:31:30.1174207Z 21 | There are a number of helper methods on the [`Formatter`] struct to help you with manual
2019-12-14T17:31:30.1174320Z    |
2019-12-14T17:31:30.1174606Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1174665Z    |
2019-12-14T17:31:30.1174665Z    |
2019-12-14T17:31:30.1175098Z 21 | There are a number of helper methods on the [`Formatter'] struct to help you with manual
2019-12-14T17:31:30.1175406Z 
2019-12-14T17:31:30.1175447Z error: unknown start of token: `
2019-12-14T17:31:30.1176176Z   --> <doctest>:22:27
2019-12-14T17:31:30.1176228Z    |
2019-12-14T17:31:30.1176228Z    |
2019-12-14T17:31:30.1176286Z 22 | implementations, such as [`debug_struct`][debug_struct].
2019-12-14T17:31:30.1176395Z    |
2019-12-14T17:31:30.1176693Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1177114Z    |
2019-12-14T17:31:30.1177114Z    |
2019-12-14T17:31:30.1177433Z 22 | implementations, such as ['debug_struct`][debug_struct].
2019-12-14T17:31:30.1177535Z 
2019-12-14T17:31:30.1177577Z error: unknown start of token: `
2019-12-14T17:31:30.1177820Z   --> <doctest>:22:40
2019-12-14T17:31:30.1177885Z    |
2019-12-14T17:31:30.1177885Z    |
2019-12-14T17:31:30.1177930Z 22 | implementations, such as [`debug_struct`][debug_struct].
2019-12-14T17:31:30.1178039Z    |
2019-12-14T17:31:30.1178332Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1178381Z    |
2019-12-14T17:31:30.1178381Z    |
2019-12-14T17:31:30.1178670Z 22 | implementations, such as [`debug_struct'][debug_struct].
2019-12-14T17:31:30.1178753Z 
2019-12-14T17:31:30.1178794Z error: unknown start of token: `
2019-12-14T17:31:30.1179203Z   --> <doctest>:24:1
2019-12-14T17:31:30.1179240Z    |
---
2019-12-14T17:31:30.1188046Z    |       ^
2019-12-14T17:31:30.1188088Z    |
2019-12-14T17:31:30.1188416Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1188486Z    |
2019-12-14T17:31:30.1188764Z 24 | `Debug' implementations using either `derive` or the debug builder API
2019-12-14T17:31:30.1188841Z 
2019-12-14T17:31:30.1188904Z error: unknown start of token: `
2019-12-14T17:31:30.1189132Z   --> <doctest>:24:38
2019-12-14T17:31:30.1189276Z    |
2019-12-14T17:31:30.1189276Z    |
2019-12-14T17:31:30.1189501Z 24 | `Debug` implementations using either `derive` or the debug builder API
2019-12-14T17:31:30.1189543Z    |                                      ^
2019-12-14T17:31:30.1189578Z    |
2019-12-14T17:31:30.1189868Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1190087Z    |
2019-12-14T17:31:30.1190339Z 24 | `Debug` implementations using either 'derive` or the debug builder API
2019-12-14T17:31:30.1190431Z 
2019-12-14T17:31:30.1190467Z error: unknown start of token: `
2019-12-14T17:31:30.1190664Z   --> <doctest>:24:45
2019-12-14T17:31:30.1190722Z    |
2019-12-14T17:31:30.1190722Z    |
2019-12-14T17:31:30.1190762Z 24 | `Debug` implementations using either `derive` or the debug builder API
2019-12-14T17:31:30.1190805Z    |                                             ^
2019-12-14T17:31:30.1190859Z    |
2019-12-14T17:31:30.1191112Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1191164Z    |
2019-12-14T17:31:30.1191404Z 24 | `Debug` implementations using either `derive' or the debug builder API
2019-12-14T17:31:30.1191495Z 
2019-12-14T17:31:30.1191532Z error: unknown start of token: `
2019-12-14T17:31:30.1191744Z   --> <doctest>:25:5
2019-12-14T17:31:30.1191783Z    |
2019-12-14T17:31:30.1191783Z    |
2019-12-14T17:31:30.1192038Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1192338Z    |
2019-12-14T17:31:30.1192624Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1192667Z    |
2019-12-14T17:31:30.1192667Z    |
2019-12-14T17:31:30.1193104Z 25 | on ['Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1193168Z 
2019-12-14T17:31:30.1193365Z error: unknown start of token: `
2019-12-14T17:31:30.1193581Z   --> <doctest>:25:15
2019-12-14T17:31:30.1193618Z    |
2019-12-14T17:31:30.1193618Z    |
2019-12-14T17:31:30.1193847Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1193939Z    |
2019-12-14T17:31:30.1194173Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1194231Z    |
2019-12-14T17:31:30.1194231Z    |
2019-12-14T17:31:30.1194470Z 25 | on [`Formatter'] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1194534Z 
2019-12-14T17:31:30.1194586Z error: unknown start of token: `
2019-12-14T17:31:30.1194771Z   --> <doctest>:25:68
2019-12-14T17:31:30.1194807Z    |
2019-12-14T17:31:30.1194807Z    |
2019-12-14T17:31:30.1195052Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1195134Z    |
2019-12-14T17:31:30.1195765Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1195821Z    |
2019-12-14T17:31:30.1195821Z    |
2019-12-14T17:31:30.1196110Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: '{:#?}`.
2019-12-14T17:31:30.1196216Z 
2019-12-14T17:31:30.1196257Z error: unknown start of token: `
2019-12-14T17:31:30.1196606Z   --> <doctest>:25:74
2019-12-14T17:31:30.1196680Z    |
2019-12-14T17:31:30.1196680Z    |
2019-12-14T17:31:30.1196992Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
2019-12-14T17:31:30.1197112Z    |
2019-12-14T17:31:30.1197400Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1197448Z    |
2019-12-14T17:31:30.1197448Z    |
2019-12-14T17:31:30.1197727Z 25 | on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}'.
2019-12-14T17:31:30.1197929Z 
2019-12-14T17:31:30.1197969Z error: unknown start of token: `
2019-12-14T17:31:30.1198236Z   --> <doctest>:28:2
2019-12-14T17:31:30.1198280Z    |
2019-12-14T17:31:30.1198280Z    |
2019-12-14T17:31:30.1198327Z 28 | [`Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-14T17:31:30.1198389Z    |  ^
2019-12-14T17:31:30.1198728Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1198776Z    |
2019-12-14T17:31:30.1198776Z    |
2019-12-14T17:31:30.1199353Z 28 | ['Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-14T17:31:30.1199391Z    |  ^
2019-12-14T17:31:30.1199468Z error: unknown start of token: `
2019-12-14T17:31:30.1199652Z   --> <doctest>:28:12
2019-12-14T17:31:30.1199688Z    |
2019-12-14T17:31:30.1199688Z    |
2019-12-14T17:31:30.1199724Z 28 | [`Formatter`]: ../../std/fmt/struct.Formatter.html
2019-12-14T17:31:30.1199819Z    |
2019-12-14T17:31:30.1200056Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1200114Z    |
2019-12-14T17:31:30.1200114Z    |
2019-12-14T17:31:30.1200324Z 28 | [`Formatter']: ../../std/fmt/struct.Formatter.html
2019-12-14T17:31:30.1200386Z 
2019-12-14T17:31:30.1200438Z error: unknown start of token: `
2019-12-14T17:31:30.1200628Z   --> <doctest>:30:22
2019-12-14T17:31:30.1200665Z    |
2019-12-14T17:31:30.1200665Z    |
2019-12-14T17:31:30.1200877Z 30 | Pretty-printing with `#?`:
2019-12-14T17:31:30.1200950Z    |
2019-12-14T17:31:30.1201184Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1201243Z    |
2019-12-14T17:31:30.1201243Z    |
2019-12-14T17:31:30.1201431Z 30 | Pretty-printing with '#?`:
2019-12-14T17:31:30.1201511Z 
2019-12-14T17:31:30.1201545Z error: unknown start of token: `
2019-12-14T17:31:30.1201736Z   --> <doctest>:30:25
2019-12-14T17:31:30.1201772Z    |
2019-12-14T17:31:30.1201772Z    |
2019-12-14T17:31:30.1201980Z 30 | Pretty-printing with `#?`:
2019-12-14T17:31:30.1202052Z    |
2019-12-14T17:31:30.1202304Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-12-14T17:31:30.1202344Z    |
2019-12-14T17:31:30.1202344Z    |
2019-12-14T17:31:30.1202537Z 30 | Pretty-printing with `#?':
2019-12-14T17:31:30.1202618Z 
2019-12-14T17:31:30.1202653Z warning: could not parse code block as Rust code
2019-12-14T17:31:30.1202864Z    --> /checkout/src/libcore/fmt/mod.rs:448:5
2019-12-14T17:31:30.1202921Z     |
---
2019-12-14T17:31:30.1204355Z 
2019-12-14T17:31:30.1204389Z error: unterminated double quote string
2019-12-14T17:31:30.1204593Z   --> <doctest>:13:24
2019-12-14T17:31:30.1204699Z    |
2019-12-14T17:31:30.1204732Z 13 |       "{"A": 10, "B": 11}"
2019-12-14T17:31:30.1204819Z 14 | |  );
2019-12-14T17:31:30.1204851Z    | |____^
2019-12-14T17:31:30.1204874Z 
2019-12-14T17:31:30.1204894Z 
---
2019-12-14T17:31:30.1206582Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-14T17:31:30.1206641Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-14T17:31:30.1206708Z 
2019-12-14T17:31:30.1206735Z 
2019-12-14T17:31:30.1208437Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-14T17:31:30.1208718Z 
2019-12-14T17:31:30.1208748Z 
2019-12-14T17:31:30.1208795Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-14T17:31:30.1208863Z Build completed unsuccessfully in 1:10:29
2019-12-14T17:31:30.1208863Z Build completed unsuccessfully in 1:10:29
2019-12-14T17:31:30.1254846Z == clock drift check ==
2019-12-14T17:31:30.1274353Z   local time: Sat Dec 14 17:31:30 UTC 2019
2019-12-14T17:31:30.6705726Z   network time: Sat, 14 Dec 2019 17:31:30 GMT
2019-12-14T17:31:30.6711115Z == end clock drift check ==
2019-12-14T17:31:33.0956500Z 
2019-12-14T17:31:33.1073339Z ##[error]Bash exited with code '1'.
2019-12-14T17:31:33.1119517Z ##[section]Starting: Checkout
2019-12-14T17:31:33.1120985Z ==============================================================================
2019-12-14T17:31:33.1121050Z Task         : Get sources
2019-12-14T17:31:33.1121090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
