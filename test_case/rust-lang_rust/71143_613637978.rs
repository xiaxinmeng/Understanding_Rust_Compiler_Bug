plain
2020-04-14T18:26:59.9857423Z ========================== Starting Command Output ===========================
2020-04-14T18:26:59.9860198Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/454fb693-134b-42c9-a0a7-2e190c5d223f.sh
2020-04-14T18:26:59.9860497Z 
2020-04-14T18:26:59.9864938Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T18:26:59.9886440Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71143/merge to s
2020-04-14T18:26:59.9890010Z Task         : Get sources
2020-04-14T18:26:59.9890354Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T18:26:59.9890682Z Version      : 1.0.0
2020-04-14T18:26:59.9890900Z Author       : Microsoft
---
2020-04-14T18:27:00.9996961Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T18:27:01.0003795Z ##[command]git config gc.auto 0
2020-04-14T18:27:01.0008166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T18:27:01.0012363Z ##[command]git config --get-all http.proxy
2020-04-14T18:27:01.0019738Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71143/merge:refs/remotes/pull/71143/merge
---
2020-04-14T18:30:15.7538155Z  ---> f58a2bb1e753
2020-04-14T18:30:15.7538874Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T18:30:15.7539648Z  ---> Using cache
2020-04-14T18:30:15.7539963Z  ---> d079cc6b6db8
2020-04-14T18:30:15.7540871Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T18:30:15.7542918Z  ---> 4183ca46ee56
2020-04-14T18:30:15.7543193Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T18:30:15.7543632Z  ---> Using cache
2020-04-14T18:30:15.7544023Z  ---> 69e7f8a2a2fb
---
2020-04-14T18:30:15.7908906Z Looks like docker image is the same as before, not uploading
2020-04-14T18:30:22.1516923Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T18:30:22.1749414Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T18:30:22.1776789Z == clock drift check ==
2020-04-14T18:30:22.1783574Z   local time: Tue Apr 14 18:30:22 UTC 2020
2020-04-14T18:30:22.3806867Z   network time: Tue, 14 Apr 2020 18:30:22 GMT
2020-04-14T18:30:22.3828068Z Starting sccache server...
2020-04-14T18:30:22.4639031Z configure: processing command line
2020-04-14T18:30:22.4639333Z configure: 
2020-04-14T18:30:22.4640258Z configure: rust.dist-src        := False
---
2020-04-14T18:35:11.5321031Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T18:35:12.7978289Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T18:35:14.1718355Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T18:35:14.5821445Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T18:35:23.2281209Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T18:35:24.6793107Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T18:35:28.5895977Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T18:35:32.2956792Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T18:35:41.3556644Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T18:55:56.1616629Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T18:55:57.6754916Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T18:55:59.4079544Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T18:56:00.6512882Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T18:56:10.7730896Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T18:56:12.9834388Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T18:56:17.8880236Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T18:56:22.8295350Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T18:56:33.5616663Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T19:19:11.7673941Z .................................................................................................... 1700/9895
2020-04-14T19:19:16.0568750Z .................................................................................................... 1800/9895
2020-04-14T19:19:23.7616157Z .................................................................................................... 1900/9895
2020-04-14T19:19:31.2117481Z .....i.............................................................................................. 2000/9895
2020-04-14T19:19:37.1010970Z ...............................................................................................iiiii 2100/9895
2020-04-14T19:19:55.5372492Z .................................................................................................... 2300/9895
2020-04-14T19:19:57.4556508Z .................................................................................................... 2400/9895
2020-04-14T19:19:59.4045124Z .................................................................................................... 2500/9895
2020-04-14T19:20:04.6530764Z .................................................................................................... 2600/9895
---
2020-04-14T19:22:46.0064902Z .................................................................................................... 5100/9895
2020-04-14T19:22:53.2114515Z .................................................................................................... 5200/9895
2020-04-14T19:22:57.5497662Z ................i................................................................................... 5300/9895
2020-04-14T19:23:06.7094215Z ......i............................................................................................. 5400/9895
2020-04-14T19:23:11.5461601Z ......ii.ii........i...i............................................................................ 5500/9895
2020-04-14T19:23:14.6758184Z ..............................................i......................................FF.F........... 5600/9895
2020-04-14T19:23:27.7764433Z ........................................................................ii.......................... 5800/9895
2020-04-14T19:23:33.8682477Z ...........i........................................................................................ 5900/9895
2020-04-14T19:23:38.9509540Z .................................................................................................... 6000/9895
2020-04-14T19:23:48.6245141Z .................................................................................................... 6100/9895
2020-04-14T19:23:48.6245141Z .................................................................................................... 6100/9895
2020-04-14T19:23:58.9356392Z .....ii...i..ii...........i......................................................................... 6200/9895
2020-04-14T19:24:11.1613226Z .................................................................................................... 6400/9895
2020-04-14T19:24:14.0659930Z .................................................................................................... 6500/9895
2020-04-14T19:24:14.0659930Z .................................................................................................... 6500/9895
2020-04-14T19:24:24.8492139Z ...................................i..ii............................................................ 6600/9895
2020-04-14T19:24:44.2866798Z .................................................................................................... 6800/9895
2020-04-14T19:24:46.0487325Z ...................................i................................................................ 6900/9895
2020-04-14T19:24:47.8603894Z .................................................................................................... 7000/9895
2020-04-14T19:24:49.8104202Z ..........................................................................i......................... 7100/9895
---
2020-04-14T19:26:18.7328137Z .................................................................................................... 7800/9895
2020-04-14T19:26:22.5380045Z .................................................................................................... 7900/9895
2020-04-14T19:26:28.6204031Z .................................................................................................... 8000/9895
2020-04-14T19:26:34.5017871Z ........................................i........................................................... 8100/9895
2020-04-14T19:26:43.2080415Z ........................................................................................iiiiii.iiiii 8200/9895
2020-04-14T19:26:49.1505020Z .i.................................................................................................. 8300/9895
2020-04-14T19:27:01.0972479Z .................................................................................................... 8500/9895
2020-04-14T19:27:10.7547598Z .................................................................................................... 8600/9895
2020-04-14T19:27:23.8998671Z .................................................................................................... 8700/9895
2020-04-14T19:27:29.8766213Z .................................................................................................... 8800/9895
---
2020-04-14T19:29:12.5660267Z 151 
2020-04-14T19:29:12.5660363Z 
2020-04-14T19:29:12.5660451Z 
2020-04-14T19:29:12.5660681Z The actual stderr differed from the expected stderr.
2020-04-14T19:29:12.5661713Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/lint-exceeding-bitshifts.noopt.stderr
2020-04-14T19:29:12.5662347Z To update references, rerun the tests and pass the `--bless` flag
2020-04-14T19:29:12.5662927Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`
2020-04-14T19:29:12.5663150Z 
2020-04-14T19:29:12.5663361Z error in revision `noopt`: 1 errors occurred comparing output.
2020-04-14T19:29:12.5663629Z status: exit code: 0
2020-04-14T19:29:12.5665775Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/auxiliary"
2020-04-14T19:29:12.5667228Z ------------------------------------------
2020-04-14T19:29:12.5667381Z 
2020-04-14T19:29:12.5667737Z ------------------------------------------
2020-04-14T19:29:12.5667922Z stderr:
2020-04-14T19:29:12.5667922Z stderr:
2020-04-14T19:29:12.5668273Z ------------------------------------------
2020-04-14T19:29:12.5668533Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5669025Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18:20
2020-04-14T19:29:12.5669255Z    |
2020-04-14T19:29:12.5669556Z LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5670299Z    |
2020-04-14T19:29:12.5670488Z note: the lint level is defined here
2020-04-14T19:29:12.5670969Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
2020-04-14T19:29:12.5671190Z    |
2020-04-14T19:29:12.5671190Z    |
2020-04-14T19:29:12.5671389Z LL | #![warn(arithmetic_overflow, const_err)]
2020-04-14T19:29:12.5671609Z    |         ^^^^^^^^^^^^^^^^^^^
2020-04-14T19:29:12.5671940Z 
2020-04-14T19:29:12.5672124Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5672644Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:22:13
2020-04-14T19:29:12.5672869Z    |
2020-04-14T19:29:12.5673115Z LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5673632Z 
2020-04-14T19:29:12.5673814Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5674322Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:27:15
2020-04-14T19:29:12.5674628Z    |
2020-04-14T19:29:12.5674628Z    |
2020-04-14T19:29:12.5674884Z LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5675414Z 
2020-04-14T19:29:12.5675595Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5676096Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:29:15
2020-04-14T19:29:12.5676448Z    |
2020-04-14T19:29:12.5676448Z    |
2020-04-14T19:29:12.5676684Z LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5677185Z 
2020-04-14T19:29:12.5677353Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5677814Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:31:15
2020-04-14T19:29:12.5678038Z    |
2020-04-14T19:29:12.5678038Z    |
2020-04-14T19:29:12.5678268Z LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5678752Z 
2020-04-14T19:29:12.5678933Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5679386Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:33:15
2020-04-14T19:29:12.5679595Z    |
2020-04-14T19:29:12.5679595Z    |
2020-04-14T19:29:12.5679845Z LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5680332Z 
2020-04-14T19:29:12.5680518Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5680975Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:35:15
2020-04-14T19:29:12.5681183Z    |
2020-04-14T19:29:12.5681183Z    |
2020-04-14T19:29:12.5681431Z LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5681911Z 
2020-04-14T19:29:12.5682086Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5682556Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:37:15
2020-04-14T19:29:12.5682763Z    |
2020-04-14T19:29:12.5682763Z    |
2020-04-14T19:29:12.5682997Z LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5683499Z 
2020-04-14T19:29:12.5683665Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5684142Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:39:15
2020-04-14T19:29:12.5684351Z    |
2020-04-14T19:29:12.5684351Z    |
2020-04-14T19:29:12.5684586Z LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5685080Z 
2020-04-14T19:29:12.5685249Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5685709Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:41:15
2020-04-14T19:29:12.5685936Z    |
2020-04-14T19:29:12.5685936Z    |
2020-04-14T19:29:12.5686168Z LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5686704Z 
2020-04-14T19:29:12.5686873Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5687339Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:44:15
2020-04-14T19:29:12.5687626Z    |
2020-04-14T19:29:12.5687626Z    |
2020-04-14T19:29:12.5687860Z LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5688341Z 
2020-04-14T19:29:12.5688522Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5688989Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:46:15
2020-04-14T19:29:12.5689197Z    |
2020-04-14T19:29:12.5689197Z    |
2020-04-14T19:29:12.5689445Z LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5689975Z 
2020-04-14T19:29:12.5690143Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5690620Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:48:15
2020-04-14T19:29:12.5690827Z    |
2020-04-14T19:29:12.5690827Z    |
2020-04-14T19:29:12.5691057Z LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5691559Z 
2020-04-14T19:29:12.5691725Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5692193Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:50:15
2020-04-14T19:29:12.5692398Z    |
2020-04-14T19:29:12.5692398Z    |
2020-04-14T19:29:12.5692631Z LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5693127Z 
2020-04-14T19:29:12.5693301Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5693752Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:52:15
2020-04-14T19:29:12.5693974Z    |
2020-04-14T19:29:12.5693974Z    |
2020-04-14T19:29:12.5694204Z LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5694702Z 
2020-04-14T19:29:12.5694873Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5695324Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:54:15
2020-04-14T19:29:12.5695546Z    |
2020-04-14T19:29:12.5695546Z    |
2020-04-14T19:29:12.5695777Z LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5696258Z 
2020-04-14T19:29:12.5696440Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5696888Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:56:15
2020-04-14T19:29:12.5697099Z    |
2020-04-14T19:29:12.5697099Z    |
2020-04-14T19:29:12.5697348Z LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5697828Z 
2020-04-14T19:29:12.5698008Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5698457Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:58:15
2020-04-14T19:29:12.5698666Z    |
2020-04-14T19:29:12.5698666Z    |
2020-04-14T19:29:12.5698914Z LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5699391Z 
2020-04-14T19:29:12.5699558Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5700025Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:62:15
2020-04-14T19:29:12.5700234Z    |
2020-04-14T19:29:12.5700234Z    |
2020-04-14T19:29:12.5700461Z LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5700946Z 
2020-04-14T19:29:12.5701112Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5701576Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:64:15
2020-04-14T19:29:12.5701785Z    |
2020-04-14T19:29:12.5701785Z    |
2020-04-14T19:29:12.5702204Z LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5702754Z 
2020-04-14T19:29:12.5702921Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5703379Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:69:15
2020-04-14T19:29:12.5703604Z    |
2020-04-14T19:29:12.5703604Z    |
2020-04-14T19:29:12.5703842Z LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5704346Z 
2020-04-14T19:29:12.5704555Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5705020Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:71:15
2020-04-14T19:29:12.5705244Z    |
2020-04-14T19:29:12.5705244Z    |
2020-04-14T19:29:12.5705484Z LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5705995Z 
2020-04-14T19:29:12.5706177Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5706630Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:77:15
2020-04-14T19:29:12.5706837Z    |
2020-04-14T19:29:12.5706837Z    |
2020-04-14T19:29:12.5707091Z LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5707593Z 
2020-04-14T19:29:12.5707777Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5708236Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:78:15
2020-04-14T19:29:12.5708448Z    |
2020-04-14T19:29:12.5708448Z    |
2020-04-14T19:29:12.5708689Z LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5709206Z 
2020-04-14T19:29:12.5709347Z warning: 24 warnings emitted
2020-04-14T19:29:12.5709486Z 
2020-04-14T19:29:12.5709571Z 
---
2020-04-14T19:29:12.5712049Z 151 
2020-04-14T19:29:12.5712141Z 
2020-04-14T19:29:12.5712223Z 
2020-04-14T19:29:12.5712410Z The actual stderr differed from the expected stderr.
2020-04-14T19:29:12.5713029Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/lint-exceeding-bitshifts.opt.stderr
2020-04-14T19:29:12.5713589Z To update references, rerun the tests and pass the `--bless` flag
2020-04-14T19:29:12.5714126Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`
2020-04-14T19:29:12.5714525Z error in revision `opt`: 1 errors occurred comparing output.
2020-04-14T19:29:12.5714769Z status: exit code: 0
2020-04-14T19:29:12.5714769Z status: exit code: 0
2020-04-14T19:29:12.5717141Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/auxiliary"
2020-04-14T19:29:12.5720026Z ------------------------------------------
2020-04-14T19:29:12.5720204Z 
2020-04-14T19:29:12.5720679Z ------------------------------------------
2020-04-14T19:29:12.5720868Z stderr:
2020-04-14T19:29:12.5720868Z stderr:
2020-04-14T19:29:12.5721220Z ------------------------------------------
2020-04-14T19:29:12.5721486Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5721994Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18:20
2020-04-14T19:29:12.5722223Z    |
2020-04-14T19:29:12.5722521Z LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5723215Z    |
2020-04-14T19:29:12.5723399Z note: the lint level is defined here
2020-04-14T19:29:12.5723881Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
2020-04-14T19:29:12.5724101Z    |
2020-04-14T19:29:12.5724101Z    |
2020-04-14T19:29:12.5724300Z LL | #![warn(arithmetic_overflow, const_err)]
2020-04-14T19:29:12.5724524Z    |         ^^^^^^^^^^^^^^^^^^^
2020-04-14T19:29:12.5724659Z 
2020-04-14T19:29:12.5724842Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5725343Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:22:13
2020-04-14T19:29:12.5725562Z    |
2020-04-14T19:29:12.5725812Z LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5726326Z 
2020-04-14T19:29:12.5726508Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5727020Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:27:15
2020-04-14T19:29:12.5727244Z    |
2020-04-14T19:29:12.5727244Z    |
2020-04-14T19:29:12.5730025Z LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5730575Z 
2020-04-14T19:29:12.5730746Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5731334Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:29:15
2020-04-14T19:29:12.5731558Z    |
2020-04-14T19:29:12.5731558Z    |
2020-04-14T19:29:12.5731793Z LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5732296Z 
2020-04-14T19:29:12.5732466Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5732919Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:31:15
2020-04-14T19:29:12.5733143Z    |
2020-04-14T19:29:12.5733143Z    |
2020-04-14T19:29:12.5733380Z LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5733859Z 
2020-04-14T19:29:12.5734044Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5734496Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:33:15
2020-04-14T19:29:12.5734705Z    |
2020-04-14T19:29:12.5734705Z    |
2020-04-14T19:29:12.5734960Z LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5735444Z 
2020-04-14T19:29:12.5735629Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5736271Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:35:15
2020-04-14T19:29:12.5736497Z    |
2020-04-14T19:29:12.5736497Z    |
2020-04-14T19:29:12.5736765Z LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5737282Z 
2020-04-14T19:29:12.5737709Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5738241Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:37:15
2020-04-14T19:29:12.5738466Z    |
2020-04-14T19:29:12.5738466Z    |
2020-04-14T19:29:12.5738718Z LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5739377Z 
2020-04-14T19:29:12.5739559Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5740080Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:39:15
2020-04-14T19:29:12.5740308Z    |
2020-04-14T19:29:12.5740308Z    |
2020-04-14T19:29:12.5740559Z LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5741094Z 
2020-04-14T19:29:12.5741274Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5741813Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:41:15
2020-04-14T19:29:12.5742059Z    |
2020-04-14T19:29:12.5742059Z    |
2020-04-14T19:29:12.5742308Z LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5742841Z 
2020-04-14T19:29:12.5743022Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5743524Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:44:15
2020-04-14T19:29:12.5743763Z    |
2020-04-14T19:29:12.5743763Z    |
2020-04-14T19:29:12.5744013Z LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5744531Z 
2020-04-14T19:29:12.5744729Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5745223Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:46:15
2020-04-14T19:29:12.5745447Z    |
2020-04-14T19:29:12.5745447Z    |
2020-04-14T19:29:12.5745718Z LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5746241Z 
2020-04-14T19:29:12.5746425Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5746931Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:48:15
2020-04-14T19:29:12.5747152Z    |
2020-04-14T19:29:12.5747152Z    |
2020-04-14T19:29:12.5747402Z LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5747941Z 
2020-04-14T19:29:12.5748121Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5748625Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:50:15
2020-04-14T19:29:12.5748847Z    |
2020-04-14T19:29:12.5748847Z    |
2020-04-14T19:29:12.5749094Z LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5749629Z 
2020-04-14T19:29:12.5749812Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5758363Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:52:15
2020-04-14T19:29:12.5758626Z    |
2020-04-14T19:29:12.5758626Z    |
2020-04-14T19:29:12.5758861Z LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5759373Z 
2020-04-14T19:29:12.5759545Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5760006Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:54:15
2020-04-14T19:29:12.5760231Z    |
2020-04-14T19:29:12.5760231Z    |
2020-04-14T19:29:12.5760466Z LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5762122Z 
2020-04-14T19:29:12.5762312Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5762846Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:56:15
2020-04-14T19:29:12.5763055Z    |
2020-04-14T19:29:12.5763055Z    |
2020-04-14T19:29:12.5763306Z LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5763790Z 
2020-04-14T19:29:12.5763975Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5764573Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:58:15
2020-04-14T19:29:12.5765385Z    |
2020-04-14T19:29:12.5765385Z    |
2020-04-14T19:29:12.5765642Z LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5766127Z 
2020-04-14T19:29:12.5766298Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5767018Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:62:15
2020-04-14T19:29:12.5767241Z    |
2020-04-14T19:29:12.5767241Z    |
2020-04-14T19:29:12.5772781Z LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5774110Z 
2020-04-14T19:29:12.5774297Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5775000Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:64:15
2020-04-14T19:29:12.5775229Z    |
2020-04-14T19:29:12.5775229Z    |
2020-04-14T19:29:12.5775698Z LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5776250Z 
2020-04-14T19:29:12.5776434Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5776926Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:69:15
2020-04-14T19:29:12.5777169Z    |
2020-04-14T19:29:12.5777169Z    |
2020-04-14T19:29:12.5777426Z LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5777972Z 
2020-04-14T19:29:12.5778160Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5778649Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:71:15
2020-04-14T19:29:12.5778889Z    |
2020-04-14T19:29:12.5778889Z    |
2020-04-14T19:29:12.5779151Z LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5779700Z 
2020-04-14T19:29:12.5779898Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5780387Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:77:15
2020-04-14T19:29:12.5780612Z    |
2020-04-14T19:29:12.5780612Z    |
2020-04-14T19:29:12.5780885Z LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5781427Z 
2020-04-14T19:29:12.5781625Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5782118Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:78:15
2020-04-14T19:29:12.5782342Z    |
2020-04-14T19:29:12.5782342Z    |
2020-04-14T19:29:12.5782602Z LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5783158Z 
2020-04-14T19:29:12.5783310Z warning: 24 warnings emitted
2020-04-14T19:29:12.5783463Z 
2020-04-14T19:29:12.5783551Z 
---
2020-04-14T19:29:12.5786504Z 151 
2020-04-14T19:29:12.5786603Z 
2020-04-14T19:29:12.5786694Z 
2020-04-14T19:29:12.5786904Z The actual stderr differed from the expected stderr.
2020-04-14T19:29:12.5787717Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks/lint-exceeding-bitshifts.opt_with_overflow_checks.stderr
2020-04-14T19:29:12.5788499Z To update references, rerun the tests and pass the `--bless` flag
2020-04-14T19:29:12.5789054Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`
2020-04-14T19:29:12.5789276Z 
2020-04-14T19:29:12.5789524Z error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
2020-04-14T19:29:12.5789824Z status: exit code: 0
2020-04-14T19:29:12.5792024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks/auxiliary"
2020-04-14T19:29:12.5793591Z ------------------------------------------
2020-04-14T19:29:12.5793762Z 
2020-04-14T19:29:12.5794108Z ------------------------------------------
2020-04-14T19:29:12.5794291Z stderr:
2020-04-14T19:29:12.5794291Z stderr:
2020-04-14T19:29:12.5794662Z ------------------------------------------
2020-04-14T19:29:12.5794907Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5795400Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18:20
2020-04-14T19:29:12.5795645Z    |
2020-04-14T19:29:12.5795929Z LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5796529Z    |
2020-04-14T19:29:12.5796697Z note: the lint level is defined here
2020-04-14T19:29:12.5797169Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:9:9
2020-04-14T19:29:12.5797408Z    |
2020-04-14T19:29:12.5797408Z    |
2020-04-14T19:29:12.5797589Z LL | #![warn(arithmetic_overflow, const_err)]
2020-04-14T19:29:12.5797807Z    |         ^^^^^^^^^^^^^^^^^^^
2020-04-14T19:29:12.5797942Z 
2020-04-14T19:29:12.5798138Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5798624Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:22:13
2020-04-14T19:29:12.5798847Z    |
2020-04-14T19:29:12.5799110Z LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5799847Z 
2020-04-14T19:29:12.5800029Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5800546Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:27:15
2020-04-14T19:29:12.5800770Z    |
2020-04-14T19:29:12.5800770Z    |
2020-04-14T19:29:12.5801021Z LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5801558Z 
2020-04-14T19:29:12.5801740Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5802240Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:29:15
2020-04-14T19:29:12.5802465Z    |
2020-04-14T19:29:12.5802465Z    |
2020-04-14T19:29:12.5802715Z LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5803248Z 
2020-04-14T19:29:12.5803708Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5804232Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:31:15
2020-04-14T19:29:12.5804471Z    |
2020-04-14T19:29:12.5804471Z    |
2020-04-14T19:29:12.5804723Z LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5805252Z 
2020-04-14T19:29:12.5805433Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5806025Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:33:15
2020-04-14T19:29:12.5806264Z    |
2020-04-14T19:29:12.5806264Z    |
2020-04-14T19:29:12.5806514Z LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5807034Z 
2020-04-14T19:29:12.5807231Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5807718Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:35:15
2020-04-14T19:29:12.5807942Z    |
2020-04-14T19:29:12.5807942Z    |
2020-04-14T19:29:12.5808260Z LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5808777Z 
2020-04-14T19:29:12.5808973Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5809467Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:37:15
2020-04-14T19:29:12.5809694Z    |
2020-04-14T19:29:12.5809694Z    |
2020-04-14T19:29:12.5809943Z LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5810476Z 
2020-04-14T19:29:12.5810654Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5811155Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:39:15
2020-04-14T19:29:12.5811379Z    |
2020-04-14T19:29:12.5811379Z    |
2020-04-14T19:29:12.5811630Z LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5812171Z 
2020-04-14T19:29:12.5812351Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5812835Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:41:15
2020-04-14T19:29:12.5813077Z    |
2020-04-14T19:29:12.5813077Z    |
2020-04-14T19:29:12.5813328Z LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5814208Z 
2020-04-14T19:29:12.5814389Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5814891Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:44:15
2020-04-14T19:29:12.5815132Z    |
2020-04-14T19:29:12.5815132Z    |
2020-04-14T19:29:12.5815383Z LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5815900Z 
2020-04-14T19:29:12.5816098Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5816586Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:46:15
2020-04-14T19:29:12.5816807Z    |
2020-04-14T19:29:12.5816807Z    |
2020-04-14T19:29:12.5817077Z LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5817598Z 
2020-04-14T19:29:12.5817794Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5818283Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:48:15
2020-04-14T19:29:12.5818504Z    |
2020-04-14T19:29:12.5818504Z    |
2020-04-14T19:29:12.5818771Z LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5819292Z 
2020-04-14T19:29:12.5819471Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5819972Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:50:15
2020-04-14T19:29:12.5820194Z    |
2020-04-14T19:29:12.5820194Z    |
2020-04-14T19:29:12.5820450Z LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5820987Z 
2020-04-14T19:29:12.5821167Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5821667Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:52:15
2020-04-14T19:29:12.5821953Z    |
2020-04-14T19:29:12.5821953Z    |
2020-04-14T19:29:12.5822204Z LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5822736Z 
2020-04-14T19:29:12.5822915Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5823409Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:54:15
2020-04-14T19:29:12.5823649Z    |
2020-04-14T19:29:12.5823649Z    |
2020-04-14T19:29:12.5823899Z LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5824499Z 
2020-04-14T19:29:12.5824679Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5825171Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:56:15
2020-04-14T19:29:12.5825407Z    |
2020-04-14T19:29:12.5825407Z    |
2020-04-14T19:29:12.5825655Z LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5826280Z 
2020-04-14T19:29:12.5826463Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5826910Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:58:15
2020-04-14T19:29:12.5827117Z    |
2020-04-14T19:29:12.5827117Z    |
2020-04-14T19:29:12.5827365Z LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5827843Z 
2020-04-14T19:29:12.5828024Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5828474Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:62:15
2020-04-14T19:29:12.5828682Z    |
2020-04-14T19:29:12.5828682Z    |
2020-04-14T19:29:12.5828908Z LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5829388Z 
2020-04-14T19:29:12.5829554Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5830458Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:64:15
2020-04-14T19:29:12.5830669Z    |
2020-04-14T19:29:12.5830669Z    |
2020-04-14T19:29:12.5831094Z LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5831589Z 
2020-04-14T19:29:12.5831755Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5832207Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:69:15
2020-04-14T19:29:12.5832431Z    |
2020-04-14T19:29:12.5832431Z    |
2020-04-14T19:29:12.5832957Z LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5833699Z 
2020-04-14T19:29:12.5833881Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5834728Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:71:15
2020-04-14T19:29:12.5834978Z    |
2020-04-14T19:29:12.5834978Z    |
2020-04-14T19:29:12.5835531Z LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5836097Z 
2020-04-14T19:29:12.5836279Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5836781Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:77:15
2020-04-14T19:29:12.5837005Z    |
2020-04-14T19:29:12.5837005Z    |
2020-04-14T19:29:12.5837283Z LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5837829Z 
2020-04-14T19:29:12.5838025Z warning: this arithmetic operation will overflow
2020-04-14T19:29:12.5838513Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:78:15
2020-04-14T19:29:12.5838736Z    |
2020-04-14T19:29:12.5838736Z    |
2020-04-14T19:29:12.5839008Z LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
2020-04-14T19:29:12.5839628Z 
2020-04-14T19:29:12.5839782Z warning: 24 warnings emitted
2020-04-14T19:29:12.5839931Z 
2020-04-14T19:29:12.5840018Z 
---
2020-04-14T19:29:12.5843562Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-14T19:29:12.5843925Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-14T19:29:12.5844145Z 
2020-04-14T19:29:12.5844234Z 
2020-04-14T19:29:12.5847567Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-14T19:29:12.5849840Z 
2020-04-14T19:29:12.5849929Z 
2020-04-14T19:29:12.5850467Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-14T19:29:12.5850813Z Build completed unsuccessfully in 0:57:14
2020-04-14T19:29:12.5850813Z Build completed unsuccessfully in 0:57:14
2020-04-14T19:29:12.5851033Z == clock drift check ==
2020-04-14T19:29:12.5851255Z   local time: Tue Apr 14 19:29:12 UTC 2020
2020-04-14T19:29:12.7725730Z   network time: Tue, 14 Apr 2020 19:29:12 GMT
2020-04-14T19:29:13.5510223Z 
2020-04-14T19:29:13.5510223Z 
2020-04-14T19:29:13.5576398Z ##[error]Bash exited with code '1'.
2020-04-14T19:29:13.5591083Z ##[section]Finishing: Run build
2020-04-14T19:29:13.5636351Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71143/merge to s
2020-04-14T19:29:13.5641094Z Task         : Get sources
2020-04-14T19:29:13.5641395Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T19:29:13.5641674Z Version      : 1.0.0
2020-04-14T19:29:13.5641891Z Author       : Microsoft
2020-04-14T19:29:13.5641891Z Author       : Microsoft
2020-04-14T19:29:13.5642212Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T19:29:13.5642563Z ==============================================================================
2020-04-14T19:29:13.8785434Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T19:29:13.8837624Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71143/merge to s
2020-04-14T19:29:13.8927758Z Cleaning up task key
2020-04-14T19:29:13.8929028Z Start cleaning up orphan processes.
2020-04-14T19:29:13.9102619Z Terminate orphan process: pid (3625) (python)
2020-04-14T19:29:13.9252759Z ##[section]Finishing: Finalize Job
