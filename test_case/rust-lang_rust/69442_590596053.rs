plain
2020-02-24T21:29:26.8672062Z ========================== Starting Command Output ===========================
2020-02-24T21:29:26.8680830Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cc5439e4-f659-4566-8ce1-651adcd182ec.sh
2020-02-24T21:29:26.8681372Z 
2020-02-24T21:29:26.8687373Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-24T21:29:26.8719338Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69442/merge to s
2020-02-24T21:29:26.8749607Z Task         : Get sources
2020-02-24T21:29:26.8749892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T21:29:26.8750185Z Version      : 1.0.0
2020-02-24T21:29:26.8750369Z Author       : Microsoft
---
2020-02-24T21:29:27.8847025Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-24T21:29:27.8855087Z ##[command]git config gc.auto 0
2020-02-24T21:29:27.8860569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-24T21:29:27.8865625Z ##[command]git config --get-all http.proxy
2020-02-24T21:29:27.8873873Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69442/merge:refs/remotes/pull/69442/merge
---
2020-02-24T22:26:37.8678259Z .................................................................................................... 1700/9704
2020-02-24T22:26:42.1507953Z .................................................................................................... 1800/9704
2020-02-24T22:26:52.6503659Z ...........................................i........................................................ 1900/9704
2020-02-24T22:26:59.9351797Z .................................................................................................... 2000/9704
2020-02-24T22:27:12.2169642Z .................................iiiii.............................................................. 2100/9704
2020-02-24T22:27:20.8645731Z .................................................................................................... 2300/9704
2020-02-24T22:27:22.9685331Z .................................................................................................... 2400/9704
2020-02-24T22:27:26.8699439Z .................................................................................................... 2500/9704
2020-02-24T22:27:45.2932449Z .................................................................................................... 2600/9704
---
2020-02-24T22:30:05.1908719Z .........i.......................................................................................... 5000/9704
2020-02-24T22:30:13.1557085Z .................................................................................................... 5100/9704
2020-02-24T22:30:17.1618401Z ....................................i............................................................... 5200/9704
2020-02-24T22:30:25.7990160Z .................................................................................................... 5300/9704
2020-02-24T22:30:30.9182182Z ............ii.ii........i...i...................................................................... 5400/9704
2020-02-24T22:30:38.3480710Z .................................................................................................... 5600/9704
2020-02-24T22:30:47.7813707Z .................................................................................................... 5700/9704
2020-02-24T22:30:54.1518778Z ...i................................................................................................ 5800/9704
2020-02-24T22:30:59.2390707Z .................................................................................................... 5900/9704
2020-02-24T22:30:59.2390707Z .................................................................................................... 5900/9704
2020-02-24T22:31:08.5816993Z ..............................................................................................ii...i 6000/9704
2020-02-24T22:31:19.5885634Z ..ii...........i.................................................................................... 6100/9704
2020-02-24T22:31:33.0324141Z .................................................................................................... 6300/9704
2020-02-24T22:31:36.3672908Z .................................................................................................... 6400/9704
2020-02-24T22:31:36.3672908Z .................................................................................................... 6400/9704
2020-02-24T22:31:48.6103103Z .........................i..ii...................................................................... 6500/9704
2020-02-24T22:32:07.8109593Z .................................................................................................... 6700/9704
2020-02-24T22:32:09.9704549Z .................i.................................................................................. 6800/9704
2020-02-24T22:32:11.9840531Z .................................................................................................... 6900/9704
2020-02-24T22:32:14.1613826Z ...............................................i.................................................... 7000/9704
---
2020-02-24T22:33:44.8896680Z .................................................................................................... 7700/9704
2020-02-24T22:33:49.6281738Z .................................................................................................... 7800/9704
2020-02-24T22:33:55.7746489Z ...........................................................................................i........ 7900/9704
2020-02-24T22:34:03.5489906Z .................................................................................................... 8000/9704
2020-02-24T22:34:10.4007607Z ........................................iiiiiii.i................................................... 8100/9704
2020-02-24T22:34:23.8472979Z .................................................................................................... 8300/9704
2020-02-24T22:34:29.9524839Z .................................................................................................... 8400/9704
2020-02-24T22:34:43.8144403Z .................................................................................................... 8500/9704
2020-02-24T22:34:50.5192501Z .................................................................................................... 8600/9704
---
2020-02-24T22:36:58.5925300Z  finished in 6.682
2020-02-24T22:36:58.6103123Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:36:58.7918427Z 
2020-02-24T22:36:58.7918837Z running 178 tests
2020-02-24T22:37:01.4112208Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-24T22:37:03.4305403Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-24T22:37:03.4311309Z 
2020-02-24T22:37:03.4311871Z  finished in 4.821
2020-02-24T22:37:03.4489361Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:37:03.5969906Z 
---
2020-02-24T22:37:05.3075965Z  finished in 1.858
2020-02-24T22:37:05.3254784Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:37:05.4664462Z 
2020-02-24T22:37:05.4665640Z running 9 tests
2020-02-24T22:37:05.4668066Z iiiiiiiii
2020-02-24T22:37:05.4669020Z 
2020-02-24T22:37:05.4669198Z  finished in 0.141
2020-02-24T22:37:05.4843302Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:37:05.6483635Z 
---
2020-02-24T22:37:23.2562693Z  finished in 17.771
2020-02-24T22:37:23.2769141Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:37:23.4451577Z 
2020-02-24T22:37:23.4451985Z running 116 tests
2020-02-24T22:37:35.4308934Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-24T22:37:37.0863330Z ....iiii.....ii.
2020-02-24T22:37:37.0864644Z 
2020-02-24T22:37:37.0868284Z  finished in 13.811
2020-02-24T22:37:37.0875543Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-24T22:37:37.0876650Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-24T22:48:53.9597846Z 
2020-02-24T22:48:53.9599532Z    Doc-tests core
2020-02-24T22:48:57.9352070Z 
2020-02-24T22:48:57.9353526Z running 2476 tests
2020-02-24T22:49:05.8439011Z ......iiiii......................................................................................... 100/2476
2020-02-24T22:49:13.6124632Z ..................................................................................ii................ 200/2476
2020-02-24T22:49:31.8268294Z .................i.................................................................................. 400/2476
2020-02-24T22:49:31.8268294Z .................i.................................................................................. 400/2476
2020-02-24T22:49:40.3786526Z ......................................................................i..i..................iiii.... 500/2476
2020-02-24T22:49:54.6846732Z .................................................................................................... 700/2476
2020-02-24T22:50:02.1191844Z .................................................................................................... 800/2476
2020-02-24T22:50:09.3890122Z .................................................................................................... 900/2476
2020-02-24T22:50:17.0684746Z .................................................................................................... 1000/2476
---
2020-02-24T22:53:30.2821700Z 
2020-02-24T22:53:30.2822464Z running 1009 tests
2020-02-24T22:53:45.9362662Z i................................................................................................... 100/1009
2020-02-24T22:53:54.8680129Z .................................................................................................... 200/1009
2020-02-24T22:54:01.0885568Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-24T22:54:05.5029330Z .................................................................................................... 400/1009
2020-02-24T22:54:11.4952825Z ............................................i..i.....................................ii............. 500/1009
2020-02-24T22:54:22.6935912Z .................................................................................................... 700/1009
2020-02-24T22:54:22.6935912Z .................................................................................................... 700/1009
2020-02-24T22:54:28.5757374Z ...................................iiii............................................................. 800/1009
2020-02-24T22:54:40.9887192Z .................................................................................................... 900/1009
2020-02-24T22:54:46.8768968Z .........................................................iiii....................................... 1000/1009
2020-02-24T22:54:47.2153804Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-24T22:54:47.2154208Z 
2020-02-24T22:54:47.2242692Z  finished in 148.099
2020-02-24T22:54:47.2255163Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-24T23:11:20.6681523Z Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
2020-02-24T23:11:20.8763179Z    Compiling same-file v1.0.4
2020-02-24T23:11:20.9846301Z    Compiling walkdir v2.2.7
2020-02-24T23:11:22.2939527Z    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
2020-02-24T23:11:23.0228065Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0001.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0230530Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:12:13
2020-02-24T23:11:23.0231902Z    |
2020-02-24T23:11:23.0233103Z 12 | E0001: Some(include_str!("./error_codes/E0001.md")),
2020-02-24T23:11:23.0235963Z    | 
2020-02-24T23:11:23.0237055Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0238070Z    |
2020-02-24T23:11:23.0238070Z    |
2020-02-24T23:11:23.0239209Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0240425Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0240863Z 
2020-02-24T23:11:23.0247500Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0002.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0248847Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:13:13
2020-02-24T23:11:23.0249577Z    |
2020-02-24T23:11:23.0250436Z 13 | E0002: Some(include_str!("./error_codes/E0002.md")),
2020-02-24T23:11:23.0252059Z    | 
2020-02-24T23:11:23.0252671Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0253243Z    |
2020-02-24T23:11:23.0253243Z    |
2020-02-24T23:11:23.0253942Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0254947Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0262355Z 
2020-02-24T23:11:23.0263708Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0004.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0265043Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:14:13
2020-02-24T23:11:23.0265774Z    |
2020-02-24T23:11:23.0266445Z 14 | E0004: Some(include_str!("./error_codes/E0004.md")),
2020-02-24T23:11:23.0268055Z    | 
2020-02-24T23:11:23.0268667Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0269249Z    |
2020-02-24T23:11:23.0269249Z    |
2020-02-24T23:11:23.0269951Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0270968Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0271388Z 
2020-02-24T23:11:23.0281950Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0005.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0283311Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:15:13
2020-02-24T23:11:23.0284056Z    |
2020-02-24T23:11:23.0284734Z 15 | E0005: Some(include_str!("./error_codes/E0005.md")),
2020-02-24T23:11:23.0286344Z    | 
2020-02-24T23:11:23.0286974Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0287527Z    |
2020-02-24T23:11:23.0287527Z    |
2020-02-24T23:11:23.0288221Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0289234Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0289659Z 
2020-02-24T23:11:23.0300231Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0007.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0301612Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:16:13
2020-02-24T23:11:23.0302337Z    |
2020-02-24T23:11:23.0303028Z 16 | E0007: Some(include_str!("./error_codes/E0007.md")),
2020-02-24T23:11:23.0304703Z    | 
2020-02-24T23:11:23.0305339Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0305899Z    |
2020-02-24T23:11:23.0305899Z    |
2020-02-24T23:11:23.0306595Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0307617Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0308038Z 
2020-02-24T23:11:23.0324410Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0009.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0325767Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:17:13
2020-02-24T23:11:23.0326508Z    |
2020-02-24T23:11:23.0327196Z 17 | E0009: Some(include_str!("./error_codes/E0009.md")),
2020-02-24T23:11:23.0328802Z    | 
2020-02-24T23:11:23.0329417Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0329970Z    |
2020-02-24T23:11:23.0329970Z    |
2020-02-24T23:11:23.0330683Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0331691Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0332113Z 
2020-02-24T23:11:23.0333102Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0010.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0334405Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:18:13
2020-02-24T23:11:23.0335126Z    |
2020-02-24T23:11:23.0335809Z 18 | E0010: Some(include_str!("./error_codes/E0010.md")),
2020-02-24T23:11:23.0337542Z    | 
2020-02-24T23:11:23.0338174Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0338725Z    |
2020-02-24T23:11:23.0338725Z    |
2020-02-24T23:11:23.0339433Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0340432Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0340847Z 
2020-02-24T23:11:23.0341827Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0013.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0343202Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:19:13
2020-02-24T23:11:23.0343921Z    |
2020-02-24T23:11:23.0344601Z 19 | E0013: Some(include_str!("./error_codes/E0013.md")),
2020-02-24T23:11:23.0346197Z    | 
2020-02-24T23:11:23.0346808Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0347356Z    |
2020-02-24T23:11:23.0347356Z    |
2020-02-24T23:11:23.0348062Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0349072Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0350051Z 
2020-02-24T23:11:23.0366700Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0014.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0368806Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:20:13
2020-02-24T23:11:23.0369722Z    |
2020-02-24T23:11:23.0370541Z 20 | E0014: Some(include_str!("./error_codes/E0014.md")),
2020-02-24T23:11:23.0374259Z    | 
2020-02-24T23:11:23.0376272Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0378772Z    |
2020-02-24T23:11:23.0378772Z    |
2020-02-24T23:11:23.0385398Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0386694Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0387266Z 
2020-02-24T23:11:23.0388374Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0015.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0390024Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:21:13
2020-02-24T23:11:23.0390921Z    |
2020-02-24T23:11:23.0391726Z 21 | E0015: Some(include_str!("./error_codes/E0015.md")),
2020-02-24T23:11:23.0393630Z    | 
2020-02-24T23:11:23.0394381Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0395083Z    |
2020-02-24T23:11:23.0395083Z    |
2020-02-24T23:11:23.0396155Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0397974Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0398511Z 
2020-02-24T23:11:23.0400485Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0019.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0401890Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:22:13
2020-02-24T23:11:23.0402662Z    |
2020-02-24T23:11:23.0403338Z 22 | E0019: Some(include_str!("./error_codes/E0019.md")),
2020-02-24T23:11:23.0405329Z    | 
2020-02-24T23:11:23.0405945Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0406517Z    |
2020-02-24T23:11:23.0406517Z    |
2020-02-24T23:11:23.0407217Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0408219Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0408653Z 
2020-02-24T23:11:23.0409631Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0023.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0411017Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:23:13
2020-02-24T23:11:23.0411738Z    |
2020-02-24T23:11:23.0412426Z 23 | E0023: Some(include_str!("./error_codes/E0023.md")),
2020-02-24T23:11:23.0414012Z    | 
2020-02-24T23:11:23.0414635Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0415189Z    |
2020-02-24T23:11:23.0415189Z    |
2020-02-24T23:11:23.0416050Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0417089Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0417507Z 
2020-02-24T23:11:23.0418474Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0025.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0419902Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:24:13
2020-02-24T23:11:23.0420569Z    |
2020-02-24T23:11:23.0421289Z 24 | E0025: Some(include_str!("./error_codes/E0025.md")),
2020-02-24T23:11:23.0423041Z    | 
2020-02-24T23:11:23.0423621Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0424323Z    |
2020-02-24T23:11:23.0424323Z    |
2020-02-24T23:11:23.0425010Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0426028Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0426443Z 
2020-02-24T23:11:23.0427411Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0026.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0428730Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:25:13
2020-02-24T23:11:23.0429446Z    |
2020-02-24T23:11:23.0430125Z 25 | E0026: Some(include_str!("./error_codes/E0026.md")),
2020-02-24T23:11:23.0431708Z    | 
2020-02-24T23:11:23.0432333Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0432898Z    |
2020-02-24T23:11:23.0432898Z    |
2020-02-24T23:11:23.0433592Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0434609Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0435025Z 
2020-02-24T23:11:23.0435992Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0027.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0437363Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:26:13
2020-02-24T23:11:23.0438035Z    |
2020-02-24T23:11:23.0438716Z 26 | E0027: Some(include_str!("./error_codes/E0027.md")),
2020-02-24T23:11:23.0440197Z    | 
2020-02-24T23:11:23.0457168Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0457712Z    |
2020-02-24T23:11:23.0457712Z    |
2020-02-24T23:11:23.0458367Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0459312Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0459812Z 
2020-02-24T23:11:23.0460730Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0029.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0461949Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:27:13
2020-02-24T23:11:23.0462612Z    |
2020-02-24T23:11:23.0463242Z 27 | E0029: Some(include_str!("./error_codes/E0029.md")),
2020-02-24T23:11:23.0464704Z    | 
2020-02-24T23:11:23.0465295Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0465806Z    |
2020-02-24T23:11:23.0465806Z    |
2020-02-24T23:11:23.0466448Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0467387Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0467769Z 
2020-02-24T23:11:23.0468659Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0030.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0469877Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:28:13
2020-02-24T23:11:23.0470546Z    |
2020-02-24T23:11:23.0471175Z 28 | E0030: Some(include_str!("./error_codes/E0030.md")),
2020-02-24T23:11:23.0472641Z    | 
2020-02-24T23:11:23.0473223Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0473732Z    |
2020-02-24T23:11:23.0473732Z    |
2020-02-24T23:11:23.0474370Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0475311Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0475758Z 
2020-02-24T23:11:23.0488758Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0033.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0489996Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:29:13
2020-02-24T23:11:23.0490686Z    |
2020-02-24T23:11:23.0491310Z 29 | E0033: Some(include_str!("./error_codes/E0033.md")),
2020-02-24T23:11:23.0492932Z    | 
2020-02-24T23:11:23.0493542Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0494065Z    |
2020-02-24T23:11:23.0494065Z    |
2020-02-24T23:11:23.0494712Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0495642Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0496046Z 
2020-02-24T23:11:23.0496947Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0034.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0498154Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:30:13
2020-02-24T23:11:23.0498845Z    |
2020-02-24T23:11:23.0499461Z 30 | E0034: Some(include_str!("./error_codes/E0034.md")),
2020-02-24T23:11:23.0500941Z    | 
2020-02-24T23:11:23.0501505Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0502035Z    |
2020-02-24T23:11:23.0502035Z    |
2020-02-24T23:11:23.0502681Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0503614Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0504019Z 
2020-02-24T23:11:23.0504918Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0038.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0506915Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:31:13
2020-02-24T23:11:23.0507674Z    |
2020-02-24T23:11:23.0508343Z 31 | E0038: Some(include_str!("./error_codes/E0038.md")),
2020-02-24T23:11:23.0510049Z    | 
2020-02-24T23:11:23.0510656Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0511221Z    |
2020-02-24T23:11:23.0511221Z    |
2020-02-24T23:11:23.0511911Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0512905Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0513337Z 
2020-02-24T23:11:23.0514304Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0040.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0515684Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:32:13
2020-02-24T23:11:23.0516420Z    |
2020-02-24T23:11:23.0517082Z 32 | E0040: Some(include_str!("./error_codes/E0040.md")),
2020-02-24T23:11:23.0518672Z    | 
2020-02-24T23:11:23.0519280Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0520113Z    |
2020-02-24T23:11:23.0520113Z    |
2020-02-24T23:11:23.0525105Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0526138Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0526584Z 
2020-02-24T23:11:23.0527557Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0044.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0529124Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:33:13
2020-02-24T23:11:23.0529821Z    |
2020-02-24T23:11:23.0530438Z 33 | E0044: Some(include_str!("./error_codes/E0044.md")),
2020-02-24T23:11:23.0531935Z    | 
2020-02-24T23:11:23.0532498Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0533024Z    |
2020-02-24T23:11:23.0533024Z    |
2020-02-24T23:11:23.0533672Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0534594Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0534996Z 
2020-02-24T23:11:23.0536007Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0045.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0537220Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:34:13
2020-02-24T23:11:23.0537896Z    |
2020-02-24T23:11:23.0538509Z 34 | E0045: Some(include_str!("./error_codes/E0045.md")),
2020-02-24T23:11:23.0539991Z    | 
2020-02-24T23:11:23.0540554Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0541079Z    |
2020-02-24T23:11:23.0541079Z    |
2020-02-24T23:11:23.0541799Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0542721Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0543123Z 
2020-02-24T23:11:23.0544013Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0046.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0545206Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:35:13
2020-02-24T23:11:23.0545888Z    |
2020-02-24T23:11:23.0546499Z 35 | E0046: Some(include_str!("./error_codes/E0046.md")),
2020-02-24T23:11:23.0548184Z    | 
2020-02-24T23:11:23.0548746Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0549273Z    |
2020-02-24T23:11:23.0549273Z    |
2020-02-24T23:11:23.0549912Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0550834Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0551234Z 
2020-02-24T23:11:23.0552124Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0049.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0553327Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:36:13
2020-02-24T23:11:23.0554007Z    |
2020-02-24T23:11:23.0554618Z 36 | E0049: Some(include_str!("./error_codes/E0049.md")),
2020-02-24T23:11:23.0556095Z    | 
2020-02-24T23:11:23.0556658Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0557190Z    |
2020-02-24T23:11:23.0557190Z    |
2020-02-24T23:11:23.0557885Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0558821Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0559221Z 
2020-02-24T23:11:23.0560110Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0050.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0570732Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:37:13
2020-02-24T23:11:23.0571566Z    |
2020-02-24T23:11:23.0572195Z 37 | E0050: Some(include_str!("./error_codes/E0050.md")),
2020-02-24T23:11:23.0573684Z    | 
2020-02-24T23:11:23.0574252Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0574782Z    |
2020-02-24T23:11:23.0574782Z    |
2020-02-24T23:11:23.0575430Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0576358Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0576765Z 
2020-02-24T23:11:23.0577666Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0053.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0578876Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:38:13
2020-02-24T23:11:23.0579558Z    |
2020-02-24T23:11:23.0580367Z 38 | E0053: Some(include_str!("./error_codes/E0053.md")),
2020-02-24T23:11:23.0581974Z    | 
2020-02-24T23:11:23.0582589Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0583164Z    |
2020-02-24T23:11:23.0583164Z    |
2020-02-24T23:11:23.0583858Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0584859Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0585292Z 
2020-02-24T23:11:23.0586316Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0054.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0587521Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:39:13
2020-02-24T23:11:23.0588494Z    |
2020-02-24T23:11:23.0589174Z 39 | E0054: Some(include_str!("./error_codes/E0054.md")),
2020-02-24T23:11:23.0590770Z    | 
2020-02-24T23:11:23.0591378Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0591948Z    |
2020-02-24T23:11:23.0591948Z    |
2020-02-24T23:11:23.0592782Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0593934Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0594460Z 
2020-02-24T23:11:23.0595440Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0055.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0596741Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:40:13
2020-02-24T23:11:23.0597475Z    |
2020-02-24T23:11:23.0598137Z 40 | E0055: Some(include_str!("./error_codes/E0055.md")),
2020-02-24T23:11:23.0599736Z    | 
2020-02-24T23:11:23.0600558Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0601098Z    |
2020-02-24T23:11:23.0601098Z    |
2020-02-24T23:11:23.0601738Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0602659Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0603060Z 
2020-02-24T23:11:23.0603954Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0057.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0605177Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:41:13
2020-02-24T23:11:23.0605848Z    |
2020-02-24T23:11:23.0606462Z 41 | E0057: Some(include_str!("./error_codes/E0057.md")),
2020-02-24T23:11:23.0607950Z    | 
2020-02-24T23:11:23.0608513Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0609038Z    |
2020-02-24T23:11:23.0609038Z    |
2020-02-24T23:11:23.0609679Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0610659Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0611066Z 
2020-02-24T23:11:23.0611965Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0059.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0613182Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:42:13
2020-02-24T23:11:23.0613847Z    |
2020-02-24T23:11:23.0614462Z 42 | E0059: Some(include_str!("./error_codes/E0059.md")),
2020-02-24T23:11:23.0616009Z    | 
2020-02-24T23:11:23.0616576Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0617105Z    |
2020-02-24T23:11:23.0617105Z    |
2020-02-24T23:11:23.0617743Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0618667Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0619066Z 
2020-02-24T23:11:23.0619954Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0060.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0621177Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:43:13
2020-02-24T23:11:23.0621846Z    |
2020-02-24T23:11:23.0622464Z 43 | E0060: Some(include_str!("./error_codes/E0060.md")),
2020-02-24T23:11:23.0623946Z    | 
2020-02-24T23:11:23.0624510Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0625037Z    |
2020-02-24T23:11:23.0625037Z    |
2020-02-24T23:11:23.0625679Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0626609Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0627011Z 
2020-02-24T23:11:23.0627907Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0061.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0629119Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:44:13
2020-02-24T23:11:23.0629783Z    |
2020-02-24T23:11:23.0630398Z 44 | E0061: Some(include_str!("./error_codes/E0061.md")),
2020-02-24T23:11:23.0631947Z    | 
2020-02-24T23:11:23.0632507Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0633033Z    |
2020-02-24T23:11:23.0633033Z    |
2020-02-24T23:11:23.0633676Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0634599Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0634997Z 
2020-02-24T23:11:23.0635972Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0062.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0637255Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:45:13
2020-02-24T23:11:23.0637922Z    |
2020-02-24T23:11:23.0638551Z 45 | E0062: Some(include_str!("./error_codes/E0062.md")),
2020-02-24T23:11:23.0640018Z    | 
2020-02-24T23:11:23.0666045Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0666660Z    |
2020-02-24T23:11:23.0666660Z    |
2020-02-24T23:11:23.0667395Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0668433Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0668852Z 
2020-02-24T23:11:23.0669823Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0063.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0671143Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:46:13
2020-02-24T23:11:23.0671862Z    |
2020-02-24T23:11:23.0672542Z 46 | E0063: Some(include_str!("./error_codes/E0063.md")),
2020-02-24T23:11:23.0674139Z    | 
2020-02-24T23:11:23.0674764Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0675313Z    |
2020-02-24T23:11:23.0675313Z    |
2020-02-24T23:11:23.0676005Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0677018Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0677430Z 
2020-02-24T23:11:23.0678528Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0067.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0679860Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:47:13
2020-02-24T23:11:23.0680846Z    |
2020-02-24T23:11:23.0681531Z 47 | E0067: Some(include_str!("./error_codes/E0067.md")),
2020-02-24T23:11:23.0683118Z    | 
2020-02-24T23:11:23.0683742Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0684382Z    |
2020-02-24T23:11:23.0684382Z    |
2020-02-24T23:11:23.0685079Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0686098Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0686514Z 
2020-02-24T23:11:23.0687483Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0069.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0688797Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:48:13
2020-02-24T23:11:23.0689514Z    |
2020-02-24T23:11:23.0690202Z 48 | E0069: Some(include_str!("./error_codes/E0069.md")),
2020-02-24T23:11:23.0691791Z    | 
2020-02-24T23:11:23.0692418Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0692967Z    |
2020-02-24T23:11:23.0692967Z    |
2020-02-24T23:11:23.0694144Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0695280Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0695701Z 
2020-02-24T23:11:23.0696744Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0070.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0698082Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:49:13
2020-02-24T23:11:23.0698839Z    |
2020-02-24T23:11:23.0699523Z 49 | E0070: Some(include_str!("./error_codes/E0070.md")),
2020-02-24T23:11:23.0701130Z    | 
2020-02-24T23:11:23.0701786Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0702525Z    |
2020-02-24T23:11:23.0702525Z    |
2020-02-24T23:11:23.0703241Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0704260Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0704675Z 
2020-02-24T23:11:23.0705641Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0071.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0706947Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:50:13
2020-02-24T23:11:23.0707746Z    |
2020-02-24T23:11:23.0708433Z 50 | E0071: Some(include_str!("./error_codes/E0071.md")),
2020-02-24T23:11:23.0710016Z    | 
2020-02-24T23:11:23.0710642Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0711192Z    |
2020-02-24T23:11:23.0711192Z    |
2020-02-24T23:11:23.0711885Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0712895Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0713314Z 
2020-02-24T23:11:23.0714282Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0072.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0715595Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:51:13
2020-02-24T23:11:23.0716314Z    |
2020-02-24T23:11:23.0716992Z 51 | E0072: Some(include_str!("./error_codes/E0072.md")),
2020-02-24T23:11:23.0718572Z    | 
2020-02-24T23:11:23.0719202Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0719758Z    |
2020-02-24T23:11:23.0719758Z    |
2020-02-24T23:11:23.0720447Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0721458Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0721875Z 
2020-02-24T23:11:23.0722833Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0073.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0724200Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:52:13
2020-02-24T23:11:23.0724935Z    |
2020-02-24T23:11:23.0725620Z 52 | E0073: Some(include_str!("./error_codes/E0073.md")),
2020-02-24T23:11:23.0727194Z    | 
2020-02-24T23:11:23.0727818Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0728372Z    |
2020-02-24T23:11:23.0728372Z    |
2020-02-24T23:11:23.0729065Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0730077Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0730544Z 
2020-02-24T23:11:23.0731517Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0074.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0732829Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:53:13
2020-02-24T23:11:23.0733548Z    |
2020-02-24T23:11:23.0734225Z 53 | E0074: Some(include_str!("./error_codes/E0074.md")),
2020-02-24T23:11:23.0735820Z    | 
2020-02-24T23:11:23.0736448Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0736995Z    |
2020-02-24T23:11:23.0736995Z    |
2020-02-24T23:11:23.0737691Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0738709Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0739122Z 
2020-02-24T23:11:23.0740227Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0075.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0741535Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:54:13
2020-02-24T23:11:23.0742272Z    |
2020-02-24T23:11:23.0742936Z 54 | E0075: Some(include_str!("./error_codes/E0075.md")),
2020-02-24T23:11:23.0744534Z    | 
2020-02-24T23:11:23.0745142Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0745707Z    |
2020-02-24T23:11:23.0745707Z    |
2020-02-24T23:11:23.0746403Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0747461Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0747893Z 
2020-02-24T23:11:23.0748861Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0076.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0750153Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:55:13
2020-02-24T23:11:23.0750886Z    |
2020-02-24T23:11:23.0751547Z 55 | E0076: Some(include_str!("./error_codes/E0076.md")),
2020-02-24T23:11:23.0753206Z    | 
2020-02-24T23:11:23.0753814Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0754377Z    |
2020-02-24T23:11:23.0754377Z    |
2020-02-24T23:11:23.0755071Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0756065Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0756497Z 
2020-02-24T23:11:23.0757455Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0077.md: No such file or directory (os error 2)
2020-02-24T23:11:23.0758756Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/all_error_codes.rs:56:13
2020-02-24T23:11:23.0759485Z    |
2020-02-24T23:11:23.0760146Z 56 | E0077: Some(include_str!("./error_codes/E0077.md")),
2020-02-24T23:11:23.0761737Z    | 
2020-02-24T23:11:23.0762346Z   ::: <::core::macros::builtin::include_str macros>:1:1
2020-02-24T23:11:23.0762910Z    |
2020-02-24T23:11:23.0762910Z    |
2020-02-24T23:11:23.0763605Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-02-24T23:11:23.0764609Z    | ---------------------------------------------------------- in this expansion of `include_str!`
2020-02-24T23:11:23.0765040Z 
2020-02-24T23:11:23.0766003Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-805c0a97b42e972b/out/./error_codes/E0080.md: No such file or directory (os error 2)
---
2020-02-24T23:11:25.2145054Z   local time: Mon Feb 24 23:11:25 UTC 2020
2020-02-24T23:11:25.5060668Z   network time: Mon, 24 Feb 2020 23:11:25 GMT
2020-02-24T23:11:25.5066523Z == end clock drift check ==
2020-02-24T23:11:27.8692889Z 
2020-02-24T23:11:27.8758628Z ##[error]Bash exited with code '1'.
2020-02-24T23:11:27.8777156Z ##[section]Finishing: Run build
2020-02-24T23:11:27.8826839Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69442/merge to s
2020-02-24T23:11:27.8831578Z Task         : Get sources
2020-02-24T23:11:27.8831940Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T23:11:27.8832260Z Version      : 1.0.0
2020-02-24T23:11:27.8832482Z Author       : Microsoft
2020-02-24T23:11:27.8832482Z Author       : Microsoft
2020-02-24T23:11:27.8832855Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-24T23:11:27.8833263Z ==============================================================================
2020-02-24T23:11:28.1986610Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-24T23:11:28.2047946Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69442/merge to s
2020-02-24T23:11:28.2144532Z Cleaning up task key
2020-02-24T23:11:28.2146212Z Start cleaning up orphan processes.
2020-02-24T23:11:28.2317520Z Terminate orphan process: pid (4281) (python)
2020-02-24T23:11:28.2557968Z ##[section]Finishing: Finalize Job
