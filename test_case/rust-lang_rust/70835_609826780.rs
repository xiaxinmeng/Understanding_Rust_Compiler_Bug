plain
2020-04-06T12:48:19.9878423Z ========================== Starting Command Output ===========================
2020-04-06T12:48:19.9883171Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c2295744-17ce-4576-8470-ccb2d290a207.sh
2020-04-06T12:48:19.9883680Z 
2020-04-06T12:48:19.9888885Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T12:48:19.9908564Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70835/merge to s
2020-04-06T12:48:19.9912145Z Task         : Get sources
2020-04-06T12:48:19.9912490Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T12:48:19.9912759Z Version      : 1.0.0
2020-04-06T12:48:19.9912945Z Author       : Microsoft
---
2020-04-06T12:48:20.9890006Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T12:48:20.9896140Z ##[command]git config gc.auto 0
2020-04-06T12:48:20.9899876Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T12:48:20.9903288Z ##[command]git config --get-all http.proxy
2020-04-06T12:48:20.9909641Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70835/merge:refs/remotes/pull/70835/merge
---
2020-04-06T12:51:25.2193785Z Looks like docker image is the same as before, not uploading
2020-04-06T12:51:32.9244678Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T12:51:32.9670652Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T12:51:32.9703680Z == clock drift check ==
2020-04-06T12:51:32.9715047Z   local time: Mon Apr  6 12:51:32 UTC 2020
2020-04-06T12:51:33.1340116Z   network time: Mon, 06 Apr 2020 12:51:33 GMT
2020-04-06T12:51:33.1381746Z Starting sccache server...
2020-04-06T12:51:33.2249851Z configure: processing command line
2020-04-06T12:51:33.2251025Z configure: 
2020-04-06T12:51:33.2252397Z configure: rust.dist-src        := False
---
2020-04-06T12:57:15.3627498Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T12:57:17.0765947Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T12:57:18.8732165Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T12:57:20.6050324Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T12:57:30.2579137Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T12:57:33.2454768Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T12:57:38.2067181Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T12:57:42.9396199Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T12:57:53.0984067Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T13:22:53.8056869Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T13:22:55.8123917Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T13:22:58.0360457Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T13:22:59.7734791Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T13:23:11.8642326Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T13:23:15.0422029Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T13:23:20.9424167Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T13:23:26.9363979Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T13:23:39.1173696Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T13:51:57.4142026Z .................................................................................................... 1700/9878
2020-04-06T13:52:01.9133428Z .................................................................................................... 1800/9878
2020-04-06T13:52:11.7143746Z .................................................................................................i.. 1900/9878
2020-04-06T13:52:20.4083285Z .................................................................................................... 2000/9878
2020-04-06T13:52:27.5275863Z .......................................................................................iiiii........ 2100/9878
2020-04-06T13:52:50.6567435Z .................................................................................................... 2300/9878
2020-04-06T13:52:53.0075421Z .................................................................................................... 2400/9878
2020-04-06T13:52:55.4261003Z .................................................................................................... 2500/9878
2020-04-06T13:53:01.8818206Z .................................................................................................... 2600/9878
---
2020-04-06T13:56:13.4035382Z .............................................................i...............i...................... 5000/9878
2020-04-06T13:56:21.2801435Z .................................................................................................... 5100/9878
2020-04-06T13:56:29.6893898Z .................................................................................................... 5200/9878
2020-04-06T13:56:35.6249082Z ......i............................................................................................. 5300/9878
2020-04-06T13:56:46.8592505Z ...............................................................................................ii.ii 5400/9878
2020-04-06T13:56:52.4407300Z ........i...i....................................................................................... 5500/9878
2020-04-06T13:57:02.3686294Z ........................................i........................................................... 5700/9878
2020-04-06T13:57:02.3686294Z ........................................i........................................................... 5700/9878
2020-04-06T13:57:13.0943544Z ............................................................ii.....................................i 5800/9878
2020-04-06T13:57:26.8541141Z .................................................................................................... 6000/9878
2020-04-06T13:57:26.8541141Z .................................................................................................... 6000/9878
2020-04-06T13:57:37.4085573Z .............................................................................................ii...i. 6100/9878
2020-04-06T13:57:50.3337017Z .ii...........i..................................................................................... 6200/9878
2020-04-06T13:58:07.6872022Z .................................................................................................... 6400/9878
2020-04-06T13:58:13.8161685Z .................................................................................................... 6500/9878
2020-04-06T13:58:13.8161685Z .................................................................................................... 6500/9878
2020-04-06T13:58:30.2318403Z .......................i..ii........................................................................ 6600/9878
2020-04-06T13:58:52.9914966Z .................................................................................................... 6800/9878
2020-04-06T13:58:55.2758435Z .......................i............................................................................ 6900/9878
2020-04-06T13:58:57.5258890Z .................................................................................................... 7000/9878
2020-04-06T13:58:59.9594016Z ..............................................................i..................................... 7100/9878
---
2020-04-06T14:00:49.2856975Z .................................................................................................... 7800/9878
2020-04-06T14:00:54.3157280Z .................................................................................................... 7900/9878
2020-04-06T14:01:00.9974743Z .................................................................................................... 8000/9878
2020-04-06T14:01:09.8008867Z ...........................i........................................................................ 8100/9878
2020-04-06T14:01:19.1243819Z ............................................................................iiiiiiiiii.i............ 8200/9878
2020-04-06T14:01:36.9356347Z ....................i......i........................................................................ 8400/9878
2020-04-06T14:01:42.2761906Z .................................................................................................... 8500/9878
2020-04-06T14:01:54.2910090Z .................................................................................................... 8600/9878
2020-04-06T14:02:08.0539733Z .................................................................................................... 8700/9878
---
2020-04-06T14:04:49.6247010Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-06T14:04:49.6442832Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T14:04:49.8664838Z 
2020-04-06T14:04:49.8665068Z running 185 tests
2020-04-06T14:04:52.9555770Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-06T14:04:55.8718015Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-06T14:04:55.8725030Z 
2020-04-06T14:04:55.8730957Z  finished in 6.228
2020-04-06T14:04:55.8738557Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-06T14:04:55.8940771Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T14:04:58.2827415Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-06T14:04:58.3068482Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T14:04:58.4875065Z 
2020-04-06T14:04:58.4875450Z running 9 tests
2020-04-06T14:04:58.4876677Z iiiiiiiii
2020-04-06T14:04:58.4877706Z 
2020-04-06T14:04:58.4880590Z  finished in 0.181
2020-04-06T14:04:58.4891126Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-06T14:04:58.5111234Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T14:05:21.2214127Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-06T14:05:21.2465180Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T14:05:21.4570959Z 
2020-04-06T14:05:21.4571301Z running 115 tests
2020-04-06T14:05:36.2634751Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-06T14:05:38.2322508Z ...iiii.....ii.
2020-04-06T14:05:38.2323785Z 
2020-04-06T14:05:38.2327335Z  finished in 16.986
2020-04-06T14:05:38.2343028Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-06T14:05:38.2343821Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T14:19:39.0322422Z 
2020-04-06T14:19:39.0325002Z    Doc-tests core
2020-04-06T14:19:44.2992691Z 
2020-04-06T14:19:44.2993500Z running 2508 tests
2020-04-06T14:19:54.3464955Z ......iiiii......................................................................................... 100/2508
2020-04-06T14:20:04.0879261Z .....................................................................................ii............. 200/2508
2020-04-06T14:20:26.9977062Z ....................i............................................................................... 400/2508
2020-04-06T14:20:26.9977062Z ....................i............................................................................... 400/2508
2020-04-06T14:20:37.7891244Z ..........................................................................i..i..................iiii 500/2508
2020-04-06T14:20:55.2135610Z ............................................................................FF.F.................... 700/2508
2020-04-06T14:21:04.1377157Z ......................................................FF.F.......................................... 800/2508
2020-04-06T14:21:13.1238896Z ................................FF.F................................................................ 900/2508
2020-04-06T14:21:13.1238896Z ................................FF.F................................................................ 900/2508
2020-04-06T14:21:21.9040036Z ..........FF.F..........................................................................F.FF........ 1000/2508
2020-04-06T14:21:30.8160504Z ..................................................................FF.F.............................. 1100/2508
2020-04-06T14:21:49.2434583Z .................................................................................................... 1300/2508
2020-04-06T14:21:58.3052397Z .................................................................................................... 1400/2508
2020-04-06T14:22:07.5136072Z .................................................................................................... 1500/2508
2020-04-06T14:22:16.6644141Z .................................................................................................... 1600/2508
---
2020-04-06T14:23:46.6091587Z 
2020-04-06T14:23:46.6092165Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6092584Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6092768Z   |
2020-04-06T14:23:46.6092940Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6093486Z   |
2020-04-06T14:23:46.6093486Z   |
2020-04-06T14:23:46.6093729Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6094129Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6094311Z 
2020-04-06T14:23:46.6094725Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6095137Z Couldn't compile the test.
---
2020-04-06T14:23:46.6098439Z 
2020-04-06T14:23:46.6098843Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6099236Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6099393Z   |
2020-04-06T14:23:46.6099577Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6100148Z   |
2020-04-06T14:23:46.6100148Z   |
2020-04-06T14:23:46.6100411Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6100809Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6100972Z 
2020-04-06T14:23:46.6101444Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6101858Z Couldn't compile the test.
---
2020-04-06T14:23:46.6106148Z   |
2020-04-06T14:23:46.6106306Z 9 | let result = two.log2();
2020-04-06T14:23:46.6106520Z   |                  ^^^^
2020-04-06T14:23:46.6106791Z   |
2020-04-06T14:23:46.6107035Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6107438Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6107597Z 
2020-04-06T14:23:46.6108032Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6108454Z Couldn't compile the test.
---
2020-04-06T14:23:46.6112018Z 
2020-04-06T14:23:46.6112417Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6114759Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6115331Z   |
2020-04-06T14:23:46.6115508Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6117430Z   |
2020-04-06T14:23:46.6117430Z   |
2020-04-06T14:23:46.6117674Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6118093Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6118269Z 
2020-04-06T14:23:46.6118782Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6119198Z Couldn't compile the test.
---
2020-04-06T14:23:46.6122509Z 
2020-04-06T14:23:46.6122891Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6123444Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6123605Z   |
2020-04-06T14:23:46.6123796Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6124158Z   |
2020-04-06T14:23:46.6124158Z   |
2020-04-06T14:23:46.6124420Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6124817Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6124988Z 
2020-04-06T14:23:46.6125420Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6125832Z Couldn't compile the test.
---
2020-04-06T14:23:46.6135968Z   |
2020-04-06T14:23:46.6136151Z 9 | let result = two.log2();
2020-04-06T14:23:46.6136397Z   |                  ^^^^
2020-04-06T14:23:46.6136567Z   |
2020-04-06T14:23:46.6136827Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6137277Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6137789Z 
2020-04-06T14:23:46.6138308Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6138746Z Couldn't compile the test.
---
2020-04-06T14:23:46.6148821Z 
2020-04-06T14:23:46.6149266Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6149873Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6150039Z   |
2020-04-06T14:23:46.6150213Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6150615Z   |
2020-04-06T14:23:46.6150615Z   |
2020-04-06T14:23:46.6150868Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6151299Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6151470Z 
2020-04-06T14:23:46.6151899Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6152324Z Couldn't compile the test.
---
2020-04-06T14:23:46.6156047Z 
2020-04-06T14:23:46.6156469Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6156859Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6157035Z   |
2020-04-06T14:23:46.6157383Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6157760Z   |
2020-04-06T14:23:46.6157760Z   |
2020-04-06T14:23:46.6158195Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6158595Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6158760Z 
2020-04-06T14:23:46.6159195Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6159793Z Couldn't compile the test.
---
2020-04-06T14:23:46.6166199Z   |
2020-04-06T14:23:46.6166395Z 9 | let result = two.log2();
2020-04-06T14:23:46.6166614Z   |                  ^^^^
2020-04-06T14:23:46.6166782Z   |
2020-04-06T14:23:46.6167043Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6167653Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6167833Z 
2020-04-06T14:23:46.6168266Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6168708Z Couldn't compile the test.
---
2020-04-06T14:23:46.6173367Z 
2020-04-06T14:23:46.6173825Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6174280Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6174449Z   |
2020-04-06T14:23:46.6174626Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6175031Z   |
2020-04-06T14:23:46.6175031Z   |
2020-04-06T14:23:46.6175595Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6176046Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6176229Z 
2020-04-06T14:23:46.6176692Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6177151Z Couldn't compile the test.
---
2020-04-06T14:23:46.6181697Z 
2020-04-06T14:23:46.6182201Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6182630Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6182821Z   |
2020-04-06T14:23:46.6183000Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6183410Z   |
2020-04-06T14:23:46.6183410Z   |
2020-04-06T14:23:46.6183670Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6184101Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6184299Z 
2020-04-06T14:23:46.6184769Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6185218Z Couldn't compile the test.
---
2020-04-06T14:23:46.6189958Z   |
2020-04-06T14:23:46.6190153Z 9 | let result = two.log2();
2020-04-06T14:23:46.6190371Z   |                  ^^^^
2020-04-06T14:23:46.6190548Z   |
2020-04-06T14:23:46.6190825Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6191255Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6191431Z 
2020-04-06T14:23:46.6191888Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6192329Z Couldn't compile the test.
---
2020-04-06T14:23:46.6195830Z 
2020-04-06T14:23:46.6196241Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6196682Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6196852Z   |
2020-04-06T14:23:46.6197031Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6197602Z   |
2020-04-06T14:23:46.6197602Z   |
2020-04-06T14:23:46.6198043Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6198493Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6198671Z 
2020-04-06T14:23:46.6199347Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6199878Z Couldn't compile the test.
---
2020-04-06T14:23:46.6207446Z 
2020-04-06T14:23:46.6208014Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6208442Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6208633Z   |
2020-04-06T14:23:46.6208811Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6209223Z   |
2020-04-06T14:23:46.6209223Z   |
2020-04-06T14:23:46.6209485Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6210962Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6211171Z 
2020-04-06T14:23:46.6211679Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6212124Z Couldn't compile the test.
---
2020-04-06T14:23:46.6217845Z   |
2020-04-06T14:23:46.6218043Z 9 | let result = two.log2();
2020-04-06T14:23:46.6218263Z   |                  ^^^^
2020-04-06T14:23:46.6218431Z   |
2020-04-06T14:23:46.6218713Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6219143Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6219321Z 
2020-04-06T14:23:46.6219786Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6220231Z Couldn't compile the test.
---
2020-04-06T14:23:46.6223769Z 
2020-04-06T14:23:46.6228898Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6229426Z  --> num/mod.rs:56:19
2020-04-06T14:23:46.6229611Z   |
2020-04-06T14:23:46.6229791Z 9 | let result = five.log(5);
2020-04-06T14:23:46.6230341Z   |
2020-04-06T14:23:46.6230341Z   |
2020-04-06T14:23:46.6230607Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6237499Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6237945Z 
2020-04-06T14:23:46.6241574Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6242968Z Couldn't compile the test.
---
2020-04-06T14:23:46.6247068Z 
2020-04-06T14:23:46.6247455Z error[E0658]: use of unstable library feature 'int_log'
2020-04-06T14:23:46.6247864Z  --> num/mod.rs:52:18
2020-04-06T14:23:46.6248021Z   |
2020-04-06T14:23:46.6248188Z 9 | let result = ten.log10();
2020-04-06T14:23:46.6248575Z   |
2020-04-06T14:23:46.6248575Z   |
2020-04-06T14:23:46.6248819Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6249236Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6249402Z 
2020-04-06T14:23:46.6255613Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6256231Z Couldn't compile the test.
---
2020-04-06T14:23:46.6263017Z   |
2020-04-06T14:23:46.6263190Z 9 | let result = two.log2();
2020-04-06T14:23:46.6263399Z   |                  ^^^^
2020-04-06T14:23:46.6263561Z   |
2020-04-06T14:23:46.6263837Z   = help: add `#![feature(int_log)]` to the crate attributes to enable
2020-04-06T14:23:46.6264253Z error: aborting due to 2 previous errors
2020-04-06T14:23:46.6264424Z 
2020-04-06T14:23:46.6264866Z For more information about this error, try `rustc --explain E0658`.
2020-04-06T14:23:46.6265289Z Couldn't compile the test.
---
2020-04-06T14:23:46.6278495Z 
2020-04-06T14:23:46.6279586Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-06T14:23:46.6280007Z Build completed unsuccessfully in 1:30:30
2020-04-06T14:23:46.6280271Z == clock drift check ==
2020-04-06T14:23:46.6280547Z   local time: Mon Apr  6 14:23:46 UTC 2020
2020-04-06T14:23:46.7996478Z   network time: Mon, 06 Apr 2020 14:23:46 GMT
2020-04-06T14:23:47.4173300Z 
2020-04-06T14:23:47.4173300Z 
2020-04-06T14:23:47.4254929Z ##[error]Bash exited with code '1'.
2020-04-06T14:23:47.4272205Z ##[section]Finishing: Run build
2020-04-06T14:23:47.4321087Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70835/merge to s
2020-04-06T14:23:47.4326326Z Task         : Get sources
2020-04-06T14:23:47.4326658Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T14:23:47.4326983Z Version      : 1.0.0
2020-04-06T14:23:47.4327196Z Author       : Microsoft
2020-04-06T14:23:47.4327196Z Author       : Microsoft
2020-04-06T14:23:47.4327562Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T14:23:47.4327974Z ==============================================================================
2020-04-06T14:23:47.7943366Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T14:23:47.7995213Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70835/merge to s
2020-04-06T14:23:47.8096434Z Cleaning up task key
2020-04-06T14:23:47.8099027Z Start cleaning up orphan processes.
2020-04-06T14:23:47.8309196Z Terminate orphan process: pid (3594) (python)
2020-04-06T14:23:47.8509018Z ##[section]Finishing: Finalize Job
