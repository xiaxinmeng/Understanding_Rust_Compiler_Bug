plain
2020-04-07T14:31:08.9777769Z ========================== Starting Command Output ===========================
2020-04-07T14:31:08.9782655Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d4428581-c28a-4b40-ac8c-27b473bca540.sh
2020-04-07T14:31:08.9782967Z 
2020-04-07T14:31:08.9788428Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T14:31:08.9808886Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70629/merge to s
2020-04-07T14:31:08.9812816Z Task         : Get sources
2020-04-07T14:31:08.9813107Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T14:31:08.9813384Z Version      : 1.0.0
2020-04-07T14:31:08.9813591Z Author       : Microsoft
---
2020-04-07T14:31:09.9884621Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T14:31:09.9893502Z ##[command]git config gc.auto 0
2020-04-07T14:31:09.9900969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T14:31:09.9915675Z ##[command]git config --get-all http.proxy
2020-04-07T14:31:09.9923053Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70629/merge:refs/remotes/pull/70629/merge
---
2020-04-07T14:33:33.5935241Z Looks like docker image is the same as before, not uploading
2020-04-07T14:33:38.3821142Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T14:33:38.4141043Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T14:33:38.4172050Z == clock drift check ==
2020-04-07T14:33:38.4179034Z   local time: Tue Apr  7 14:33:38 UTC 2020
2020-04-07T14:33:38.4838009Z   network time: Tue, 07 Apr 2020 14:33:38 GMT
2020-04-07T14:33:38.4860696Z Starting sccache server...
2020-04-07T14:33:38.5776122Z configure: processing command line
2020-04-07T14:33:38.5776948Z configure: 
2020-04-07T14:33:38.5777930Z configure: rust.dist-src        := False
---
2020-04-07T14:39:20.4567358Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T14:39:22.1289129Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T14:39:23.8800961Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T14:39:25.7072231Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T14:39:35.1102319Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T14:39:38.4293852Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T14:39:43.3716020Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T14:39:48.1589029Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T14:39:58.1443392Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T15:04:56.9486396Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T15:04:58.9610963Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T15:05:01.1368898Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T15:05:03.6590366Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T15:05:15.3101521Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T15:05:19.1457237Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T15:05:25.1487072Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T15:05:31.1004317Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T15:05:43.3176435Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T15:33:44.8390305Z .................................................................................................... 1700/9879
2020-04-07T15:33:48.9701133Z .................................................................................................... 1800/9879
2020-04-07T15:33:58.7943770Z ..................................................................................................i. 1900/9879
2020-04-07T15:34:07.5098423Z .................................................................................................... 2000/9879
2020-04-07T15:34:14.5122363Z ........................................................................................iiiii....... 2100/9879
2020-04-07T15:34:37.4752723Z .................................................................................................... 2300/9879
2020-04-07T15:34:39.7928159Z .................................................................................................... 2400/9879
2020-04-07T15:34:42.1538937Z .................................................................................................... 2500/9879
2020-04-07T15:34:48.6416397Z .................................................................................................... 2600/9879
---
2020-04-07T15:37:58.9800447Z ..............................................................i...............i..................... 5000/9879
2020-04-07T15:38:06.7387493Z .................................................................................................... 5100/9879
2020-04-07T15:38:14.8979374Z .................................................................................................... 5200/9879
2020-04-07T15:38:20.6848442Z .......i............................................................................................ 5300/9879
2020-04-07T15:38:31.3901543Z ................................................................................................ii.i 5400/9879
2020-04-07T15:38:36.5633233Z i........i...i.............................................F........................................ 5500/9879
2020-04-07T15:38:45.6999306Z .........................................i.......................................................... 5700/9879
2020-04-07T15:38:56.4497345Z .............................................................ii..................................... 5800/9879
2020-04-07T15:39:04.6856203Z i................................................................................................... 5900/9879
2020-04-07T15:39:10.3010471Z .................................................................................................... 6000/9879
2020-04-07T15:39:10.3010471Z .................................................................................................... 6000/9879
2020-04-07T15:39:20.6693266Z ..............................................................................................ii...i 6100/9879
2020-04-07T15:39:33.2696498Z ..ii...........i.................................................................................... 6200/9879
2020-04-07T15:39:49.8428088Z .................................................................................................... 6400/9879
2020-04-07T15:39:55.9869348Z .................................................................................................... 6500/9879
2020-04-07T15:39:55.9869348Z .................................................................................................... 6500/9879
2020-04-07T15:40:23.5969968Z ........................i..ii....................................................................... 6600/9879
2020-04-07T15:40:24.7891830Z ...................test [ui] ui/mpsc_stress.rs has been running for over 60 seconds
2020-04-07T15:40:45.8893931Z .................................................................................................... 6800/9879
2020-04-07T15:40:48.1085427Z ........................i........................................................................... 6900/9879
2020-04-07T15:40:50.3402042Z .................................................................................................... 7000/9879
2020-04-07T15:40:52.7306033Z ...............................................................i.................................... 7100/9879
---
2020-04-07T15:42:40.1923941Z .................................................................................................... 7800/9879
2020-04-07T15:42:44.8961872Z .................................................................................................... 7900/9879
2020-04-07T15:42:51.1419263Z .................................................................................................... 8000/9879
2020-04-07T15:42:59.0806414Z ............................i....................................................................... 8100/9879
2020-04-07T15:43:08.0685903Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-07T15:43:24.9243990Z .....................i......i....................................................................... 8400/9879
2020-04-07T15:43:29.8800819Z .................................................................................................... 8500/9879
2020-04-07T15:43:41.2925155Z .................................................................................................... 8600/9879
2020-04-07T15:43:54.1288071Z .................................................................................................... 8700/9879
---
2020-04-07T15:45:57.9811116Z diff of stderr:
2020-04-07T15:45:57.9811266Z 
2020-04-07T15:45:57.9811475Z 316    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-07T15:45:57.9811690Z 317 
2020-04-07T15:45:57.9811924Z 318 error: layout_of(i32) = Layout {
2020-04-07T15:45:57.9812421Z -     fields: Union(
2020-04-07T15:45:57.9813154Z -     ),
2020-04-07T15:45:57.9813362Z +     fields: Primitive,
2020-04-07T15:45:57.9813626Z 322     variants: Single {
2020-04-07T15:45:57.9813906Z 323         index: 0,
2020-04-07T15:45:57.9813906Z 323         index: 0,
2020-04-07T15:45:57.9814109Z 324     },
2020-04-07T15:45:57.9814241Z 
2020-04-07T15:45:57.9814352Z 
2020-04-07T15:45:57.9814587Z The actual stderr differed from the expected stderr.
2020-04-07T15:45:57.9816429Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug/debug.stderr
2020-04-07T15:45:57.9817499Z To update references, rerun the tests and pass the `--bless` flag
2020-04-07T15:45:57.9818176Z To only update this specific test, also pass `--test-args layout/debug.rs`
2020-04-07T15:45:57.9818659Z error: 1 errors occurred comparing output.
2020-04-07T15:45:57.9818928Z status: exit code: 1
2020-04-07T15:45:57.9818928Z status: exit code: 1
2020-04-07T15:45:57.9821073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug/auxiliary"
2020-04-07T15:45:57.9822853Z ------------------------------------------
2020-04-07T15:45:57.9823054Z 
2020-04-07T15:45:57.9823466Z ------------------------------------------
2020-04-07T15:45:57.9823720Z stderr:
2020-04-07T15:45:57.9823720Z stderr:
2020-04-07T15:45:57.9824141Z ------------------------------------------
2020-04-07T15:45:57.9824420Z error: layout_of(E) = Layout {
2020-04-07T15:45:57.9824687Z     fields: Arbitrary {
2020-04-07T15:45:57.9824918Z         offsets: [
2020-04-07T15:45:57.9825367Z                 raw: 0,
2020-04-07T15:45:57.9825746Z             },
2020-04-07T15:45:57.9825927Z         ],
2020-04-07T15:45:57.9826284Z         memory_index: [
---
2020-04-07T15:45:57.9830985Z                 false,
2020-04-07T15:45:57.9831206Z             ),
2020-04-07T15:45:57.9831457Z             valid_range: 0..=0,
2020-04-07T15:45:57.9831694Z         },
2020-04-07T15:45:57.9831901Z         discr_kind: Tag,
2020-04-07T15:45:57.9832146Z         discr_index: 0,
2020-04-07T15:45:57.9832615Z             Layout {
2020-04-07T15:45:57.9832868Z                 fields: Arbitrary {
2020-04-07T15:45:57.9833177Z                     offsets: [],
2020-04-07T15:45:57.9833477Z                     memory_index: [],
2020-04-07T15:45:57.9833477Z                     memory_index: [],
2020-04-07T15:45:57.9833734Z                 },
2020-04-07T15:45:57.9834004Z                 variants: Single {
2020-04-07T15:45:57.9834290Z                     index: 0,
2020-04-07T15:45:57.9834526Z                 },
2020-04-07T15:45:57.9834781Z                 abi: Aggregate {
2020-04-07T15:45:57.9835063Z                     sized: true,
2020-04-07T15:45:57.9835303Z                 },
2020-04-07T15:45:57.9835555Z                 largest_niche: None,
2020-04-07T15:45:57.9835891Z                 align: AbiAndPrefAlign {
2020-04-07T15:45:57.9836187Z                     abi: Align {
2020-04-07T15:45:57.9836469Z                         pow2: 0,
2020-04-07T15:45:57.9836989Z                     pref: Align {
2020-04-07T15:45:57.9837276Z                         pow2: 3,
2020-04-07T15:45:57.9837543Z                     },
2020-04-07T15:45:57.9837751Z                 },
---
2020-04-07T15:45:57.9843688Z                 },
2020-04-07T15:45:57.9843934Z                 variants: Single {
2020-04-07T15:45:57.9844237Z                     index: 1,
2020-04-07T15:45:57.9844472Z                 },
2020-04-07T15:45:57.9844712Z                 abi: Uninhabited,
2020-04-07T15:45:57.9845023Z                 largest_niche: None,
2020-04-07T15:45:57.9845338Z                 align: AbiAndPrefAlign {
2020-04-07T15:45:57.9845636Z                     abi: Align {
2020-04-07T15:45:57.9845938Z                         pow2: 2,
2020-04-07T15:45:57.9846441Z                     pref: Align {
2020-04-07T15:45:57.9846746Z                         pow2: 3,
2020-04-07T15:45:57.9846999Z                     },
2020-04-07T15:45:57.9847207Z                 },
---
2020-04-07T15:45:57.9848574Z     },
2020-04-07T15:45:57.9848762Z     abi: Aggregate {
2020-04-07T15:45:57.9849002Z         sized: true,
2020-04-07T15:45:57.9849194Z     },
2020-04-07T15:45:57.9849392Z     largest_niche: Some(
2020-04-07T15:45:57.9849627Z         Niche {
2020-04-07T15:45:57.9849859Z             offset: Size {
2020-04-07T15:45:57.9850109Z                 raw: 0,
2020-04-07T15:45:57.9850589Z             scalar: Scalar {
2020-04-07T15:45:57.9850852Z                 value: Int(
2020-04-07T15:45:57.9851096Z                     I32,
2020-04-07T15:45:57.9851341Z                     false,
2020-04-07T15:45:57.9851341Z                     false,
2020-04-07T15:45:57.9851558Z                 ),
2020-04-07T15:45:57.9851808Z                 valid_range: 0..=0,
2020-04-07T15:45:57.9852075Z             },
2020-04-07T15:45:57.9852253Z         },
2020-04-07T15:45:57.9852414Z     ),
2020-04-07T15:45:57.9852621Z     align: AbiAndPrefAlign {
2020-04-07T15:45:57.9852879Z         abi: Align {
2020-04-07T15:45:57.9853104Z             pow2: 2,
2020-04-07T15:45:57.9853518Z         pref: Align {
2020-04-07T15:45:57.9853746Z             pow2: 3,
2020-04-07T15:45:57.9853943Z         },
2020-04-07T15:45:57.9854116Z     },
2020-04-07T15:45:57.9854116Z     },
2020-04-07T15:45:57.9854294Z     size: Size {
2020-04-07T15:45:57.9854505Z         raw: 12,
2020-04-07T15:45:57.9854698Z     },
2020-04-07T15:45:57.9854841Z }
2020-04-07T15:45:57.9855626Z   --> /checkout/src/test/ui/layout/debug.rs:6:1
2020-04-07T15:45:57.9855874Z    |
2020-04-07T15:45:57.9856168Z LL | enum E { Foo, Bar(!, i32, i32) } //~ ERROR: layout_of
2020-04-07T15:45:57.9856694Z 
2020-04-07T15:45:57.9856694Z 
2020-04-07T15:45:57.9856908Z error: layout_of(S) = Layout {
2020-04-07T15:45:57.9857153Z     fields: Arbitrary {
2020-04-07T15:45:57.9857382Z         offsets: [
2020-04-07T15:45:57.9857841Z                 raw: 0,
2020-04-07T15:45:57.9858057Z             },
2020-04-07T15:45:57.9858266Z             Size {
2020-04-07T15:45:57.9858487Z                 raw: 0,
---
2020-04-07T15:45:57.9860787Z     },
2020-04-07T15:45:57.9860978Z     variants: Single {
2020-04-07T15:45:57.9861219Z         index: 0,
2020-04-07T15:45:57.9861402Z     },
2020-04-07T15:45:57.9861589Z     abi: ScalarPair(
2020-04-07T15:45:57.9861814Z         Scalar {
2020-04-07T15:45:57.9862031Z             value: Int(
2020-04-07T15:45:57.9862490Z                 true,
2020-04-07T15:45:57.9862691Z             ),
2020-04-07T15:45:57.9862942Z             valid_range: 0..=4294967295,
2020-04-07T15:45:57.9863202Z         },
---
2020-04-07T15:45:57.9864256Z             ),
2020-04-07T15:45:57.9864505Z             valid_range: 0..=4294967295,
2020-04-07T15:45:57.9864763Z         },
2020-04-07T15:45:57.9864923Z     ),
2020-04-07T15:45:57.9865122Z     largest_niche: None,
2020-04-07T15:45:57.9865385Z     align: AbiAndPrefAlign {
2020-04-07T15:45:57.9865883Z         abi: Align {
2020-04-07T15:45:57.9866105Z             pow2: 2,
2020-04-07T15:45:57.9866520Z         pref: Align {
2020-04-07T15:45:57.9866748Z             pow2: 3,
2020-04-07T15:45:57.9866944Z         },
2020-04-07T15:45:57.9867122Z     },
2020-04-07T15:45:57.9867122Z     },
2020-04-07T15:45:57.9867398Z     size: Size {
2020-04-07T15:45:57.9867614Z         raw: 8,
2020-04-07T15:45:57.9867809Z     },
2020-04-07T15:45:57.9867951Z }
2020-04-07T15:45:57.9868507Z   --> /checkout/src/test/ui/layout/debug.rs:9:1
2020-04-07T15:45:57.9868754Z    |
2020-04-07T15:45:57.9869076Z LL | struct S { f1: i32, f2: (), f3: i32 } //~ ERROR: layout_of
2020-04-07T15:45:57.9869652Z 
2020-04-07T15:45:57.9869652Z 
2020-04-07T15:45:57.9869864Z error: layout_of(U) = Layout {
2020-04-07T15:45:57.9870101Z     fields: Union(
2020-04-07T15:45:57.9870469Z     ),
2020-04-07T15:45:57.9870662Z     variants: Single {
2020-04-07T15:45:57.9870885Z         index: 0,
2020-04-07T15:45:57.9871067Z     },
2020-04-07T15:45:57.9871067Z     },
2020-04-07T15:45:57.9871268Z     abi: Aggregate {
2020-04-07T15:45:57.9871490Z         sized: true,
2020-04-07T15:45:57.9871678Z     },
2020-04-07T15:45:57.9871893Z     largest_niche: None,
2020-04-07T15:45:57.9872140Z     align: AbiAndPrefAlign {
2020-04-07T15:45:57.9872385Z         abi: Align {
2020-04-07T15:45:57.9872628Z             pow2: 2,
2020-04-07T15:45:57.9873061Z         pref: Align {
2020-04-07T15:45:57.9873288Z             pow2: 3,
2020-04-07T15:45:57.9873500Z         },
2020-04-07T15:45:57.9873659Z     },
2020-04-07T15:45:57.9873659Z     },
2020-04-07T15:45:57.9873837Z     size: Size {
2020-04-07T15:45:57.9874056Z         raw: 8,
2020-04-07T15:45:57.9874240Z     },
2020-04-07T15:45:57.9874381Z }
2020-04-07T15:45:57.9874873Z   --> /checkout/src/test/ui/layout/debug.rs:12:1
2020-04-07T15:45:57.9875119Z    |
2020-04-07T15:45:57.9875414Z LL | union U { f1: (i32, i32), f3: i32 } //~ ERROR: layout_of
2020-04-07T15:45:57.9875976Z 
2020-04-07T15:45:57.9875976Z 
2020-04-07T15:45:57.9876243Z error: layout_of(std::result::Result<i32, i32>) = Layout {
2020-04-07T15:45:57.9876557Z     fields: Arbitrary {
2020-04-07T15:45:57.9876801Z         offsets: [
2020-04-07T15:45:57.9877240Z                 raw: 0,
2020-04-07T15:45:57.9877473Z             },
2020-04-07T15:45:57.9877650Z         ],
2020-04-07T15:45:57.9877858Z         memory_index: [
---
2020-04-07T15:45:57.9879685Z                 false,
2020-04-07T15:45:57.9879887Z             ),
2020-04-07T15:45:57.9880117Z             valid_range: 0..=1,
2020-04-07T15:45:57.9880358Z         },
2020-04-07T15:45:57.9880565Z         discr_kind: Tag,
2020-04-07T15:45:57.9880809Z         discr_index: 0,
2020-04-07T15:45:57.9881276Z             Layout {
2020-04-07T15:45:57.9881733Z                 fields: Arbitrary {
2020-04-07T15:45:57.9882023Z                     offsets: [
2020-04-07T15:45:57.9882315Z                         Size {
---
2020-04-07T15:45:57.9884866Z                 },
2020-04-07T15:45:57.9885104Z                 abi: Aggregate {
2020-04-07T15:45:57.9885385Z                     sized: true,
2020-04-07T15:45:57.9885644Z                 },
2020-04-07T15:45:57.9885897Z                 largest_niche: None,
2020-04-07T15:45:57.9886207Z                 align: AbiAndPrefAlign {
2020-04-07T15:45:57.9886521Z                     abi: Align {
2020-04-07T15:45:57.9886880Z                         pow2: 2,
2020-04-07T15:45:57.9887411Z                     pref: Align {
2020-04-07T15:45:57.9887699Z                         pow2: 3,
2020-04-07T15:45:57.9887954Z                     },
2020-04-07T15:45:57.9888177Z                 },
---
2020-04-07T15:45:57.9892678Z                 },
2020-04-07T15:45:57.9892916Z                 abi: Aggregate {
2020-04-07T15:45:57.9893213Z                     sized: true,
2020-04-07T15:45:57.9893460Z                 },
2020-04-07T15:45:57.9893712Z                 largest_niche: None,
2020-04-07T15:45:57.9894038Z                 align: AbiAndPrefAlign {
2020-04-07T15:45:57.9894334Z                     abi: Align {
2020-04-07T15:45:57.9894619Z                         pow2: 2,
2020-04-07T15:45:57.9895137Z                     pref: Align {
2020-04-07T15:45:57.9895427Z                         pow2: 3,
2020-04-07T15:45:57.9895692Z                     },
2020-04-07T15:45:57.9895899Z                 },
2020-04-07T15:45:57.9895899Z                 },
2020-04-07T15:45:57.9896136Z                 size: Size {
2020-04-07T15:45:57.9896415Z                     raw: 8,
2020-04-07T15:45:57.9896647Z                 },
2020-04-07T15:45:57.9896838Z             },
2020-04-07T15:45:57.9897014Z         ],
2020-04-07T15:45:57.9897190Z     },
2020-04-07T15:45:57.9897376Z     abi: ScalarPair(
2020-04-07T15:45:57.9904229Z         Scalar {
2020-04-07T15:45:57.9904489Z             value: Int(
2020-04-07T15:45:57.9905072Z                 false,
2020-04-07T15:45:57.9905285Z             ),
2020-04-07T15:45:57.9905511Z             valid_range: 0..=1,
2020-04-07T15:45:57.9906799Z         },
---
2020-04-07T15:45:57.9907869Z             ),
2020-04-07T15:45:57.9908121Z             valid_range: 0..=4294967295,
2020-04-07T15:45:57.9908365Z         },
2020-04-07T15:45:57.9908552Z     ),
2020-04-07T15:45:57.9908755Z     largest_niche: Some(
2020-04-07T15:45:57.9908973Z         Niche {
2020-04-07T15:45:57.9909213Z             offset: Size {
2020-04-07T15:45:57.9909464Z                 raw: 0,
2020-04-07T15:45:57.9935018Z             scalar: Scalar {
2020-04-07T15:45:57.9935318Z                 value: Int(
2020-04-07T15:45:57.9935579Z                     I32,
2020-04-07T15:45:57.9935809Z                     false,
2020-04-07T15:45:57.9935809Z                     false,
2020-04-07T15:45:57.9936048Z                 ),
2020-04-07T15:45:57.9936300Z                 valid_range: 0..=1,
2020-04-07T15:45:57.9936543Z             },
2020-04-07T15:45:57.9936775Z         },
2020-04-07T15:45:57.9936938Z     ),
2020-04-07T15:45:57.9937142Z     align: AbiAndPrefAlign {
2020-04-07T15:45:57.9937382Z         abi: Align {
2020-04-07T15:45:57.9937628Z             pow2: 2,
2020-04-07T15:45:57.9938027Z         pref: Align {
2020-04-07T15:45:57.9938298Z             pow2: 3,
2020-04-07T15:45:57.9938650Z         },
2020-04-07T15:45:57.9938826Z     },
2020-04-07T15:45:57.9938826Z     },
2020-04-07T15:45:57.9939024Z     size: Size {
2020-04-07T15:45:57.9939231Z         raw: 8,
2020-04-07T15:45:57.9939457Z     },
2020-04-07T15:45:57.9939602Z }
2020-04-07T15:45:57.9940457Z   --> /checkout/src/test/ui/layout/debug.rs:15:1
2020-04-07T15:45:57.9940710Z    |
2020-04-07T15:45:57.9940980Z LL | type Test = Result<i32, i32>; //~ ERROR: layout_of
2020-04-07T15:45:57.9941517Z 
2020-04-07T15:45:57.9941517Z 
2020-04-07T15:45:57.9941716Z error: layout_of(i32) = Layout {
2020-04-07T15:45:57.9941981Z     fields: Primitive,
2020-04-07T15:45:57.9942212Z     variants: Single {
2020-04-07T15:45:57.9942435Z         index: 0,
2020-04-07T15:45:57.9942813Z     abi: Scalar(
2020-04-07T15:45:57.9945765Z         Scalar {
2020-04-07T15:45:57.9946131Z             value: Int(
2020-04-07T15:45:57.9946387Z                 I32,
2020-04-07T15:45:57.9946387Z                 I32,
2020-04-07T15:45:57.9946599Z                 true,
2020-04-07T15:45:57.9946811Z             ),
2020-04-07T15:45:57.9947082Z             valid_range: 0..=4294967295,
2020-04-07T15:45:57.9947326Z         },
2020-04-07T15:45:57.9947486Z     ),
2020-04-07T15:45:57.9947704Z     largest_niche: None,
2020-04-07T15:45:57.9947952Z     align: AbiAndPrefAlign {
2020-04-07T15:45:57.9948191Z         abi: Align {
2020-04-07T15:45:57.9948416Z             pow2: 2,
2020-04-07T15:45:57.9952617Z         pref: Align {
2020-04-07T15:45:57.9952847Z             pow2: 2,
2020-04-07T15:45:57.9953074Z         },
2020-04-07T15:45:57.9953234Z     },
2020-04-07T15:45:57.9953234Z     },
2020-04-07T15:45:57.9953414Z     size: Size {
2020-04-07T15:45:57.9953634Z         raw: 4,
2020-04-07T15:45:57.9953813Z     },
2020-04-07T15:45:57.9953955Z }
2020-04-07T15:45:57.9954597Z   --> /checkout/src/test/ui/layout/debug.rs:18:1
2020-04-07T15:45:57.9954865Z    |
2020-04-07T15:45:57.9956669Z LL | type T = impl std::fmt::Debug; //~ ERROR: layout_of
2020-04-07T15:45:57.9958801Z 
2020-04-07T15:45:57.9959015Z error: aborting due to 5 previous errors
2020-04-07T15:45:57.9959208Z 
2020-04-07T15:45:57.9959318Z 
---
2020-04-07T15:45:57.9970326Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-07T15:45:57.9970803Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T15:45:57.9971069Z 
2020-04-07T15:45:57.9971198Z 
2020-04-07T15:45:57.9975800Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T15:45:57.9979068Z 
2020-04-07T15:45:57.9979182Z 
2020-04-07T15:45:57.9979884Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T15:45:57.9980344Z Build completed unsuccessfully in 1:10:37
2020-04-07T15:45:57.9980344Z Build completed unsuccessfully in 1:10:37
2020-04-07T15:45:57.9980629Z == clock drift check ==
2020-04-07T15:45:57.9980908Z   local time: Tue Apr  7 15:45:57 UTC 2020
2020-04-07T15:45:58.2918606Z   network time: Tue, 07 Apr 2020 15:45:58 GMT
2020-04-07T15:45:58.7259966Z 
2020-04-07T15:45:58.7259966Z 
2020-04-07T15:45:58.7368765Z ##[error]Bash exited with code '1'.
2020-04-07T15:45:58.7383648Z ##[section]Finishing: Run build
2020-04-07T15:45:58.7455726Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70629/merge to s
2020-04-07T15:45:58.7460162Z Task         : Get sources
2020-04-07T15:45:58.7460471Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T15:45:58.7461492Z Version      : 1.0.0
2020-04-07T15:45:58.7461751Z Author       : Microsoft
2020-04-07T15:45:58.7461751Z Author       : Microsoft
2020-04-07T15:45:58.7462115Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T15:45:58.7462531Z ==============================================================================
2020-04-07T15:45:59.1228210Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T15:45:59.1277995Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70629/merge to s
2020-04-07T15:45:59.1399255Z Cleaning up task key
2020-04-07T15:45:59.1400528Z Start cleaning up orphan processes.
2020-04-07T15:45:59.1593528Z Terminate orphan process: pid (3497) (python)
2020-04-07T15:45:59.1780862Z ##[section]Finishing: Finalize Job
