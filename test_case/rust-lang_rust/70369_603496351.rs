plain
2020-03-24T19:46:24.9717009Z ========================== Starting Command Output ===========================
2020-03-24T19:46:24.9719547Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3eb08a47-eaa3-466b-b044-45ce24f21cc7.sh
2020-03-24T19:46:24.9719965Z 
2020-03-24T19:46:24.9723590Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T19:46:24.9759709Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70369/merge to s
2020-03-24T19:46:24.9765093Z Task         : Get sources
2020-03-24T19:46:24.9765643Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T19:46:24.9766174Z Version      : 1.0.0
2020-03-24T19:46:24.9766550Z Author       : Microsoft
---
2020-03-24T19:46:25.9845700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T19:46:25.9850388Z ##[command]git config gc.auto 0
2020-03-24T19:46:25.9854054Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T19:46:25.9857195Z ##[command]git config --get-all http.proxy
2020-03-24T19:46:25.9862326Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70369/merge:refs/remotes/pull/70369/merge
---
2020-03-24T19:53:05.1278291Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T19:53:05.5433697Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T19:53:12.5822738Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T19:53:19.1661157Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T19:53:22.1133632Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T19:53:23.7842391Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T19:53:48.1566139Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T19:53:55.2546914Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T19:54:37.3820880Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T20:13:57.5793366Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T20:13:58.6868834Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T20:14:09.3666614Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T20:14:20.7530828Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T20:14:26.1342965Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T20:14:27.6455776Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T20:15:10.4581309Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T20:15:20.6151682Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T20:16:25.4218878Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T20:38:03.3471887Z .................................................................................................... 1700/9832
2020-03-24T20:38:06.7607820Z .................................................................................................... 1800/9832
2020-03-24T20:38:15.1435666Z .......................................................................................i............ 1900/9832
2020-03-24T20:38:21.1746237Z .................................................................................................... 2000/9832
2020-03-24T20:38:26.6354651Z .............................................................................iiiii.................. 2100/9832
2020-03-24T20:38:44.8868288Z .................................................................................................... 2300/9832
2020-03-24T20:38:46.8560249Z .................................................................................................... 2400/9832
2020-03-24T20:38:49.0200299Z .................................................................................................... 2500/9832
2020-03-24T20:38:56.9487523Z .................................................................................................... 2600/9832
---
2020-03-24T20:41:20.6483600Z ....................................................i...............i............................... 5000/9832
2020-03-24T20:41:27.2427584Z .................................................................................................... 5100/9832
2020-03-24T20:41:33.5420916Z .................................................................................................i.. 5200/9832
2020-03-24T20:41:37.8270211Z .................................................................................................... 5300/9832
2020-03-24T20:41:46.6968621Z ................................................................................ii.ii........i...i.. 5400/9832
2020-03-24T20:41:52.9398693Z ....................i............................................................................... 5600/9832
2020-03-24T20:41:59.0827380Z .........................i.......................................................................... 5700/9832
2020-03-24T20:42:05.5885234Z ..........................................ii....................................i................... 5800/9832
2020-03-24T20:42:11.9956852Z .................................................................................................... 5900/9832
2020-03-24T20:42:11.9956852Z .................................................................................................... 5900/9832
2020-03-24T20:42:16.9845691Z .................................................................................................... 6000/9832
2020-03-24T20:42:24.9975985Z ..........................................................................ii...i..ii...........i.... 6100/9832
2020-03-24T20:42:42.2540810Z .................................................................................................... 6300/9832
2020-03-24T20:42:45.2584672Z .................................................................................................... 6400/9832
2020-03-24T20:42:48.2868952Z .................................................................................................... 6500/9832
2020-03-24T20:42:48.2868952Z .................................................................................................... 6500/9832
2020-03-24T20:42:58.5392727Z ....i..ii........................................................................................... 6600/9832
2020-03-24T20:43:15.4717130Z .................................................................................................... 6800/9832
2020-03-24T20:43:17.2469138Z ...i................................................................................................ 6900/9832
2020-03-24T20:43:18.9997542Z .................................................................................................... 7000/9832
2020-03-24T20:43:20.9653438Z ......................................i............................................................. 7100/9832
---
2020-03-24T20:44:44.1904325Z .................................................................................................... 7800/9832
2020-03-24T20:44:48.1532050Z .................................................................................................... 7900/9832
2020-03-24T20:44:53.6785688Z ..............................................................................................i..... 8000/9832
2020-03-24T20:45:00.4097603Z .................................................................................................... 8100/9832
2020-03-24T20:45:06.5947521Z ...........................................iiiiiiiiii.i............................................. 8200/9832
2020-03-24T20:45:18.8430964Z .................................................................................................... 8400/9832
2020-03-24T20:45:23.4691607Z .................................................................................................... 8500/9832
2020-03-24T20:45:35.8444201Z .................................................................................................... 8600/9832
2020-03-24T20:45:43.9399623Z .................................................................................................... 8700/9832
---
2020-03-24T20:47:21.0595039Z 1 error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0595572Z -   --> $DIR/typeck_type_placeholder_item.rs:154:18
2020-03-24T20:47:21.0596050Z +   --> $DIR/typeck_type_placeholder_item.rs:152:18
2020-03-24T20:47:21.0596271Z 3    |
2020-03-24T20:47:21.0596441Z 4 LL | struct BadStruct<_>(_);
2020-03-24T20:47:21.0596945Z 
2020-03-24T20:47:21.0597056Z 6 
2020-03-24T20:47:21.0597285Z 7 error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0597763Z -   --> $DIR/typeck_type_placeholder_item.rs:157:16
2020-03-24T20:47:21.0597763Z -   --> $DIR/typeck_type_placeholder_item.rs:157:16
2020-03-24T20:47:21.0598414Z +   --> $DIR/typeck_type_placeholder_item.rs:155:16
2020-03-24T20:47:21.0598660Z 9    |
2020-03-24T20:47:21.0598824Z 10 LL | trait BadTrait<_> {}
2020-03-24T20:47:21.0599303Z 
2020-03-24T20:47:21.0599429Z 12 
2020-03-24T20:47:21.0599835Z 13 error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0600447Z -   --> $DIR/typeck_type_placeholder_item.rs:167:19
2020-03-24T20:47:21.0600447Z -   --> $DIR/typeck_type_placeholder_item.rs:167:19
2020-03-24T20:47:21.0600937Z +   --> $DIR/typeck_type_placeholder_item.rs:165:19
2020-03-24T20:47:21.0601156Z 15    |
2020-03-24T20:47:21.0601336Z 16 LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0601848Z 
2020-03-24T20:47:21.0601960Z 18 
2020-03-24T20:47:21.0602176Z 19 error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0602667Z -   --> $DIR/typeck_type_placeholder_item.rs:167:22
2020-03-24T20:47:21.0602667Z -   --> $DIR/typeck_type_placeholder_item.rs:167:22
2020-03-24T20:47:21.0603136Z +   --> $DIR/typeck_type_placeholder_item.rs:165:22
2020-03-24T20:47:21.0603354Z 21    |
2020-03-24T20:47:21.0603548Z 22 LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0604193Z 
2020-03-24T20:47:21.0604327Z 24 
2020-03-24T20:47:21.0604550Z 25 error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0605052Z -   --> $DIR/typeck_type_placeholder_item.rs:172:19
2020-03-24T20:47:21.0605052Z -   --> $DIR/typeck_type_placeholder_item.rs:172:19
2020-03-24T20:47:21.0605537Z +   --> $DIR/typeck_type_placeholder_item.rs:170:19
2020-03-24T20:47:21.0609297Z 27    |
2020-03-24T20:47:21.0609493Z 28 LL | struct BadStruct2<_, T>(_, T);
2020-03-24T20:47:21.0610022Z 
2020-03-24T20:47:21.0610134Z 30 
2020-03-24T20:47:21.0612761Z 31 error: associated constant in `impl` without body
2020-03-24T20:47:21.0613437Z -   --> $DIR/typeck_type_placeholder_item.rs:203:5
2020-03-24T20:47:21.0613437Z -   --> $DIR/typeck_type_placeholder_item.rs:203:5
2020-03-24T20:47:21.0613908Z +   --> $DIR/typeck_type_placeholder_item.rs:201:5
2020-03-24T20:47:21.0614126Z 33    |
2020-03-24T20:47:21.0614311Z 34 LL |     const C: _;
2020-03-24T20:47:21.0614655Z 35    |     ^^^^^^^^^^-
2020-03-24T20:47:21.0614787Z 
2020-03-24T20:47:21.0615061Z 37    |               help: provide a definition for the constant: `= <expr>;`
2020-03-24T20:47:21.0615368Z 38 
2020-03-24T20:47:21.0615874Z 39 error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
2020-03-24T20:47:21.0616918Z +   --> $DIR/typeck_type_placeholder_item.rs:165:22
2020-03-24T20:47:21.0617136Z 41    |
2020-03-24T20:47:21.0617136Z 41    |
2020-03-24T20:47:21.0617316Z 42 LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0617730Z 43    |                   -  ^ already used
2020-03-24T20:47:21.0618061Z 143    |         ^^^             ^
2020-03-24T20:47:21.0618245Z 144 
2020-03-24T20:47:21.0618519Z 145 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0619047Z -   --> $DIR/typeck_type_placeholder_item.rs:47:26
2020-03-24T20:47:21.0619047Z -   --> $DIR/typeck_type_placeholder_item.rs:47:26
2020-03-24T20:47:21.0619529Z +   --> $DIR/typeck_type_placeholder_item.rs:46:26
2020-03-24T20:47:21.0619750Z 147    |
2020-03-24T20:47:21.0620101Z 148 LL | fn test11(x: &usize) -> &_ {
2020-03-24T20:47:21.0620501Z 149    |                         -^
2020-03-24T20:47:21.0620667Z 
2020-03-24T20:47:21.0620955Z 152    |                         help: replace with the correct return type: `&&usize`
2020-03-24T20:47:21.0621532Z 154 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0622057Z -   --> $DIR/typeck_type_placeholder_item.rs:52:52
2020-03-24T20:47:21.0622520Z +   --> $DIR/typeck_type_placeholder_item.rs:51:52
2020-03-24T20:47:21.0622752Z 156    |
2020-03-24T20:47:21.0622752Z 156    |
2020-03-24T20:47:21.0623171Z 157 LL | unsafe fn test12(x: *const usize) -> *const *const _ {
2020-03-24T20:47:21.0623861Z 
2020-03-24T20:47:21.0623861Z 
2020-03-24T20:47:21.0624208Z 161    |                                      help: replace with the correct return type: `*const *const usize`
2020-03-24T20:47:21.0624968Z 163 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0625583Z -   --> $DIR/typeck_type_placeholder_item.rs:66:8
2020-03-24T20:47:21.0626043Z +   --> $DIR/typeck_type_placeholder_item.rs:65:8
2020-03-24T20:47:21.0626260Z 165    |
---
2020-03-24T20:47:21.0629391Z 188 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0629849Z -   --> $DIR/typeck_type_placeholder_item.rs:74:15
2020-03-24T20:47:21.0630253Z +   --> $DIR/typeck_type_placeholder_item.rs:73:15
2020-03-24T20:47:21.0630457Z 190    |
2020-03-24T20:47:21.0630616Z 191 LL |     static B: _ = 42;
2020-03-24T20:47:21.0630914Z 
2020-03-24T20:47:21.0631148Z 195    |               help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0631370Z 196 
2020-03-24T20:47:21.0631606Z 197 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
2020-03-24T20:47:21.0633652Z 203 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0634106Z -   --> $DIR/typeck_type_placeholder_item.rs:79:21
2020-03-24T20:47:21.0634510Z +   --> $DIR/typeck_type_placeholder_item.rs:78:21
2020-03-24T20:47:21.0634714Z 205    |
2020-03-24T20:47:21.0635005Z 206 LL |     fn fn_test() -> _ { 5 }
2020-03-24T20:47:21.0635335Z 
2020-03-24T20:47:21.0635572Z 210    |                     help: replace with the correct return type: `i32`
2020-03-24T20:47:21.0635809Z 211 
2020-03-24T20:47:21.0636057Z 212 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0636057Z 212 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0636514Z -   --> $DIR/typeck_type_placeholder_item.rs:82:23
2020-03-24T20:47:21.0636918Z +   --> $DIR/typeck_type_placeholder_item.rs:81:23
2020-03-24T20:47:21.0637108Z 214    |
2020-03-24T20:47:21.0637441Z 215 LL |     fn fn_test2() -> (_, _) { (5, 5) }
2020-03-24T20:47:21.0637792Z 216    |                      -^--^-
2020-03-24T20:47:21.0637926Z 
2020-03-24T20:47:21.0638187Z 220    |                      help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0638671Z 222 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0639140Z -   --> $DIR/typeck_type_placeholder_item.rs:85:22
2020-03-24T20:47:21.0639545Z +   --> $DIR/typeck_type_placeholder_item.rs:84:22
2020-03-24T20:47:21.0639735Z 224    |
2020-03-24T20:47:21.0639735Z 224    |
2020-03-24T20:47:21.0639924Z 225 LL |     static FN_TEST3: _ = "test";
2020-03-24T20:47:21.0640259Z 
2020-03-24T20:47:21.0640494Z 229    |                      help: replace `_` with the correct type: `&str`
2020-03-24T20:47:21.0640743Z 230 
2020-03-24T20:47:21.0640977Z 231 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0640977Z 231 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0641742Z -   --> $DIR/typeck_type_placeholder_item.rs:88:22
2020-03-24T20:47:21.0642237Z +   --> $DIR/typeck_type_placeholder_item.rs:87:22
2020-03-24T20:47:21.0642455Z 233    |
2020-03-24T20:47:21.0642654Z 234 LL |     static FN_TEST4: _ = 145;
2020-03-24T20:47:21.0643047Z 
2020-03-24T20:47:21.0643313Z 238    |                      help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0643583Z 239 
2020-03-24T20:47:21.0643866Z 240 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0643866Z 240 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0644387Z -   --> $DIR/typeck_type_placeholder_item.rs:91:22
2020-03-24T20:47:21.0644848Z +   --> $DIR/typeck_type_placeholder_item.rs:90:22
2020-03-24T20:47:21.0645081Z 242    |
2020-03-24T20:47:21.0645293Z 243 LL |     static FN_TEST5: (_, _) = (1, 2);
2020-03-24T20:47:21.0645816Z 
2020-03-24T20:47:21.0645937Z 245 
2020-03-24T20:47:21.0646207Z 246 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0646743Z -   --> $DIR/typeck_type_placeholder_item.rs:94:20
2020-03-24T20:47:21.0646743Z -   --> $DIR/typeck_type_placeholder_item.rs:94:20
2020-03-24T20:47:21.0647208Z +   --> $DIR/typeck_type_placeholder_item.rs:93:20
2020-03-24T20:47:21.0647425Z 248    |
2020-03-24T20:47:21.0647633Z 249 LL |     fn fn_test6(_: _) { }
2020-03-24T20:47:21.0648099Z 
2020-03-24T20:47:21.0648260Z 255    |                ^^^    ^
2020-03-24T20:47:21.0648441Z 256 
2020-03-24T20:47:21.0648709Z 257 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0648709Z 257 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0649231Z -   --> $DIR/typeck_type_placeholder_item.rs:97:20
2020-03-24T20:47:21.0649709Z +   --> $DIR/typeck_type_placeholder_item.rs:96:20
2020-03-24T20:47:21.0649927Z 259    |
2020-03-24T20:47:21.0650157Z 260 LL |     fn fn_test7(x: _) { let _x: usize = x; }
2020-03-24T20:47:21.0650681Z 
2020-03-24T20:47:21.0650843Z 266    |                ^^^    ^
2020-03-24T20:47:21.0651208Z 267 
2020-03-24T20:47:21.0651492Z 268 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0651492Z 268 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0652037Z -   --> $DIR/typeck_type_placeholder_item.rs:100:29
2020-03-24T20:47:21.0652505Z +   --> $DIR/typeck_type_placeholder_item.rs:99:29
2020-03-24T20:47:21.0652739Z 270    |
2020-03-24T20:47:21.0653104Z 271 LL |     fn fn_test8(_f: fn() -> _) { }
2020-03-24T20:47:21.0653633Z 
2020-03-24T20:47:21.0653806Z 277    |                ^^^             ^
2020-03-24T20:47:21.0653986Z 278 
2020-03-24T20:47:21.0654255Z 279 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
2020-03-24T20:47:21.0656545Z 298 error[E0282]: type annotations needed
2020-03-24T20:47:21.0657008Z -   --> $DIR/typeck_type_placeholder_item.rs:128:18
2020-03-24T20:47:21.0657480Z +   --> $DIR/typeck_type_placeholder_item.rs:126:18
2020-03-24T20:47:21.0658085Z 300    |
2020-03-24T20:47:21.0658764Z 301 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-24T20:47:21.0659246Z 
2020-03-24T20:47:21.0659361Z 303 
2020-03-24T20:47:21.0659647Z 304 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0663717Z -   --> $DIR/typeck_type_placeholder_item.rs:128:28
2020-03-24T20:47:21.0663717Z -   --> $DIR/typeck_type_placeholder_item.rs:128:28
2020-03-24T20:47:21.0664348Z +   --> $DIR/typeck_type_placeholder_item.rs:126:28
2020-03-24T20:47:21.0664566Z 306    |
2020-03-24T20:47:21.0664915Z 307 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-24T20:47:21.0665381Z 
2020-03-24T20:47:21.0665586Z 310    |                            not allowed in type signatures
2020-03-24T20:47:21.0665778Z 311 
2020-03-24T20:47:21.0666016Z 312 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0666016Z 312 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0666490Z -   --> $DIR/typeck_type_placeholder_item.rs:132:30
2020-03-24T20:47:21.0666900Z +   --> $DIR/typeck_type_placeholder_item.rs:130:30
2020-03-24T20:47:21.0667090Z 314    |
2020-03-24T20:47:21.0667447Z 315 LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
2020-03-24T20:47:21.0667833Z 316    |                             -^--^-
2020-03-24T20:47:21.0667977Z 
2020-03-24T20:47:21.0668258Z 320    |                             help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0669325Z 322 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0669857Z -   --> $DIR/typeck_type_placeholder_item.rs:135:33
2020-03-24T20:47:21.0670269Z +   --> $DIR/typeck_type_placeholder_item.rs:133:33
2020-03-24T20:47:21.0670460Z 324    |
2020-03-24T20:47:21.0670460Z 324    |
2020-03-24T20:47:21.0670802Z 325 LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
2020-03-24T20:47:21.0671342Z 
2020-03-24T20:47:21.0671342Z 
2020-03-24T20:47:21.0671598Z 329    |                           help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0672337Z 331 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0672861Z -   --> $DIR/typeck_type_placeholder_item.rs:154:21
2020-03-24T20:47:21.0673351Z +   --> $DIR/typeck_type_placeholder_item.rs:152:21
2020-03-24T20:47:21.0673574Z 333    |
2020-03-24T20:47:21.0673574Z 333    |
2020-03-24T20:47:21.0673750Z 334 LL | struct BadStruct<_>(_);
2020-03-24T20:47:21.0674216Z 
2020-03-24T20:47:21.0674372Z 340    |                  ^  ^
2020-03-24T20:47:21.0674535Z 341 
2020-03-24T20:47:21.0674820Z 342 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0674820Z 342 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0678762Z -   --> $DIR/typeck_type_placeholder_item.rs:159:15
2020-03-24T20:47:21.0679409Z +   --> $DIR/typeck_type_placeholder_item.rs:157:15
2020-03-24T20:47:21.0679655Z 344    |
2020-03-24T20:47:21.0679852Z 345 LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-24T20:47:21.0688684Z 
2020-03-24T20:47:21.0688887Z 353    |     ^^^          ^                ^
2020-03-24T20:47:21.0689091Z 354 
2020-03-24T20:47:21.0689408Z 355 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0689408Z 355 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0690562Z -   --> $DIR/typeck_type_placeholder_item.rs:162:34
2020-03-24T20:47:21.0691377Z +   --> $DIR/typeck_type_placeholder_item.rs:160:34
2020-03-24T20:47:21.0691622Z 357    |
2020-03-24T20:47:21.0692045Z 358 LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-24T20:47:21.0692608Z 
2020-03-24T20:47:21.0692749Z 360 
2020-03-24T20:47:21.0693297Z 361 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0693883Z -   --> $DIR/typeck_type_placeholder_item.rs:167:25
2020-03-24T20:47:21.0693883Z -   --> $DIR/typeck_type_placeholder_item.rs:167:25
2020-03-24T20:47:21.0694406Z +   --> $DIR/typeck_type_placeholder_item.rs:165:25
2020-03-24T20:47:21.0694646Z 363    |
2020-03-24T20:47:21.0694843Z 364 LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0695595Z 
2020-03-24T20:47:21.0695780Z 370    |                   ^     ^
2020-03-24T20:47:21.0695963Z 371 
2020-03-24T20:47:21.0696272Z 372 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0696272Z 372 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0696857Z -   --> $DIR/typeck_type_placeholder_item.rs:172:25
2020-03-24T20:47:21.0697455Z +   --> $DIR/typeck_type_placeholder_item.rs:170:25
2020-03-24T20:47:21.0697690Z 374    |
2020-03-24T20:47:21.0697878Z 375 LL | struct BadStruct2<_, T>(_, T);
2020-03-24T20:47:21.0698370Z 
2020-03-24T20:47:21.0698533Z 381    |                   ^     ^
2020-03-24T20:47:21.0698701Z 382 
2020-03-24T20:47:21.0698972Z 383 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0698972Z 383 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0699513Z -   --> $DIR/typeck_type_placeholder_item.rs:176:14
2020-03-24T20:47:21.0699982Z +   --> $DIR/typeck_type_placeholder_item.rs:174:14
2020-03-24T20:47:21.0700209Z 385    |
2020-03-24T20:47:21.0700389Z 386 LL | type X = Box<_>;
2020-03-24T20:47:21.0700811Z 
2020-03-24T20:47:21.0700937Z 388 
2020-03-24T20:47:21.0701205Z 389 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0701728Z -   --> $DIR/typeck_type_placeholder_item.rs:43:27
2020-03-24T20:47:21.0701728Z -   --> $DIR/typeck_type_placeholder_item.rs:43:27
2020-03-24T20:47:21.0702206Z +   --> $DIR/typeck_type_placeholder_item.rs:42:27
2020-03-24T20:47:21.0702423Z 391    |
2020-03-24T20:47:21.0702739Z 392 LL |     fn test10(&self, _x : _) { }
2020-03-24T20:47:21.0703195Z 
2020-03-24T20:47:21.0703344Z 398    |              ^^^             ^
2020-03-24T20:47:21.0703498Z 399 
2020-03-24T20:47:21.0703746Z 400 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0703746Z 400 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0704208Z -   --> $DIR/typeck_type_placeholder_item.rs:140:31
2020-03-24T20:47:21.0704620Z +   --> $DIR/typeck_type_placeholder_item.rs:138:31
2020-03-24T20:47:21.0704824Z 402    |
2020-03-24T20:47:21.0705006Z 403 LL |     fn method_test1(&self, x: _);
2020-03-24T20:47:21.0705458Z 
2020-03-24T20:47:21.0705630Z 409    |                    ^^^           ^
2020-03-24T20:47:21.0705791Z 410 
2020-03-24T20:47:21.0706026Z 411 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0706026Z 411 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0706497Z -   --> $DIR/typeck_type_placeholder_item.rs:142:31
2020-03-24T20:47:21.0706906Z +   --> $DIR/typeck_type_placeholder_item.rs:140:31
2020-03-24T20:47:21.0707098Z 413    |
2020-03-24T20:47:21.0707443Z 414 LL |     fn method_test2(&self, x: _) -> _;
2020-03-24T20:47:21.0707919Z 
2020-03-24T20:47:21.0708100Z 422    |                    ^^^           ^     ^
2020-03-24T20:47:21.0708272Z 423 
2020-03-24T20:47:21.0708507Z 424 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
2020-03-24T20:47:21.0710894Z 435 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0711353Z -   --> $DIR/typeck_type_placeholder_item.rs:146:26
2020-03-24T20:47:21.0711776Z +   --> $DIR/typeck_type_placeholder_item.rs:144:26
2020-03-24T20:47:21.0711968Z 437    |
2020-03-24T20:47:21.0712211Z 438 LL |     fn assoc_fn_test1(x: _);
2020-03-24T20:47:21.0712658Z 
2020-03-24T20:47:21.0712809Z 444    |                      ^^^    ^
2020-03-24T20:47:21.0712961Z 445 
2020-03-24T20:47:21.0713210Z 446 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0713210Z 446 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0713677Z -   --> $DIR/typeck_type_placeholder_item.rs:148:26
2020-03-24T20:47:21.0714082Z +   --> $DIR/typeck_type_placeholder_item.rs:146:26
2020-03-24T20:47:21.0714289Z 448    |
2020-03-24T20:47:21.0714606Z 449 LL |     fn assoc_fn_test2(x: _) -> _;
2020-03-24T20:47:21.0715073Z 
2020-03-24T20:47:21.0715233Z 457    |                      ^^^    ^     ^
2020-03-24T20:47:21.0715392Z 458 
2020-03-24T20:47:21.0715629Z 459 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
2020-03-24T20:47:21.0717985Z 470 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0718451Z -   --> $DIR/typeck_type_placeholder_item.rs:61:37
2020-03-24T20:47:21.0718856Z +   --> $DIR/typeck_type_placeholder_item.rs:60:37
2020-03-24T20:47:21.0719047Z 472    |
2020-03-24T20:47:21.0719279Z 473 LL |     fn clone_from(&mut self, other: _) { *self = Test9; }
2020-03-24T20:47:21.0719790Z 
2020-03-24T20:47:21.0719955Z 479    |                  ^^^                   ^
2020-03-24T20:47:21.0720137Z 480 
2020-03-24T20:47:21.0720372Z 481 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0720372Z 481 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0720829Z -   --> $DIR/typeck_type_placeholder_item.rs:110:34
2020-03-24T20:47:21.0721253Z +   --> $DIR/typeck_type_placeholder_item.rs:108:34
2020-03-24T20:47:21.0721443Z 483    |
2020-03-24T20:47:21.0721636Z 484 LL |         fn fn_test10(&self, _x : _) { }
2020-03-24T20:47:21.0722118Z 
2020-03-24T20:47:21.0722279Z 490    |                     ^^^             ^
2020-03-24T20:47:21.0722442Z 491 
2020-03-24T20:47:21.0722691Z 492 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0722691Z 492 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0723148Z -   --> $DIR/typeck_type_placeholder_item.rs:118:41
2020-03-24T20:47:21.0723561Z +   --> $DIR/typeck_type_placeholder_item.rs:116:41
2020-03-24T20:47:21.0723767Z 494    |
2020-03-24T20:47:21.0723995Z 495 LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }
2020-03-24T20:47:21.0724535Z 
2020-03-24T20:47:21.0724706Z 501    |                      ^^^                   ^
2020-03-24T20:47:21.0724880Z 502 
2020-03-24T20:47:21.0725128Z 503 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0725128Z 503 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0725586Z -   --> $DIR/typeck_type_placeholder_item.rs:182:21
2020-03-24T20:47:21.0725993Z +   --> $DIR/typeck_type_placeholder_item.rs:180:21
2020-03-24T20:47:21.0726196Z 505    |
2020-03-24T20:47:21.0726349Z 506 LL | type Y = impl Trait<_>;
2020-03-24T20:47:21.0726743Z 
2020-03-24T20:47:21.0726856Z 508 
2020-03-24T20:47:21.0727125Z 509 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0727619Z -   --> $DIR/typeck_type_placeholder_item.rs:190:14
---
2020-03-24T20:47:21.0731137Z 521 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0732480Z -   --> $DIR/typeck_type_placeholder_item.rs:194:14
2020-03-24T20:47:21.0732980Z +   --> $DIR/typeck_type_placeholder_item.rs:192:14
2020-03-24T20:47:21.0733202Z 523    |
2020-03-24T20:47:21.0733382Z 524 LL |     const D: _ = 42;
2020-03-24T20:47:21.0733733Z 
2020-03-24T20:47:21.0733985Z 528    |              help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0734239Z 529 
2020-03-24T20:47:21.0734883Z 530 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0734883Z 530 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0735461Z -   --> $DIR/typeck_type_placeholder_item.rs:40:24
2020-03-24T20:47:21.0735924Z +   --> $DIR/typeck_type_placeholder_item.rs:39:24
2020-03-24T20:47:21.0736158Z 532    |
2020-03-24T20:47:21.0736504Z 533 LL |     fn test9(&self) -> _ { () }
2020-03-24T20:47:21.0736894Z 
2020-03-24T20:47:21.0737183Z 537    |                        help: replace with the correct return type: `()`
2020-03-24T20:47:21.0737460Z 538 
2020-03-24T20:47:21.0737730Z 539 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0737730Z 539 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0738460Z -   --> $DIR/typeck_type_placeholder_item.rs:58:24
2020-03-24T20:47:21.0738945Z +   --> $DIR/typeck_type_placeholder_item.rs:57:24
2020-03-24T20:47:21.0739339Z 541    |
2020-03-24T20:47:21.0739735Z 542 LL |     fn clone(&self) -> _ { Test9 }
2020-03-24T20:47:21.0740290Z 
2020-03-24T20:47:21.0740587Z 546    |                        help: replace with the correct return type: `Test9`
2020-03-24T20:47:21.0740983Z 547 
2020-03-24T20:47:21.0741276Z 548 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0741276Z 548 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0741859Z -   --> $DIR/typeck_type_placeholder_item.rs:107:31
2020-03-24T20:47:21.0742391Z +   --> $DIR/typeck_type_placeholder_item.rs:105:31
2020-03-24T20:47:21.0742629Z 550    |
2020-03-24T20:47:21.0743017Z 551 LL |         fn fn_test9(&self) -> _ { () }
2020-03-24T20:47:21.0743476Z 
2020-03-24T20:47:21.0743787Z 555    |                               help: replace with the correct return type: `()`
2020-03-24T20:47:21.0744114Z 556 
2020-03-24T20:47:21.0744404Z 557 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0744404Z 557 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0744972Z -   --> $DIR/typeck_type_placeholder_item.rs:115:28
2020-03-24T20:47:21.0745599Z +   --> $DIR/typeck_type_placeholder_item.rs:113:28
2020-03-24T20:47:21.0745818Z 559    |
2020-03-24T20:47:21.0746186Z 560 LL |         fn clone(&self) -> _ { FnTest9 }
2020-03-24T20:47:21.0746604Z 
2020-03-24T20:47:21.0747006Z 564    |                            help: replace with the correct return type: `main::FnTest9`
2020-03-24T20:47:21.0747359Z 565 
2020-03-24T20:47:21.0747640Z 566 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
2020-03-24T20:47:21.0755478Z 584 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0756043Z -   --> $DIR/typeck_type_placeholder_item.rs:206:14
2020-03-24T20:47:21.0756563Z +   --> $DIR/typeck_type_placeholder_item.rs:204:14
2020-03-24T20:47:21.0756800Z 586    |
2020-03-24T20:47:21.0756994Z 587 LL |     const D: _ = 42;
2020-03-24T20:47:21.0757513Z 
2020-03-24T20:47:21.0757615Z 
2020-03-24T20:47:21.0757858Z The actual stderr differed from the expected stderr.
2020-03-24T20:47:21.0758626Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/typeck_type_placeholder_item.stderr
2020-03-24T20:47:21.0758626Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/typeck_type_placeholder_item.stderr
2020-03-24T20:47:21.0759439Z To update references, rerun the tests and pass the `--bless` flag
2020-03-24T20:47:21.0760186Z To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`
2020-03-24T20:47:21.0760634Z error: 1 errors occurred comparing output.
2020-03-24T20:47:21.0760857Z status: exit code: 1
2020-03-24T20:47:21.0760857Z status: exit code: 1
2020-03-24T20:47:21.0762770Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/auxiliary"
2020-03-24T20:47:21.0764463Z ------------------------------------------
2020-03-24T20:47:21.0764625Z 
2020-03-24T20:47:21.0765071Z ------------------------------------------
2020-03-24T20:47:21.0765282Z stderr:
2020-03-24T20:47:21.0765282Z stderr:
2020-03-24T20:47:21.0765635Z ------------------------------------------
2020-03-24T20:47:21.0765908Z error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0766453Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:152:18
2020-03-24T20:47:21.0766705Z    |
2020-03-24T20:47:21.0766868Z LL | struct BadStruct<_>(_);
2020-03-24T20:47:21.0767445Z 
2020-03-24T20:47:21.0767649Z error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0768182Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:155:16
2020-03-24T20:47:21.0768447Z    |
2020-03-24T20:47:21.0768447Z    |
2020-03-24T20:47:21.0768604Z LL | trait BadTrait<_> {}
2020-03-24T20:47:21.0769218Z 
2020-03-24T20:47:21.0769423Z error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0770118Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:165:19
2020-03-24T20:47:21.0770508Z    |
2020-03-24T20:47:21.0770508Z    |
2020-03-24T20:47:21.0770687Z LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0771845Z 
2020-03-24T20:47:21.0772071Z error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0772750Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:165:22
2020-03-24T20:47:21.0772992Z    |
2020-03-24T20:47:21.0772992Z    |
2020-03-24T20:47:21.0773168Z LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0773648Z 
2020-03-24T20:47:21.0773851Z error: expected identifier, found reserved identifier `_`
2020-03-24T20:47:21.0774563Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:170:19
2020-03-24T20:47:21.0774937Z    |
2020-03-24T20:47:21.0774937Z    |
2020-03-24T20:47:21.0775112Z LL | struct BadStruct2<_, T>(_, T);
2020-03-24T20:47:21.0775614Z 
2020-03-24T20:47:21.0775805Z error: associated constant in `impl` without body
2020-03-24T20:47:21.0776354Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:201:5
2020-03-24T20:47:21.0776599Z    |
2020-03-24T20:47:21.0776599Z    |
2020-03-24T20:47:21.0776756Z LL |     const C: _;
2020-03-24T20:47:21.0777095Z    |     ^^^^^^^^^^-
2020-03-24T20:47:21.0777411Z    |               |
2020-03-24T20:47:21.0777704Z    |               help: provide a definition for the constant: `= <expr>;`
2020-03-24T20:47:21.0777943Z 
2020-03-24T20:47:21.0778456Z error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
2020-03-24T20:47:21.0779319Z    |
2020-03-24T20:47:21.0779319Z    |
2020-03-24T20:47:21.0779486Z LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0779867Z    |                   -  ^ already used
2020-03-24T20:47:21.0780309Z    |                   first use of `_`
2020-03-24T20:47:21.0780464Z 
2020-03-24T20:47:21.0780719Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0781300Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:6:14
---
2020-03-24T20:47:21.0783011Z 
2020-03-24T20:47:21.0783265Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0783964Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:9:16
2020-03-24T20:47:21.0784226Z    |
2020-03-24T20:47:21.0784556Z LL | fn test2() -> (_, _) { (5, 5) }
2020-03-24T20:47:21.0784908Z    |               -^--^-
2020-03-24T20:47:21.0785114Z    |               ||  |
2020-03-24T20:47:21.0785349Z    |               ||  not allowed in type signatures
2020-03-24T20:47:21.0785626Z    |               |not allowed in type signatures
2020-03-24T20:47:21.0785982Z    |               help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0786638Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0787208Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:12:15
2020-03-24T20:47:21.0787472Z    |
2020-03-24T20:47:21.0787472Z    |
2020-03-24T20:47:21.0787647Z LL | static TEST3: _ = "test";
2020-03-24T20:47:21.0788036Z    |               |
2020-03-24T20:47:21.0788258Z    |               not allowed in type signatures
2020-03-24T20:47:21.0788583Z    |               help: replace `_` with the correct type: `&str`
2020-03-24T20:47:21.0788823Z 
---
2020-03-24T20:47:21.0792957Z 
2020-03-24T20:47:21.0793208Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0793794Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:21:13
2020-03-24T20:47:21.0794040Z    |
2020-03-24T20:47:21.0794204Z LL | fn test6(_: _) { }
2020-03-24T20:47:21.0794656Z    |
2020-03-24T20:47:21.0794929Z help: use type parameters instead
2020-03-24T20:47:21.0795091Z    |
2020-03-24T20:47:21.0795237Z LL | fn test6<T>(_: T) { }
2020-03-24T20:47:21.0795237Z LL | fn test6<T>(_: T) { }
2020-03-24T20:47:21.0795408Z    |         ^^^    ^
2020-03-24T20:47:21.0795521Z 
2020-03-24T20:47:21.0795753Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0796250Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:24:18
2020-03-24T20:47:21.0796464Z    |
2020-03-24T20:47:21.0796641Z LL | fn test6_b<T>(_: _, _: T) { }
2020-03-24T20:47:21.0797050Z    |
2020-03-24T20:47:21.0797212Z help: use type parameters instead
2020-03-24T20:47:21.0797359Z    |
2020-03-24T20:47:21.0797359Z    |
2020-03-24T20:47:21.0797527Z LL | fn test6_b<T, K>(_: K, _: T) { }
2020-03-24T20:47:21.0797858Z 
2020-03-24T20:47:21.0798084Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0798583Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:27:30
2020-03-24T20:47:21.0798811Z    |
2020-03-24T20:47:21.0798811Z    |
2020-03-24T20:47:21.0799016Z LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
2020-03-24T20:47:21.0799515Z    |
2020-03-24T20:47:21.0799664Z help: use type parameters instead
2020-03-24T20:47:21.0799813Z    |
2020-03-24T20:47:21.0799813Z    |
2020-03-24T20:47:21.0800034Z LL | fn test6_c<T, K, L, A, B, C>(_: C, _: (T, K, L, A, B)) { }
2020-03-24T20:47:21.0800424Z 
2020-03-24T20:47:21.0800875Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0801449Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:30:13
2020-03-24T20:47:21.0801834Z    |
2020-03-24T20:47:21.0801834Z    |
2020-03-24T20:47:21.0802036Z LL | fn test7(x: _) { let _x: usize = x; }
2020-03-24T20:47:21.0802621Z    |
2020-03-24T20:47:21.0802790Z help: use type parameters instead
2020-03-24T20:47:21.0802974Z    |
2020-03-24T20:47:21.0802974Z    |
2020-03-24T20:47:21.0803180Z LL | fn test7<T>(x: T) { let _x: usize = x; }
2020-03-24T20:47:21.0803553Z 
2020-03-24T20:47:21.0803806Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0804394Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:33:22
2020-03-24T20:47:21.0804653Z    |
2020-03-24T20:47:21.0804653Z    |
2020-03-24T20:47:21.0804981Z LL | fn test8(_f: fn() -> _) { }
2020-03-24T20:47:21.0805457Z    |
2020-03-24T20:47:21.0805643Z help: use type parameters instead
2020-03-24T20:47:21.0805813Z    |
2020-03-24T20:47:21.0805813Z    |
2020-03-24T20:47:21.0806147Z LL | fn test8<T>(_f: fn() -> T) { }
2020-03-24T20:47:21.0806537Z 
2020-03-24T20:47:21.0806792Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0807378Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:46:26
2020-03-24T20:47:21.0807625Z    |
2020-03-24T20:47:21.0807625Z    |
2020-03-24T20:47:21.0807951Z LL | fn test11(x: &usize) -> &_ {
2020-03-24T20:47:21.0808320Z    |                         -^
2020-03-24T20:47:21.0808807Z    |                         |not allowed in type signatures
2020-03-24T20:47:21.0808807Z    |                         |not allowed in type signatures
2020-03-24T20:47:21.0809181Z    |                         help: replace with the correct return type: `&&usize`
2020-03-24T20:47:21.0809707Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0810275Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:51:52
2020-03-24T20:47:21.0810537Z    |
2020-03-24T20:47:21.0810537Z    |
2020-03-24T20:47:21.0810936Z LL | unsafe fn test12(x: *const usize) -> *const *const _ {
2020-03-24T20:47:21.0811896Z    |                                      |             |
2020-03-24T20:47:21.0812236Z    |                                      |             not allowed in type signatures
2020-03-24T20:47:21.0812236Z    |                                      |             not allowed in type signatures
2020-03-24T20:47:21.0812698Z    |                                      help: replace with the correct return type: `*const *const usize`
2020-03-24T20:47:21.0813320Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0813817Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:65:8
2020-03-24T20:47:21.0814043Z    |
2020-03-24T20:47:21.0814169Z LL |     a: _,
2020-03-24T20:47:21.0814169Z LL |     a: _,
2020-03-24T20:47:21.0814354Z    |        ^ not allowed in type signatures
2020-03-24T20:47:21.0814658Z LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0814917Z LL |     b: (_, _),
2020-03-24T20:47:21.0815304Z    |         |
2020-03-24T20:47:21.0815494Z    |         not allowed in type signatures
2020-03-24T20:47:21.0815656Z    |
2020-03-24T20:47:21.0815806Z help: use type parameters instead
2020-03-24T20:47:21.0815806Z help: use type parameters instead
2020-03-24T20:47:21.0815969Z    |
2020-03-24T20:47:21.0816103Z LL | struct Test10<T> {
2020-03-24T20:47:21.0816254Z LL |     a: T,
2020-03-24T20:47:21.0816524Z LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0816778Z LL |     b: (T, T),
2020-03-24T20:47:21.0816996Z 
2020-03-24T20:47:21.0817158Z error: missing type for `static` item
2020-03-24T20:47:21.0817590Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:71:12
2020-03-24T20:47:21.0817803Z    |
2020-03-24T20:47:21.0817803Z    |
2020-03-24T20:47:21.0817950Z LL |     static A = 42;
2020-03-24T20:47:21.0818188Z    |            ^ help: provide a type for the item: `A: i32`
2020-03-24T20:47:21.0818378Z 
2020-03-24T20:47:21.0818708Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0819216Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:73:15
2020-03-24T20:47:21.0819431Z    |
2020-03-24T20:47:21.0819591Z LL |     static B: _ = 42;
2020-03-24T20:47:21.0819911Z    |               |
2020-03-24T20:47:21.0820105Z    |               not allowed in type signatures
2020-03-24T20:47:21.0820402Z    |               help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0820596Z 
---
2020-03-24T20:47:21.0822140Z 
2020-03-24T20:47:21.0822360Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0822856Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:78:21
2020-03-24T20:47:21.0823085Z    |
2020-03-24T20:47:21.0823362Z LL |     fn fn_test() -> _ { 5 }
2020-03-24T20:47:21.0823726Z    |                     |
2020-03-24T20:47:21.0823935Z    |                     not allowed in type signatures
2020-03-24T20:47:21.0824241Z    |                     help: replace with the correct return type: `i32`
2020-03-24T20:47:21.0824465Z 
2020-03-24T20:47:21.0824465Z 
2020-03-24T20:47:21.0824686Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0825178Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:81:23
2020-03-24T20:47:21.0825406Z    |
2020-03-24T20:47:21.0825706Z LL |     fn fn_test2() -> (_, _) { (5, 5) }
2020-03-24T20:47:21.0826038Z    |                      -^--^-
2020-03-24T20:47:21.0826228Z    |                      ||  |
2020-03-24T20:47:21.0826467Z    |                      ||  not allowed in type signatures
2020-03-24T20:47:21.0826728Z    |                      |not allowed in type signatures
2020-03-24T20:47:21.0827046Z    |                      help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0827505Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0827999Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:84:22
2020-03-24T20:47:21.0828227Z    |
2020-03-24T20:47:21.0828227Z    |
2020-03-24T20:47:21.0828391Z LL |     static FN_TEST3: _ = "test";
2020-03-24T20:47:21.0828770Z    |                      |
2020-03-24T20:47:21.0828982Z    |                      not allowed in type signatures
2020-03-24T20:47:21.0829286Z    |                      help: replace `_` with the correct type: `&str`
2020-03-24T20:47:21.0829502Z 
2020-03-24T20:47:21.0829502Z 
2020-03-24T20:47:21.0829739Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0830235Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:87:22
2020-03-24T20:47:21.0830449Z    |
2020-03-24T20:47:21.0830623Z LL |     static FN_TEST4: _ = 145;
2020-03-24T20:47:21.0830984Z    |                      |
2020-03-24T20:47:21.0831209Z    |                      not allowed in type signatures
2020-03-24T20:47:21.0831512Z    |                      help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0831718Z 
2020-03-24T20:47:21.0831718Z 
2020-03-24T20:47:21.0831951Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0832443Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:90:22
2020-03-24T20:47:21.0832656Z    |
2020-03-24T20:47:21.0832841Z LL |     static FN_TEST5: (_, _) = (1, 2);
2020-03-24T20:47:21.0833344Z 
2020-03-24T20:47:21.0833563Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0834229Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:93:20
2020-03-24T20:47:21.0834453Z    |
2020-03-24T20:47:21.0834453Z    |
2020-03-24T20:47:21.0834607Z LL |     fn fn_test6(_: _) { }
2020-03-24T20:47:21.0835028Z    |
2020-03-24T20:47:21.0835177Z help: use type parameters instead
2020-03-24T20:47:21.0835339Z    |
2020-03-24T20:47:21.0835339Z    |
2020-03-24T20:47:21.0835498Z LL |     fn fn_test6<T>(_: T) { }
2020-03-24T20:47:21.0835810Z 
2020-03-24T20:47:21.0836045Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0836569Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:96:20
2020-03-24T20:47:21.0836790Z    |
2020-03-24T20:47:21.0836790Z    |
2020-03-24T20:47:21.0836995Z LL |     fn fn_test7(x: _) { let _x: usize = x; }
2020-03-24T20:47:21.0842249Z    |
2020-03-24T20:47:21.0842459Z help: use type parameters instead
2020-03-24T20:47:21.0842642Z    |
2020-03-24T20:47:21.0842642Z    |
2020-03-24T20:47:21.0842882Z LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
2020-03-24T20:47:21.0843328Z 
2020-03-24T20:47:21.0843604Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0844378Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:99:29
2020-03-24T20:47:21.0844662Z    |
2020-03-24T20:47:21.0844662Z    |
2020-03-24T20:47:21.0845046Z LL |     fn fn_test8(_f: fn() -> _) { }
2020-03-24T20:47:21.0845617Z    |
2020-03-24T20:47:21.0845803Z help: use type parameters instead
2020-03-24T20:47:21.0845987Z    |
2020-03-24T20:47:21.0845987Z    |
2020-03-24T20:47:21.0846402Z LL |     fn fn_test8<T>(_f: fn() -> T) { }
2020-03-24T20:47:21.0846841Z 
2020-03-24T20:47:21.0847118Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0847756Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:121:12
2020-03-24T20:47:21.0848025Z    |
2020-03-24T20:47:21.0848025Z    |
2020-03-24T20:47:21.0848320Z LL |         a: _,
2020-03-24T20:47:21.0848583Z    |            ^ not allowed in type signatures
2020-03-24T20:47:21.0848954Z LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0849288Z LL |         b: (_, _),
2020-03-24T20:47:21.0849801Z    |             |
2020-03-24T20:47:21.0850035Z    |             not allowed in type signatures
2020-03-24T20:47:21.0850254Z    |
2020-03-24T20:47:21.0850439Z help: use type parameters instead
2020-03-24T20:47:21.0850439Z help: use type parameters instead
2020-03-24T20:47:21.0850630Z    |
2020-03-24T20:47:21.0850820Z LL |     struct FnTest10<T> {
2020-03-24T20:47:21.0851857Z LL |         a: T,
2020-03-24T20:47:21.0852185Z LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0852626Z LL |         b: (T, T),
2020-03-24T20:47:21.0852884Z 
2020-03-24T20:47:21.0853059Z error[E0282]: type annotations needed
2020-03-24T20:47:21.0853593Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:126:18
2020-03-24T20:47:21.0853842Z    |
2020-03-24T20:47:21.0853842Z    |
2020-03-24T20:47:21.0854221Z LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-24T20:47:21.0854668Z 
2020-03-24T20:47:21.0854923Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0855511Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:126:28
2020-03-24T20:47:21.0855954Z    |
2020-03-24T20:47:21.0855954Z    |
2020-03-24T20:47:21.0856546Z LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
2020-03-24T20:47:21.0857151Z    |                            |
2020-03-24T20:47:21.0857415Z    |                            not allowed in type signatures
2020-03-24T20:47:21.0857606Z 
2020-03-24T20:47:21.0857874Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0857874Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0858459Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:130:30
2020-03-24T20:47:21.0858708Z    |
2020-03-24T20:47:21.0859097Z LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
2020-03-24T20:47:21.0859520Z    |                             -^--^-
2020-03-24T20:47:21.0859755Z    |                             ||  |
2020-03-24T20:47:21.0860050Z    |                             ||  not allowed in type signatures
2020-03-24T20:47:21.0860373Z    |                             |not allowed in type signatures
2020-03-24T20:47:21.0860775Z    |                             help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0861314Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0861921Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:133:33
2020-03-24T20:47:21.0862170Z    |
2020-03-24T20:47:21.0862170Z    |
2020-03-24T20:47:21.0862561Z LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
2020-03-24T20:47:21.0863271Z    |                           |     |
2020-03-24T20:47:21.0863530Z    |                           |     not allowed in type signatures
2020-03-24T20:47:21.0863530Z    |                           |     not allowed in type signatures
2020-03-24T20:47:21.0863874Z    |                           help: replace with the correct return type: `(i32, i32)`
2020-03-24T20:47:21.0864340Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0864842Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:152:21
2020-03-24T20:47:21.0865061Z    |
2020-03-24T20:47:21.0865061Z    |
2020-03-24T20:47:21.0865398Z LL | struct BadStruct<_>(_);
2020-03-24T20:47:21.0865853Z    |
2020-03-24T20:47:21.0866038Z help: use type parameters instead
2020-03-24T20:47:21.0866210Z    |
2020-03-24T20:47:21.0866210Z    |
2020-03-24T20:47:21.0866370Z LL | struct BadStruct<T>(T);
2020-03-24T20:47:21.0866712Z 
2020-03-24T20:47:21.0866968Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0867584Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:157:15
2020-03-24T20:47:21.0867814Z    |
2020-03-24T20:47:21.0867814Z    |
2020-03-24T20:47:21.0867974Z LL | impl BadTrait<_> for BadStruct<_> {}
2020-03-24T20:47:21.0868457Z    |               |
2020-03-24T20:47:21.0868655Z    |               not allowed in type signatures
2020-03-24T20:47:21.0868825Z    |
2020-03-24T20:47:21.0868989Z help: use type parameters instead
2020-03-24T20:47:21.0868989Z help: use type parameters instead
2020-03-24T20:47:21.0869139Z    |
2020-03-24T20:47:21.0869301Z LL | impl<T> BadTrait<T> for BadStruct<T> {}
2020-03-24T20:47:21.0869666Z 
2020-03-24T20:47:21.0869886Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0870385Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:160:34
2020-03-24T20:47:21.0870617Z    |
2020-03-24T20:47:21.0870617Z    |
2020-03-24T20:47:21.0870913Z LL | fn impl_trait() -> impl BadTrait<_> {
2020-03-24T20:47:21.0871362Z 
2020-03-24T20:47:21.0871584Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0872080Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:165:25
2020-03-24T20:47:21.0872376Z    |
2020-03-24T20:47:21.0872376Z    |
2020-03-24T20:47:21.0872541Z LL | struct BadStruct1<_, _>(_);
2020-03-24T20:47:21.0872953Z    |
2020-03-24T20:47:21.0873116Z help: use type parameters instead
2020-03-24T20:47:21.0873265Z    |
2020-03-24T20:47:21.0873265Z    |
2020-03-24T20:47:21.0873410Z LL | struct BadStruct1<T, _>(T);
2020-03-24T20:47:21.0873728Z 
2020-03-24T20:47:21.0873946Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0874467Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:170:25
2020-03-24T20:47:21.0874684Z    |
2020-03-24T20:47:21.0874684Z    |
2020-03-24T20:47:21.0874834Z LL | struct BadStruct2<_, T>(_, T);
2020-03-24T20:47:21.0875265Z    |
2020-03-24T20:47:21.0875413Z help: use type parameters instead
2020-03-24T20:47:21.0875560Z    |
2020-03-24T20:47:21.0875560Z    |
2020-03-24T20:47:21.0875728Z LL | struct BadStruct2<K, T>(K, T);
2020-03-24T20:47:21.0876041Z 
2020-03-24T20:47:21.0876275Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0876776Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:174:14
2020-03-24T20:47:21.0876992Z    |
2020-03-24T20:47:21.0876992Z    |
2020-03-24T20:47:21.0877123Z LL | type X = Box<_>;
2020-03-24T20:47:21.0877484Z 
2020-03-24T20:47:21.0877703Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0878210Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:42:27
2020-03-24T20:47:21.0878423Z    |
2020-03-24T20:47:21.0878423Z    |
2020-03-24T20:47:21.0878591Z LL |     fn test10(&self, _x : _) { }
2020-03-24T20:47:21.0879048Z    |
2020-03-24T20:47:21.0879201Z help: use type parameters instead
2020-03-24T20:47:21.0879364Z    |
2020-03-24T20:47:21.0879364Z    |
2020-03-24T20:47:21.0879538Z LL |     fn test10<T>(&self, _x : T) { }
2020-03-24T20:47:21.0879885Z 
2020-03-24T20:47:21.0880121Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0880622Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:138:31
2020-03-24T20:47:21.0880838Z    |
2020-03-24T20:47:21.0880838Z    |
2020-03-24T20:47:21.0881020Z LL |     fn method_test1(&self, x: _);
2020-03-24T20:47:21.0881471Z    |
2020-03-24T20:47:21.0881633Z help: use type parameters instead
2020-03-24T20:47:21.0881782Z    |
2020-03-24T20:47:21.0881782Z    |
2020-03-24T20:47:21.0881956Z LL |     fn method_test1<T>(&self, x: T);
2020-03-24T20:47:21.0882326Z 
2020-03-24T20:47:21.0882549Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0883052Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:140:31
2020-03-24T20:47:21.0883284Z    |
2020-03-24T20:47:21.0883284Z    |
2020-03-24T20:47:21.0883598Z LL |     fn method_test2(&self, x: _) -> _;
2020-03-24T20:47:21.0884136Z    |                               |
2020-03-24T20:47:21.0884373Z    |                               not allowed in type signatures
2020-03-24T20:47:21.0884565Z    |
2020-03-24T20:47:21.0884729Z help: use type parameters instead
2020-03-24T20:47:21.0884729Z help: use type parameters instead
2020-03-24T20:47:21.0884876Z    |
2020-03-24T20:47:21.0885198Z LL |     fn method_test2<T>(&self, x: T) -> T;
2020-03-24T20:47:21.0885598Z 
2020-03-24T20:47:21.0885819Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0886353Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:142:31
2020-03-24T20:47:21.0886617Z    |
---
2020-03-24T20:47:21.0888482Z 
2020-03-24T20:47:21.0888736Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0889321Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:144:26
2020-03-24T20:47:21.0889569Z    |
2020-03-24T20:47:21.0889750Z LL |     fn assoc_fn_test1(x: _);
2020-03-24T20:47:21.0890257Z    |
2020-03-24T20:47:21.0890426Z help: use type parameters instead
2020-03-24T20:47:21.0890618Z    |
2020-03-24T20:47:21.0890618Z    |
2020-03-24T20:47:21.0890807Z LL |     fn assoc_fn_test1<T>(x: T);
2020-03-24T20:47:21.0891282Z 
2020-03-24T20:47:21.0891551Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0892133Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:146:26
2020-03-24T20:47:21.0892380Z    |
2020-03-24T20:47:21.0892380Z    |
2020-03-24T20:47:21.0892737Z LL |     fn assoc_fn_test2(x: _) -> _;
2020-03-24T20:47:21.0893302Z    |                          |
2020-03-24T20:47:21.0893577Z    |                          not allowed in type signatures
2020-03-24T20:47:21.0893788Z    |
2020-03-24T20:47:21.0893959Z help: use type parameters instead
2020-03-24T20:47:21.0893959Z help: use type parameters instead
2020-03-24T20:47:21.0894141Z    |
2020-03-24T20:47:21.0894495Z LL |     fn assoc_fn_test2<T>(x: T) -> T;
2020-03-24T20:47:21.0894933Z 
2020-03-24T20:47:21.0895186Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0895759Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:148:28
2020-03-24T20:47:21.0896006Z    |
---
2020-03-24T20:47:21.0897905Z 
2020-03-24T20:47:21.0898157Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0898737Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:60:37
2020-03-24T20:47:21.0898985Z    |
2020-03-24T20:47:21.0899223Z LL |     fn clone_from(&mut self, other: _) { *self = Test9; }
2020-03-24T20:47:21.0899820Z    |
2020-03-24T20:47:21.0899990Z help: use type parameters instead
2020-03-24T20:47:21.0900159Z    |
2020-03-24T20:47:21.0900159Z    |
2020-03-24T20:47:21.0900417Z LL |     fn clone_from<T>(&mut self, other: T) { *self = Test9; }
2020-03-24T20:47:21.0900890Z 
2020-03-24T20:47:21.0901155Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0901728Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:108:34
2020-03-24T20:47:21.0902073Z    |
2020-03-24T20:47:21.0902073Z    |
2020-03-24T20:47:21.0902265Z LL |         fn fn_test10(&self, _x : _) { }
2020-03-24T20:47:21.0902734Z    |
2020-03-24T20:47:21.0902896Z help: use type parameters instead
2020-03-24T20:47:21.0903147Z    |
2020-03-24T20:47:21.0903147Z    |
2020-03-24T20:47:21.0903333Z LL |         fn fn_test10<T>(&self, _x : T) { }
2020-03-24T20:47:21.0903723Z 
2020-03-24T20:47:21.0903942Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0904449Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:116:41
2020-03-24T20:47:21.0904679Z    |
2020-03-24T20:47:21.0904679Z    |
2020-03-24T20:47:21.0904893Z LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }
2020-03-24T20:47:21.0905431Z    |
2020-03-24T20:47:21.0905580Z help: use type parameters instead
2020-03-24T20:47:21.0905727Z    |
2020-03-24T20:47:21.0905727Z    |
2020-03-24T20:47:21.0905961Z LL |         fn clone_from<T>(&mut self, other: T) { *self = FnTest9; }
2020-03-24T20:47:21.0906392Z 
2020-03-24T20:47:21.0906617Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0907133Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:180:21
2020-03-24T20:47:21.0907347Z    |
2020-03-24T20:47:21.0907347Z    |
2020-03-24T20:47:21.0907488Z LL | type Y = impl Trait<_>;
2020-03-24T20:47:21.0907879Z 
2020-03-24T20:47:21.0908102Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0908614Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:188:14
2020-03-24T20:47:21.0908831Z    |
---
2020-03-24T20:47:21.0910769Z 
2020-03-24T20:47:21.0911004Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0911499Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:192:14
2020-03-24T20:47:21.0911715Z    |
2020-03-24T20:47:21.0911874Z LL |     const D: _ = 42;
2020-03-24T20:47:21.0912190Z    |              |
2020-03-24T20:47:21.0912395Z    |              not allowed in type signatures
2020-03-24T20:47:21.0912671Z    |              help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0912866Z 
2020-03-24T20:47:21.0912866Z 
2020-03-24T20:47:21.0913100Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0913596Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:39:24
2020-03-24T20:47:21.0913817Z    |
2020-03-24T20:47:21.0914115Z LL |     fn test9(&self) -> _ { () }
2020-03-24T20:47:21.0914480Z    |                        |
2020-03-24T20:47:21.0914699Z    |                        not allowed in type signatures
2020-03-24T20:47:21.0915025Z    |                        help: replace with the correct return type: `()`
2020-03-24T20:47:21.0915239Z 
2020-03-24T20:47:21.0915239Z 
2020-03-24T20:47:21.0915461Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0915969Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:57:24
2020-03-24T20:47:21.0916185Z    |
2020-03-24T20:47:21.0916476Z LL |     fn clone(&self) -> _ { Test9 }
2020-03-24T20:47:21.0916859Z    |                        |
2020-03-24T20:47:21.0917078Z    |                        not allowed in type signatures
2020-03-24T20:47:21.0917449Z    |                        help: replace with the correct return type: `Test9`
2020-03-24T20:47:21.0917704Z 
2020-03-24T20:47:21.0917704Z 
2020-03-24T20:47:21.0917925Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0918427Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:105:31
2020-03-24T20:47:21.0918657Z    |
2020-03-24T20:47:21.0918957Z LL |         fn fn_test9(&self) -> _ { () }
2020-03-24T20:47:21.0919376Z    |                               |
2020-03-24T20:47:21.0919615Z    |                               not allowed in type signatures
2020-03-24T20:47:21.0919948Z    |                               help: replace with the correct return type: `()`
2020-03-24T20:47:21.0920188Z 
2020-03-24T20:47:21.0920188Z 
2020-03-24T20:47:21.0920408Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0920906Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:113:28
2020-03-24T20:47:21.0921138Z    |
2020-03-24T20:47:21.0921448Z LL |         fn clone(&self) -> _ { FnTest9 }
2020-03-24T20:47:21.0921855Z    |                            |
2020-03-24T20:47:21.0922087Z    |                            not allowed in type signatures
2020-03-24T20:47:21.0922431Z    |                            help: replace with the correct return type: `main::FnTest9`
2020-03-24T20:47:21.0922671Z 
---
2020-03-24T20:47:21.0926996Z 
2020-03-24T20:47:21.0927215Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-24T20:47:21.0927724Z   --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:204:14
2020-03-24T20:47:21.0927939Z    |
2020-03-24T20:47:21.0928082Z LL |     const D: _ = 42;
2020-03-24T20:47:21.0928417Z    |              |
2020-03-24T20:47:21.0928607Z    |              not allowed in type signatures
2020-03-24T20:47:21.0928886Z    |              help: replace `_` with the correct type: `i32`
2020-03-24T20:47:21.0929092Z 
---
2020-03-24T20:47:21.0932612Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T20:47:21.0932956Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-24T20:47:21.0933146Z 
2020-03-24T20:47:21.0933225Z 
2020-03-24T20:47:21.0936303Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T20:47:21.0938515Z 
2020-03-24T20:47:21.0938610Z 
2020-03-24T20:47:21.0939016Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T20:47:21.0939319Z Build completed unsuccessfully in 0:56:49
2020-03-24T20:47:21.0939319Z Build completed unsuccessfully in 0:56:49
2020-03-24T20:47:21.0939536Z == clock drift check ==
2020-03-24T20:47:21.0939743Z   local time: Tue Mar 24 20:47:21 UTC 2020
2020-03-24T20:47:21.3658724Z   network time: Tue, 24 Mar 2020 20:47:21 GMT
2020-03-24T20:47:21.3659378Z == end clock drift check ==
2020-03-24T20:47:21.9018096Z 
2020-03-24T20:47:21.9103046Z ##[error]Bash exited with code '1'.
2020-03-24T20:47:21.9114595Z ##[section]Finishing: Run build
2020-03-24T20:47:21.9158638Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70369/merge to s
2020-03-24T20:47:21.9162878Z Task         : Get sources
2020-03-24T20:47:21.9163159Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T20:47:21.9163435Z Version      : 1.0.0
2020-03-24T20:47:21.9163620Z Author       : Microsoft
2020-03-24T20:47:21.9163620Z Author       : Microsoft
2020-03-24T20:47:21.9163925Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T20:47:21.9164271Z ==============================================================================
2020-03-24T20:47:22.1937944Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T20:47:22.1977689Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70369/merge to s
2020-03-24T20:47:22.2046309Z Cleaning up task key
2020-03-24T20:47:22.2047350Z Start cleaning up orphan processes.
2020-03-24T20:47:22.2314298Z Terminate orphan process: pid (3599) (python)
2020-03-24T20:47:22.2363092Z ##[section]Finishing: Finalize Job
