plain
2020-04-13T16:30:40.5155642Z ========================== Starting Command Output ===========================
2020-04-13T16:30:40.5159675Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9e9de948-9438-4614-82e2-c0cb7fe0b9d7.sh
2020-04-13T16:30:40.5160058Z 
2020-04-13T16:30:40.5164508Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T16:30:40.5183475Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T16:30:40.5187285Z Task         : Get sources
2020-04-13T16:30:40.5187598Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T16:30:40.5187900Z Version      : 1.0.0
2020-04-13T16:30:40.5188094Z Author       : Microsoft
---
2020-04-13T16:30:41.7370150Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T16:30:41.7505924Z ##[command]git config gc.auto 0
2020-04-13T16:30:41.7566681Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T16:30:41.7610066Z ##[command]git config --get-all http.proxy
2020-04-13T16:30:41.7699027Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71003/merge:refs/remotes/pull/71003/merge
---
2020-04-13T16:34:16.3019204Z Looks like docker image is the same as before, not uploading
2020-04-13T16:34:25.8799567Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T16:34:25.9147989Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T16:34:25.9171409Z == clock drift check ==
2020-04-13T16:34:25.9185462Z   local time: Mon Apr 13 16:34:25 UTC 2020
2020-04-13T16:34:25.9712885Z   network time: Mon, 13 Apr 2020 16:34:25 GMT
2020-04-13T16:34:25.9728044Z Starting sccache server...
2020-04-13T16:34:26.0617975Z configure: processing command line
2020-04-13T16:34:26.0618219Z configure: 
2020-04-13T16:34:26.0619105Z configure: rust.dist-src        := False
---
2020-04-13T16:39:26.7645717Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T16:39:28.1757950Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T16:39:29.7123855Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T16:39:30.6709356Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T16:39:39.5226852Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T16:39:41.6225102Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T16:39:45.8841101Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T16:39:49.8925522Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T16:39:59.2285127Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T17:01:53.2648075Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T17:01:55.0783135Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T17:01:57.0403601Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T17:01:58.3994402Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T17:02:09.4089842Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T17:02:11.7019684Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T17:02:17.0959627Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T17:02:22.5035302Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T17:02:33.9646304Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T17:27:40.1460724Z .................................................................................................... 1700/9891
2020-04-13T17:27:44.4251100Z .................................................................................................... 1800/9891
2020-04-13T17:27:52.9072728Z .................................................................................................... 1900/9891
2020-04-13T17:28:01.0915921Z ....i............................................................................................... 2000/9891
2020-04-13T17:28:07.3608718Z ..............................................................................................iiiii. 2100/9891
2020-04-13T17:28:28.3449158Z .................................................................................................... 2300/9891
2020-04-13T17:28:30.5340268Z .................................................................................................... 2400/9891
2020-04-13T17:28:32.7949015Z .................................................................................................... 2500/9891
2020-04-13T17:28:38.4374680Z .................................................................................................... 2600/9891
---
2020-04-13T17:31:37.2893772Z .................................................................................................... 5100/9891
2020-04-13T17:31:44.8349826Z .................................................................................................... 5200/9891
2020-04-13T17:31:49.8699075Z ..............i..................................................................................... 5300/9891
2020-04-13T17:31:59.5334375Z .................................................................................................... 5400/9891
2020-04-13T17:32:04.6644713Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-13T17:32:12.3227528Z ..................................................i................................................. 5700/9891
2020-04-13T17:32:22.2975688Z ......................................................................ii............................ 5800/9891
2020-04-13T17:32:28.7804584Z .........i.......................................................................................... 5900/9891
2020-04-13T17:32:34.3232307Z .................................................................................................... 6000/9891
2020-04-13T17:32:34.3232307Z .................................................................................................... 6000/9891
2020-04-13T17:32:44.6853218Z .................................................................................................... 6100/9891
2020-04-13T17:32:55.6376672Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-13T17:33:11.0027670Z .................................................................................................... 6400/9891
2020-04-13T17:33:15.0345826Z .................................................................................................... 6500/9891
2020-04-13T17:33:15.0345826Z .................................................................................................... 6500/9891
2020-04-13T17:33:27.2100426Z .................................i..ii.............................................................. 6600/9891
2020-04-13T17:33:49.0697661Z .................................................................................................... 6800/9891
2020-04-13T17:33:51.0590137Z .................................i.................................................................. 6900/9891
2020-04-13T17:33:53.1435272Z .................................................................................................... 7000/9891
2020-04-13T17:33:55.3401362Z ........................................................................i........................... 7100/9891
---
2020-04-13T17:35:33.2995418Z .................................................................................................... 7800/9891
2020-04-13T17:35:37.4892094Z .................................................................................................... 7900/9891
2020-04-13T17:35:44.2764247Z .................................................................................................... 8000/9891
2020-04-13T17:35:51.0127696Z ......................................i............................................................. 8100/9891
2020-04-13T17:36:00.6618189Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-13T17:36:16.8382253Z ................................i......i............................................................ 8400/9891
2020-04-13T17:36:20.4631208Z .................................................................................................... 8500/9891
2020-04-13T17:36:31.1739642Z .................................................................................................... 8600/9891
2020-04-13T17:36:44.6647955Z .................................................................................................... 8700/9891
---
2020-04-13T17:39:05.6178990Z Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
2020-04-13T17:39:05.6376454Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T17:39:05.8219787Z 
2020-04-13T17:39:05.8220105Z running 89 tests
2020-04-13T17:39:13.4370290Z .......FF....................F..F........FFF.....F....................FFF..FF............
2020-04-13T17:39:13.4372999Z 
2020-04-13T17:39:13.4393837Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-04-13T17:39:13.4394283Z 24 }
2020-04-13T17:39:13.4394530Z 25 
2020-04-13T17:39:13.4394530Z 25 
2020-04-13T17:39:13.4394861Z 26 alloc0 (static: FOO, size: 16, align: 8) {
2020-04-13T17:39:13.4395574Z -     ╾──────alloc14+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4396262Z +     ╾──────alloc17+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4397386Z 29 
2020-04-13T17:39:13.4397386Z 29 
2020-04-13T17:39:13.4397899Z - alloc14 (size: 72, align: 8) {
2020-04-13T17:39:13.4398272Z + alloc17 (size: 72, align: 8) {
2020-04-13T17:39:13.4398926Z 31     0x00 │ 00 00 00 00 __ __ __ __ ╾──────alloc4+0───────╼ │ ....░░░░╾──────╼
2020-04-13T17:39:13.4399616Z 32     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 __ __ __ __ │ ............░░░░
2020-04-13T17:39:13.4402183Z -     0x20 │ ╾──────alloc7+0───────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4405207Z -     0x30 │ 01 00 00 00 2a 00 00 00 ╾──────alloc11+0──────╼ │ ....*...╾──────╼
2020-04-13T17:39:13.4408289Z +     0x20 │ ╾──────alloc8+0───────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4409145Z +     0x30 │ 01 00 00 00 2a 00 00 00 ╾──────alloc13+0──────╼ │ ....*...╾──────╼
2020-04-13T17:39:13.4410272Z 36 }
2020-04-13T17:39:13.4410530Z 37 
2020-04-13T17:39:13.4410732Z 
2020-04-13T17:39:13.4410732Z 
2020-04-13T17:39:13.4411033Z 38 alloc4 (size: 0, align: 8) {}
2020-04-13T17:39:13.4411429Z 39 
2020-04-13T17:39:13.4414444Z - alloc7 (size: 32, align: 8) {
2020-04-13T17:39:13.4415550Z -     0x00 │ ╾──────alloc6+0───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4416279Z -     0x10 │ ╾──────alloc8+0───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4416685Z + alloc8 (size: 32, align: 8) {
2020-04-13T17:39:13.4417341Z +     0x00 │ ╾──────alloc7+0───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4421697Z +     0x10 │ ╾──────alloc9+0───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4422073Z 44 
2020-04-13T17:39:13.4422073Z 44 
2020-04-13T17:39:13.4422413Z - alloc6 (size: 3, align: 1) {
2020-04-13T17:39:13.4422655Z + alloc7 (size: 3, align: 1) {
2020-04-13T17:39:13.4424417Z 47 }
2020-04-13T17:39:13.4424589Z 48 
2020-04-13T17:39:13.4424690Z 
2020-04-13T17:39:13.4424690Z 
2020-04-13T17:39:13.4427215Z - alloc8 (size: 3, align: 1) {
2020-04-13T17:39:13.4427453Z + alloc9 (size: 3, align: 1) {
2020-04-13T17:39:13.4428189Z 51 }
2020-04-13T17:39:13.4428308Z 52 
2020-04-13T17:39:13.4428405Z 
2020-04-13T17:39:13.4428405Z 
2020-04-13T17:39:13.4428760Z - alloc11 (size: 48, align: 8) {
2020-04-13T17:39:13.4429287Z -     0x00 │ ╾──────alloc10+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4429877Z -     0x10 │ ╾──────alloc12+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4430465Z -     0x20 │ ╾──────alloc13+0──────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4430752Z + alloc13 (size: 48, align: 8) {
2020-04-13T17:39:13.4431265Z +     0x00 │ ╾──────alloc12+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4431849Z +     0x10 │ ╾──────alloc14+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4432430Z +     0x20 │ ╾──────alloc15+0──────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4433223Z 58 
2020-04-13T17:39:13.4433223Z 58 
2020-04-13T17:39:13.4433584Z - alloc10 (size: 3, align: 1) {
2020-04-13T17:39:13.4433814Z + alloc12 (size: 3, align: 1) {
2020-04-13T17:39:13.4434544Z 60     6d 65 68                                        │ meh
2020-04-13T17:39:13.4434871Z 62 
2020-04-13T17:39:13.4434982Z 
2020-04-13T17:39:13.4434982Z 
2020-04-13T17:39:13.4435314Z - alloc12 (size: 3, align: 1) {
2020-04-13T17:39:13.4435558Z + alloc14 (size: 3, align: 1) {
2020-04-13T17:39:13.4435999Z 64     6d 6f 70                                        │ mop
2020-04-13T17:39:13.4436338Z 66 
2020-04-13T17:39:13.4436433Z 
2020-04-13T17:39:13.4436433Z 
2020-04-13T17:39:13.4436777Z - alloc13 (size: 4, align: 1) {
2020-04-13T17:39:13.4437005Z + alloc15 (size: 4, align: 1) {
2020-04-13T17:39:13.4437449Z 68     6d c3 b6 70                                     │ m..p
2020-04-13T17:39:13.4437964Z 70 
2020-04-13T17:39:13.4438057Z 
2020-04-13T17:39:13.4438057Z 
2020-04-13T17:39:13.4438884Z thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4439957Z 
2020-04-13T17:39:13.4440347Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-13T17:39:13.4440644Z 24 }
2020-04-13T17:39:13.4440770Z 25 
2020-04-13T17:39:13.4440770Z 25 
2020-04-13T17:39:13.4441198Z 26 alloc0 (static: FOO, size: 16, align: 8) {
2020-04-13T17:39:13.4441776Z -     ╾──────alloc21+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4442322Z +     ╾──────alloc24+0──────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4442882Z 29 
2020-04-13T17:39:13.4442882Z 29 
2020-04-13T17:39:13.4443470Z - alloc21 (size: 72, align: 8) {
2020-04-13T17:39:13.4443722Z + alloc24 (size: 72, align: 8) {
2020-04-13T17:39:13.4444272Z 31     0x00 │ 00 00 00 00 __ __ __ __ ╾──────alloc9+0───────╼ │ ....░░░░╾──────╼
2020-04-13T17:39:13.4444832Z 32     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 __ __ __ __ │ ............░░░░
2020-04-13T17:39:13.4445402Z -     0x20 │ ╾──────alloc13+0──────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4445982Z -     0x30 │ 01 00 00 00 2a 00 00 00 ╾──────alloc20+0──────╼ │ ....*...╾──────╼
2020-04-13T17:39:13.4446567Z +     0x20 │ ╾──────alloc14+0──────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
2020-04-13T17:39:13.4447134Z +     0x30 │ 01 00 00 00 2a 00 00 00 ╾──────alloc22+0──────╼ │ ....*...╾──────╼
2020-04-13T17:39:13.4447898Z 36 }
2020-04-13T17:39:13.4448016Z 37 
2020-04-13T17:39:13.4448111Z 
2020-04-13T17:39:13.4448111Z 
2020-04-13T17:39:13.4448296Z 38 alloc9 (size: 0, align: 8) {}
2020-04-13T17:39:13.4448481Z 39 
2020-04-13T17:39:13.4448819Z - alloc13 (size: 16, align: 8) {
2020-04-13T17:39:13.4449349Z -     ╾──────alloc11+0──────╼ ╾──────alloc12+0──────╼ │ ╾──────╼╾──────╼
2020-04-13T17:39:13.4449626Z + alloc14 (size: 16, align: 8) {
2020-04-13T17:39:13.4450135Z +     ╾──────alloc12+0──────╼ ╾──────alloc13+0──────╼ │ ╾──────╼╾──────╼
2020-04-13T17:39:13.4450491Z 43 
2020-04-13T17:39:13.4450491Z 43 
2020-04-13T17:39:13.4450823Z - alloc11 (size: 1, align: 1) {
2020-04-13T17:39:13.4451066Z + alloc12 (size: 1, align: 1) {
2020-04-13T17:39:13.4451719Z 46 }
2020-04-13T17:39:13.4451838Z 47 
2020-04-13T17:39:13.4451948Z 
2020-04-13T17:39:13.4451948Z 
2020-04-13T17:39:13.4452275Z - alloc12 (size: 1, align: 1) {
2020-04-13T17:39:13.4452504Z + alloc13 (size: 1, align: 1) {
2020-04-13T17:39:13.4453150Z 50 }
2020-04-13T17:39:13.4453268Z 51 
2020-04-13T17:39:13.4453363Z 
2020-04-13T17:39:13.4453363Z 
2020-04-13T17:39:13.4453815Z - alloc20 (size: 24, align: 8) {
2020-04-13T17:39:13.4454348Z -     0x00 │ ╾──────alloc16+3──────╼ ╾──────alloc17+0──────╼ │ ╾──────╼╾──────╼
2020-04-13T17:39:13.4454893Z -     0x10 │ ╾──────alloc19+2──────╼                         │ ╾──────╼
2020-04-13T17:39:13.4455181Z + alloc22 (size: 24, align: 8) {
2020-04-13T17:39:13.4455748Z +     0x00 │ ╾──────alloc18+3──────╼ ╾──────alloc19+0──────╼ │ ╾──────╼╾──────╼
2020-04-13T17:39:13.4456296Z +     0x10 │ ╾──────alloc21+2──────╼                         │ ╾──────╼
2020-04-13T17:39:13.4456716Z 56 
2020-04-13T17:39:13.4456716Z 56 
2020-04-13T17:39:13.4457063Z - alloc16 (size: 4, align: 1) {
2020-04-13T17:39:13.4457307Z + alloc18 (size: 4, align: 1) {
2020-04-13T17:39:13.4457750Z 58     2a 45 15 6f                                     │ *E.o
2020-04-13T17:39:13.4458088Z 60 
2020-04-13T17:39:13.4458184Z 
2020-04-13T17:39:13.4458184Z 
2020-04-13T17:39:13.4458510Z - alloc17 (size: 1, align: 1) {
2020-04-13T17:39:13.4458736Z + alloc19 (size: 1, align: 1) {
2020-04-13T17:39:13.4459396Z 63 }
2020-04-13T17:39:13.4459515Z 64 
2020-04-13T17:39:13.4460404Z 
2020-04-13T17:39:13.4460404Z 
2020-04-13T17:39:13.4460781Z - alloc19 (size: 4, align: 1) {
2020-04-13T17:39:13.4461009Z + alloc21 (size: 4, align: 1) {
2020-04-13T17:39:13.4461452Z 66     2a 45 15 6f                                     │ *E.o
2020-04-13T17:39:13.4461790Z 68 
2020-04-13T17:39:13.4461884Z 
2020-04-13T17:39:13.4461884Z 
2020-04-13T17:39:13.4463012Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4463873Z ---- [mir-opt] mir-opt/copy_propagation.rs stdout ----
2020-04-13T17:39:13.4463873Z ---- [mir-opt] mir-opt/copy_propagation.rs stdout ----
2020-04-13T17:39:13.4464796Z thread '[mir-opt] mir-opt/copy_propagation.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/runtest.rs:3104:34
2020-04-13T17:39:13.4465666Z ---- [mir-opt] mir-opt/copy_propagation_arg.rs stdout ----
2020-04-13T17:39:13.4465666Z ---- [mir-opt] mir-opt/copy_propagation_arg.rs stdout ----
2020-04-13T17:39:13.4466599Z thread '[mir-opt] mir-opt/copy_propagation_arg.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/runtest.rs:3104:34
2020-04-13T17:39:13.4467942Z ---- [mir-opt] mir-opt/inline/inline-any-operand.rs stdout ----
2020-04-13T17:39:13.4467942Z ---- [mir-opt] mir-opt/inline/inline-any-operand.rs stdout ----
2020-04-13T17:39:13.4468590Z 4     let mut _0: bool;                    // return place in scope 0 at $DIR/inline-any-operand.rs:10:13: 10:17
2020-04-13T17:39:13.4469282Z 5     let _1: fn(i32, i32) -> bool {foo};  // in scope 0 at $DIR/inline-any-operand.rs:11:9: 11:10
2020-04-13T17:39:13.4469968Z 6     let mut _2: fn(i32, i32) -> bool {foo}; // in scope 0 at $DIR/inline-any-operand.rs:12:5: 12:6
2020-04-13T17:39:13.4470643Z -     let mut _5: i32;                     // in scope 0 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4471306Z -     let mut _6: i32;                     // in scope 0 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4471984Z +     let mut _3: i32;                     // in scope 0 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4472646Z +     let mut _4: i32;                     // in scope 0 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4473479Z 10         debug f => _1;                   // in scope 1 at $DIR/inline-any-operand.rs:11:9: 11:10
2020-04-13T17:39:13.4473777Z 11         scope 2 {
2020-04-13T17:39:13.4473897Z 
2020-04-13T17:39:13.4474394Z -             debug x => _5;               // in scope 2 at $DIR/inline-any-operand.rs:16:8: 16:9
2020-04-13T17:39:13.4474394Z -             debug x => _5;               // in scope 2 at $DIR/inline-any-operand.rs:16:8: 16:9
2020-04-13T17:39:13.4494044Z -             debug y => _6;               // in scope 2 at $DIR/inline-any-operand.rs:16:16: 16:17
2020-04-13T17:39:13.4495157Z -             let mut _3: i32;             // in scope 2 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4495850Z -             let mut _4: i32;             // in scope 2 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4496533Z +             debug x => _3;               // in scope 2 at $DIR/inline-any-operand.rs:16:8: 16:9
2020-04-13T17:39:13.4497187Z +             debug y => _4;               // in scope 2 at $DIR/inline-any-operand.rs:16:16: 16:17
2020-04-13T17:39:13.4497637Z 17     }
2020-04-13T17:39:13.4497840Z 18 
2020-04-13T17:39:13.4497944Z 
2020-04-13T17:39:13.4497944Z 
2020-04-13T17:39:13.4498587Z 27                                          // + literal: Const { ty: fn(i32, i32) -> bool {foo}, val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4499425Z 28         StorageLive(_2);                 // bb0[2]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:6
2020-04-13T17:39:13.4500183Z 29         _2 = _1;                         // bb0[3]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:6
2020-04-13T17:39:13.4501165Z -         _5 = const 1i32;                 // bb0[4]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4501923Z +         _3 = const 1i32;                 // bb0[4]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4502334Z 31                                          // ty::Const
2020-04-13T17:39:13.4502634Z 32                                          // + ty: i32
2020-04-13T17:39:13.4502971Z 33                                          // + val: Value(Scalar(0x00000001))
2020-04-13T17:39:13.4503414Z 34                                          // mir::Constant
2020-04-13T17:39:13.4503414Z 34                                          // mir::Constant
2020-04-13T17:39:13.4504608Z 35                                          // + span: $DIR/inline-any-operand.rs:12:7: 12:8
2020-04-13T17:39:13.4505259Z 36                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
2020-04-13T17:39:13.4507348Z -         _6 = const -1i32;                // bb0[5]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4512874Z +         _4 = const -1i32;                // bb0[5]: scope 1 at $DIR/inline-any-operand.rs:12:5: 12:13
2020-04-13T17:39:13.4513297Z 38                                          // ty::Const
2020-04-13T17:39:13.4513611Z 39                                          // + ty: i32
2020-04-13T17:39:13.4513947Z 40                                          // + val: Value(Scalar(0xffffffff))
2020-04-13T17:39:13.4514399Z 41                                          // mir::Constant
2020-04-13T17:39:13.4514399Z 41                                          // mir::Constant
2020-04-13T17:39:13.4515055Z 42                                          // + span: $DIR/inline-any-operand.rs:12:10: 12:12
2020-04-13T17:39:13.4515565Z 43                                          // + literal: Const { ty: i32, val: Value(Scalar(0xffffffff)) }
2020-04-13T17:39:13.4516346Z -         StorageLive(_3);                 // bb0[6]: scope 2 at $DIR/inline-any-operand.rs:17:5: 17:6
2020-04-13T17:39:13.4517342Z -         _3 = _5;                         // bb0[7]: scope 2 at $DIR/inline-any-operand.rs:17:5: 17:6
2020-04-13T17:39:13.4518119Z -         StorageLive(_4);                 // bb0[8]: scope 2 at $DIR/inline-any-operand.rs:17:10: 17:11
2020-04-13T17:39:13.4519003Z -         _4 = _6;                         // bb0[9]: scope 2 at $DIR/inline-any-operand.rs:17:10: 17:11
2020-04-13T17:39:13.4519768Z -         _0 = Eq(move _3, move _4);       // bb0[10]: scope 2 at $DIR/inline-any-operand.rs:17:5: 17:11
2020-04-13T17:39:13.4520619Z -         StorageDead(_4);                 // bb0[11]: scope 2 at $DIR/inline-any-operand.rs:17:10: 17:11
2020-04-13T17:39:13.4521409Z -         StorageDead(_3);                 // bb0[12]: scope 2 at $DIR/inline-any-operand.rs:17:10: 17:11
2020-04-13T17:39:13.4522170Z -         StorageDead(_2);                 // bb0[13]: scope 1 at $DIR/inline-any-operand.rs:12:12: 12:13
2020-04-13T17:39:13.4522924Z -         StorageDead(_1);                 // bb0[14]: scope 0 at $DIR/inline-any-operand.rs:13:1: 13:2
2020-04-13T17:39:13.4524140Z -         return;                          // bb0[15]: scope 0 at $DIR/inline-any-operand.rs:13:2: 13:2
2020-04-13T17:39:13.4524903Z +         _0 = Eq(move _3, move _4);       // bb0[6]: scope 2 at $DIR/inline-any-operand.rs:17:5: 17:11
2020-04-13T17:39:13.4525672Z +         StorageDead(_2);                 // bb0[7]: scope 1 at $DIR/inline-any-operand.rs:12:12: 12:13
2020-04-13T17:39:13.4526532Z +         StorageDead(_1);                 // bb0[8]: scope 0 at $DIR/inline-any-operand.rs:13:1: 13:2
2020-04-13T17:39:13.4527366Z +         return;                          // bb0[9]: scope 0 at $DIR/inline-any-operand.rs:13:2: 13:2
2020-04-13T17:39:13.4527845Z 55 }
2020-04-13T17:39:13.4527965Z 56 
2020-04-13T17:39:13.4528062Z 
2020-04-13T17:39:13.4528062Z 
2020-04-13T17:39:13.4529036Z thread '[mir-opt] mir-opt/inline/inline-any-operand.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-any-operand/rustc.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4529936Z ---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
2020-04-13T17:39:13.4529936Z ---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
2020-04-13T17:39:13.4530586Z 9     let mut _5: (&i32, &i32);            // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4532052Z 10     let mut _6: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:7: 16:8
2020-04-13T17:39:13.4532790Z 11     let mut _7: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:10: 16:11
2020-04-13T17:39:13.4533501Z +     let mut _8: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4534205Z 12     let mut _9: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4534926Z -     let mut _10: &i32;                   // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4535784Z 15         debug x => _3;                   // in scope 1 at $DIR/inline-closure-borrows-arg.rs:12:9: 12:10
2020-04-13T17:39:13.4536112Z 16         scope 2 {
2020-04-13T17:39:13.4536234Z 
2020-04-13T17:39:13.4536750Z -             debug r => _9;               // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:14: 12:15
2020-04-13T17:39:13.4536750Z -             debug r => _9;               // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:14: 12:15
2020-04-13T17:39:13.4537451Z -             debug _s => _10;             // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:23: 12:25
2020-04-13T17:39:13.4538150Z -             let _8: &i32;                // in scope 2 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4538845Z +             debug r => _8;               // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:14: 12:15
2020-04-13T17:39:13.4539549Z +             debug _s => _9;              // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:23: 12:25
2020-04-13T17:39:13.4540170Z 21     }
2020-04-13T17:39:13.4540336Z 22     scope 3 {
2020-04-13T17:39:13.4540451Z 
2020-04-13T17:39:13.4540451Z 
2020-04-13T17:39:13.4541055Z 42         StorageLive(_7);                 // bb0[7]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:10: 16:11
2020-04-13T17:39:13.4541884Z 43         _7 = &(*_2);                     // bb0[8]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:10: 16:11
2020-04-13T17:39:13.4542696Z 44         _5 = (move _6, move _7);         // bb0[9]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4543522Z -         _9 = move (_5.0: &i32);          // bb0[10]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4544369Z -         _10 = move (_5.1: &i32);         // bb0[11]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4545754Z -         StorageLive(_8);                 // bb0[12]: scope 2 at $DIR/inline-closure-borrows-arg.rs:13:13: 13:21
2020-04-13T17:39:13.4546587Z -         _8 = _9;                         // bb0[13]: scope 2 at $DIR/inline-closure-borrows-arg.rs:13:24: 13:27
2020-04-13T17:39:13.4547560Z -         _0 = (*_8);                      // bb0[14]: scope 3 at $DIR/inline-closure-borrows-arg.rs:14:9: 14:18
2020-04-13T17:39:13.4548367Z -         StorageDead(_8);                 // bb0[15]: scope 2 at $DIR/inline-closure-borrows-arg.rs:15:5: 15:6
2020-04-13T17:39:13.4549432Z -         StorageDead(_7);                 // bb0[16]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4550624Z -         StorageDead(_6);                 // bb0[17]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4551740Z -         StorageDead(_5);                 // bb0[18]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4552613Z -         StorageDead(_4);                 // bb0[19]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4553460Z -         StorageDead(_3);                 // bb0[20]: scope 0 at $DIR/inline-closure-borrows-arg.rs:17:1: 17:2
2020-04-13T17:39:13.4554265Z -         return;                          // bb0[21]: scope 0 at $DIR/inline-closure-borrows-arg.rs:17:2: 17:2
2020-04-13T17:39:13.4555101Z +         _8 = move (_5.0: &i32);          // bb0[10]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4555935Z +         _9 = move (_5.1: &i32);          // bb0[11]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
2020-04-13T17:39:13.4556917Z +         _0 = (*_8);                      // bb0[12]: scope 3 at $DIR/inline-closure-borrows-arg.rs:14:9: 14:18
2020-04-13T17:39:13.4557749Z +         StorageDead(_7);                 // bb0[13]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4558561Z +         StorageDead(_6);                 // bb0[14]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4559505Z +         StorageDead(_5);                 // bb0[15]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4560332Z +         StorageDead(_4);                 // bb0[16]: scope 1 at $DIR/inline-closure-borrows-arg.rs:16:11: 16:12
2020-04-13T17:39:13.4561134Z +         StorageDead(_3);                 // bb0[17]: scope 0 at $DIR/inline-closure-borrows-arg.rs:17:1: 17:2
2020-04-13T17:39:13.4561946Z +         return;                          // bb0[18]: scope 0 at $DIR/inline-closure-borrows-arg.rs:17:2: 17:2
2020-04-13T17:39:13.4562619Z 58 }
2020-04-13T17:39:13.4562754Z 59 
2020-04-13T17:39:13.4562851Z 
2020-04-13T17:39:13.4562851Z 
2020-04-13T17:39:13.4564046Z thread '[mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-closure-borrows-arg/rustc.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4564964Z ---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
2020-04-13T17:39:13.4564964Z ---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
2020-04-13T17:39:13.4565651Z 10     let mut _6: &[closure@foo<T>::{{closure}}#0 q:&i32, t:&T]; // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:6
2020-04-13T17:39:13.4566412Z 11     let mut _7: (i32,);                  // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4567101Z 12     let mut _8: i32;                     // in scope 0 at $DIR/inline-closure-captures.rs:12:7: 12:8
2020-04-13T17:39:13.4567787Z -     let mut _10: i32;                    // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4568488Z +     let mut _11: i32;                    // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4569442Z 15         debug x => _3;                   // in scope 1 at $DIR/inline-closure-captures.rs:11:9: 11:10
2020-04-13T17:39:13.4569772Z 16         scope 2 {
2020-04-13T17:39:13.4569893Z 
2020-04-13T17:39:13.4569893Z 
2020-04-13T17:39:13.4570408Z -             debug _q => _10;             // in scope 2 at $DIR/inline-closure-captures.rs:11:14: 11:16
2020-04-13T17:39:13.4571197Z +             debug _q => _11;             // in scope 2 at $DIR/inline-closure-captures.rs:11:14: 11:16
2020-04-13T17:39:13.4571904Z 18             debug q => (*((*_6).0: &i32)); // in scope 2 at $DIR/inline-closure-captures.rs:10:23: 10:24
2020-04-13T17:39:13.4572629Z 19             debug t => (*((*_6).1: &T)); // in scope 2 at $DIR/inline-closure-captures.rs:10:17: 10:18
2020-04-13T17:39:13.4573340Z -             let mut _9: T;               // in scope 2 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4574227Z +             let mut _9: i32;             // in scope 2 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4574947Z +             let mut _10: T;              // in scope 2 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4575412Z 22     }
2020-04-13T17:39:13.4575538Z 23 
2020-04-13T17:39:13.4575633Z 
2020-04-13T17:39:13.4575633Z 
2020-04-13T17:39:13.4576221Z 44         StorageLive(_8);                 // bb0[11]: scope 1 at $DIR/inline-closure-captures.rs:12:7: 12:8
2020-04-13T17:39:13.4577013Z 45         _8 = _2;                         // bb0[12]: scope 1 at $DIR/inline-closure-captures.rs:12:7: 12:8
2020-04-13T17:39:13.4577935Z 46         _7 = (move _8,);                 // bb0[13]: scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4578752Z -         _10 = move (_7.0: i32);          // bb0[14]: scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4579587Z -         (_0.0: i32) = (*((*_6).0: &i32)); // bb0[15]: scope 2 at $DIR/inline-closure-captures.rs:11:19: 11:20
2020-04-13T17:39:13.4580418Z -         StorageLive(_9);                 // bb0[16]: scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
2020-04-13T17:39:13.4581223Z -         _9 = (*((*_6).1: &T));           // bb0[17]: scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
2020-04-13T17:39:13.4582035Z -         (_0.1: T) = move _9;             // bb0[18]: scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
2020-04-13T17:39:13.4582981Z -         StorageDead(_9);                 // bb0[19]: scope 2 at $DIR/inline-closure-captures.rs:11:23: 11:24
2020-04-13T17:39:13.4583779Z -         StorageDead(_8);                 // bb0[20]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4584575Z -         StorageDead(_7);                 // bb0[21]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4585355Z -         StorageDead(_6);                 // bb0[22]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4586139Z -         StorageDead(_3);                 // bb0[23]: scope 0 at $DIR/inline-closure-captures.rs:13:1: 13:2
2020-04-13T17:39:13.4586931Z -         return;                          // bb0[24]: scope 0 at $DIR/inline-closure-captures.rs:13:2: 13:2
2020-04-13T17:39:13.4587727Z +         _11 = move (_7.0: i32);          // bb0[14]: scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
2020-04-13T17:39:13.4588528Z +         StorageLive(_9);                 // bb0[15]: scope 2 at $DIR/inline-closure-captures.rs:11:19: 11:20
2020-04-13T17:39:13.4589354Z +         _9 = (*((*_6).0: &i32));         // bb0[16]: scope 2 at $DIR/inline-closure-captures.rs:11:19: 11:20
2020-04-13T17:39:13.4590154Z +         StorageLive(_10);                // bb0[17]: scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
2020-04-13T17:39:13.4591086Z +         _10 = (*((*_6).1: &T));          // bb0[18]: scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
2020-04-13T17:39:13.4591925Z +         (_0.0: i32) = move _9;           // bb0[19]: scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
2020-04-13T17:39:13.4592735Z +         (_0.1: T) = move _10;            // bb0[20]: scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
2020-04-13T17:39:13.4593548Z +         StorageDead(_10);                // bb0[21]: scope 2 at $DIR/inline-closure-captures.rs:11:23: 11:24
2020-04-13T17:39:13.4594342Z +         StorageDead(_9);                 // bb0[22]: scope 2 at $DIR/inline-closure-captures.rs:11:23: 11:24
2020-04-13T17:39:13.4595564Z +         StorageDead(_8);                 // bb0[23]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4596915Z +         StorageDead(_7);                 // bb0[24]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4597700Z +         StorageDead(_6);                 // bb0[25]: scope 1 at $DIR/inline-closure-captures.rs:12:8: 12:9
2020-04-13T17:39:13.4598578Z +         StorageDead(_3);                 // bb0[26]: scope 0 at $DIR/inline-closure-captures.rs:13:1: 13:2
2020-04-13T17:39:13.4599386Z +         return;                          // bb0[27]: scope 0 at $DIR/inline-closure-captures.rs:13:2: 13:2
2020-04-13T17:39:13.4599855Z 59 }
2020-04-13T17:39:13.4599989Z 60 
2020-04-13T17:39:13.4600088Z 
2020-04-13T17:39:13.4600088Z 
2020-04-13T17:39:13.4600940Z thread '[mir-opt] mir-opt/inline/inline-closure-captures.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-closure-captures/rustc.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4602110Z ---- [mir-opt] mir-opt/inline/inline-trait-method_2.rs stdout ----
2020-04-13T17:39:13.4602110Z ---- [mir-opt] mir-opt/inline/inline-trait-method_2.rs stdout ----
2020-04-13T17:39:13.4602740Z 7     let mut _3: &dyn X;                  // in scope 0 at $DIR/inline-trait-method_2.rs:5:10: 5:11
2020-04-13T17:39:13.4603786Z 9         debug x => _2;                   // in scope 1 at $DIR/inline-trait-method_2.rs:9:9: 9:10
2020-04-13T17:39:13.4603786Z 9         debug x => _2;                   // in scope 1 at $DIR/inline-trait-method_2.rs:9:9: 9:10
2020-04-13T17:39:13.4604454Z -         let mut _4: &dyn X;              // in scope 1 at $DIR/inline-trait-method_2.rs:5:5: 5:12
2020-04-13T17:39:13.4604885Z 12 
2020-04-13T17:39:13.4605026Z 13     bb0: {
2020-04-13T17:39:13.4605142Z 
2020-04-13T17:39:13.4605142Z 
2020-04-13T17:39:13.4605717Z 16         _3 = &(*_1);                     // bb0[2]: scope 0 at $DIR/inline-trait-method_2.rs:5:10: 5:11
2020-04-13T17:39:13.4606507Z 17         _2 = move _3 as &dyn X (Pointer(Unsize)); // bb0[3]: scope 0 at $DIR/inline-trait-method_2.rs:5:10: 5:11
2020-04-13T17:39:13.4607292Z 18         StorageDead(_3);                 // bb0[4]: scope 0 at $DIR/inline-trait-method_2.rs:5:10: 5:11
2020-04-13T17:39:13.4608075Z -         StorageLive(_4);                 // bb0[5]: scope 1 at $DIR/inline-trait-method_2.rs:10:5: 10:6
2020-04-13T17:39:13.4608840Z -         _4 = _2;                         // bb0[6]: scope 1 at $DIR/inline-trait-method_2.rs:10:5: 10:6
2020-04-13T17:39:13.4609835Z -         _0 = const <dyn X as X>::y(move _4) -> bb1; // bb0[7]: scope 1 at $DIR/inline-trait-method_2.rs:10:5: 10:10
2020-04-13T17:39:13.4610701Z +         _0 = const <dyn X as X>::y(move _2) -> bb1; // bb0[5]: scope 1 at $DIR/inline-trait-method_2.rs:10:5: 10:10
2020-04-13T17:39:13.4611151Z 22                                          // ty::Const
2020-04-13T17:39:13.4611795Z 23                                          // + ty: for<'r> fn(&'r dyn X) -> bool {<dyn X as X>::y}
2020-04-13T17:39:13.4612252Z 24                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4612594Z 28     }
2020-04-13T17:39:13.4612734Z 29 
2020-04-13T17:39:13.4612874Z 30     bb1: {
2020-04-13T17:39:13.4612874Z 30     bb1: {
2020-04-13T17:39:13.4613462Z -         StorageDead(_4);                 // bb1[0]: scope 1 at $DIR/inline-trait-method_2.rs:10:9: 10:10
2020-04-13T17:39:13.4614247Z -         StorageDead(_2);                 // bb1[1]: scope 0 at $DIR/inline-trait-method_2.rs:5:11: 5:12
2020-04-13T17:39:13.4615004Z -         return;                          // bb1[2]: scope 0 at $DIR/inline-trait-method_2.rs:6:2: 6:2
2020-04-13T17:39:13.4615776Z +         StorageDead(_2);                 // bb1[0]: scope 0 at $DIR/inline-trait-method_2.rs:5:11: 5:12
2020-04-13T17:39:13.4617012Z +         return;                          // bb1[1]: scope 0 at $DIR/inline-trait-method_2.rs:6:2: 6:2
2020-04-13T17:39:13.4617489Z 35 }
2020-04-13T17:39:13.4617609Z 36 
2020-04-13T17:39:13.4617829Z 
2020-04-13T17:39:13.4617829Z 
2020-04-13T17:39:13.4618693Z thread '[mir-opt] mir-opt/inline/inline-trait-method_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-trait-method_2/rustc.test2.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4619556Z ---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
2020-04-13T17:39:13.4619787Z 13           scope 2 {
2020-04-13T17:39:13.4619966Z 14           }
2020-04-13T17:39:13.4620132Z 15           scope 3 {
---
2020-04-13T17:39:13.4622035Z 19   
2020-04-13T17:39:13.4622134Z 
2020-04-13T17:39:13.4622263Z 49       }
2020-04-13T17:39:13.4622393Z 50   
2020-04-13T17:39:13.4622553Z 51       bb3: {
2020-04-13T17:39:13.4623168Z -           _5 = ((_1 as Foo).0: u8);        // bb3[0]: scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:19
2020-04-13T17:39:13.4623985Z -           ((_2 as Foo).0: u8) = move _5;   // bb3[1]: scope 3 at $DIR/simplify-arm-identity.rs:19:24: 19:35
2020-04-13T17:39:13.4624817Z +           _4 = ((_1 as Foo).0: u8);        // bb3[0]: scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:19
2020-04-13T17:39:13.4625633Z +           ((_2 as Foo).0: u8) = move _4;   // bb3[1]: scope 3 at $DIR/simplify-arm-identity.rs:19:24: 19:35
2020-04-13T17:39:13.4626435Z 54           discriminant(_2) = 0;            // bb3[2]: scope 3 at $DIR/simplify-arm-identity.rs:19:24: 19:35
2020-04-13T17:39:13.4627231Z 55           goto -> bb4;                     // bb3[3]: scope 1 at $DIR/simplify-arm-identity.rs:18:18: 21:6
2020-04-13T17:39:13.4627681Z 
2020-04-13T17:39:13.4627681Z 
2020-04-13T17:39:13.4628532Z thread '[mir-opt] mir-opt/simplify-arm-identity.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify-arm-identity/rustc.main.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4629406Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs stdout ----
2020-04-13T17:39:13.4629406Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs stdout ----
2020-04-13T17:39:13.4630095Z 15 -     let mut _10: u8;                     // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4630873Z 16 -     let mut _11: Temp;                   // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4631640Z 17 +     let _1: ();                          // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4632422Z - +     let mut _2: ((), ());                // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4633180Z - +     let _3: ();                          // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4633961Z - +     let mut _4: u8;                      // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4634722Z + +     let _2: ();                          // in scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4635228Z 22       }
2020-04-13T17:39:13.4635362Z 23   
2020-04-13T17:39:13.4635460Z 
2020-04-13T17:39:13.4635460Z 
2020-04-13T17:39:13.4636089Z 26 -         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:21: 13:23
2020-04-13T17:39:13.4636993Z 27 -         _2 = const ();                   // bb0[2]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:21: 13:23
2020-04-13T17:39:13.4637874Z 28 +         StorageLive(_1);                 // bb0[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4638814Z - +         StorageLive(_2);                 // bb0[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4639744Z - +         _2 = const {transmute(()): ((), ())}; // bb0[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4640741Z + +         _1 = const use_zst(const {transmute(()): ((), ())}) -> bb1; // bb0[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4641285Z 31                                            // ty::Const
2020-04-13T17:39:13.4642123Z 32 -                                          // + ty: ()
2020-04-13T17:39:13.4642691Z 33 -                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4642927Z 
2020-04-13T17:39:13.4643671Z 44 -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4644538Z 45 -         _1 = const {transmute(()): ((), ())}; // bb0[5]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:20: 13:28
2020-04-13T17:39:13.4645233Z 46 -                                          // ty::Const
2020-04-13T17:39:13.4645721Z -                                            // + ty: ((), ())
2020-04-13T17:39:13.4646256Z -                                            // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4646791Z -                                            // mir::Constant
2020-04-13T17:39:13.4647285Z + -                                          // + ty: ((), ())
2020-04-13T17:39:13.4647822Z + -                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4648358Z + -                                          // mir::Constant
2020-04-13T17:39:13.4649945Z 50 -                                          // + span: $DIR/simplify-locals-removes-unused-consts.rs:13:20: 13:28
2020-04-13T17:39:13.4651001Z 51 -                                          // + literal: Const { ty: ((), ()), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4651873Z 52 -         StorageDead(_3);                 // bb0[6]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:27: 13:28
2020-04-13T17:39:13.4652228Z 
2020-04-13T17:39:13.4652850Z 53 -         StorageDead(_2);                 // bb0[7]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:27: 13:28
2020-04-13T17:39:13.4653750Z 54 -         StorageDead(_1);                 // bb0[8]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:13:28: 13:29
2020-04-13T17:39:13.4654637Z 55 -         StorageLive(_4);                 // bb0[9]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4655521Z - -         StorageLive(_5);                 // bb0[10]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4656422Z - -         StorageLive(_6);                 // bb0[11]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:14: 14:16
2020-04-13T17:39:13.4657306Z - -         _6 = const ();                   // bb0[12]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:14: 14:16
2020-04-13T17:39:13.4658207Z + -         StorageLive(_6);                 // bb0[10]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:14: 14:16
2020-04-13T17:39:13.4659094Z + -         _6 = const ();                   // bb0[11]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:14: 14:16
2020-04-13T17:39:13.4659740Z 59 -                                          // ty::Const
2020-04-13T17:39:13.4660228Z 60 -                                          // + ty: ()
2020-04-13T17:39:13.4660757Z 61 -                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4661391Z 62 -                                          // mir::Constant
2020-04-13T17:39:13.4661391Z 62 -                                          // mir::Constant
2020-04-13T17:39:13.4662615Z 63 -                                          // + span: $DIR/simplify-locals-removes-unused-consts.rs:14:14: 14:16
2020-04-13T17:39:13.4663395Z 64 -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4664366Z - -         StorageLive(_7);                 // bb0[13]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:18: 14:20
2020-04-13T17:39:13.4665252Z - -         _7 = const ();                   // bb0[14]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:18: 14:20
2020-04-13T17:39:13.4666136Z + -         StorageLive(_7);                 // bb0[12]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:18: 14:20
2020-04-13T17:39:13.4667096Z + -         _7 = const ();                   // bb0[13]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:18: 14:20
2020-04-13T17:39:13.4667758Z 67 -                                          // ty::Const
2020-04-13T17:39:13.4668229Z 68 -                                          // + ty: ()
2020-04-13T17:39:13.4668771Z 69 -                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4669667Z 70 -                                          // mir::Constant
2020-04-13T17:39:13.4669667Z 70 -                                          // mir::Constant
2020-04-13T17:39:13.4670396Z 71 -                                          // + span: $DIR/simplify-locals-removes-unused-consts.rs:14:18: 14:20
2020-04-13T17:39:13.4671166Z 72 -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4672026Z - -         _5 = const {transmute(()): ((), ())}; // bb0[15]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4672961Z + -         StorageDead(_7);                 // bb0[14]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:20: 14:21
2020-04-13T17:39:13.4674080Z + -         StorageDead(_6);                 // bb0[15]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:20: 14:21
2020-04-13T17:39:13.4675072Z + -         _4 = const use_zst(const {transmute(()): ((), ())}) -> bb1; // bb0[16]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4675791Z 74 -                                          // ty::Const
2020-04-13T17:39:13.4676279Z - -                                          // + ty: ((), ())
2020-04-13T17:39:13.4676828Z - -                                          // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4677351Z - -                                          // mir::Constant
2020-04-13T17:39:13.4678036Z -                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4678846Z -                                            // + literal: Const { ty: ((), ()), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4679686Z - -         StorageDead(_7);                 // bb0[16]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:20: 14:21
2020-04-13T17:39:13.4680570Z - -         StorageDead(_6);                 // bb0[17]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:20: 14:21
2020-04-13T17:39:13.4681475Z - -         _4 = const use_zst(move _5) -> bb1; // bb0[18]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4682371Z - +         _1 = const use_zst(move _2) -> bb1; // bb0[3]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:22
2020-04-13T17:39:13.4683029Z -                                            // ty::Const
2020-04-13T17:39:13.4687151Z 85                                            // + ty: fn(((), ())) {use_zst}
2020-04-13T17:39:13.4687565Z 86                                            // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4688141Z 
2020-04-13T17:39:13.4688141Z 
2020-04-13T17:39:13.4688890Z 88                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:14:5: 14:12
2020-04-13T17:39:13.4689513Z 89                                            // + literal: Const { ty: fn(((), ())) {use_zst}, val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4689971Z +                                            // ty::Const
2020-04-13T17:39:13.4690458Z +                                            // + ty: ((), ())
2020-04-13T17:39:13.4690813Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4691353Z +                                            // mir::Constant
2020-04-13T17:39:13.4692079Z +                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:14:13: 14:21
2020-04-13T17:39:13.4692731Z +                                            // + literal: Const { ty: ((), ()), val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4693206Z 91   
2020-04-13T17:39:13.4693370Z 92       bb1: {
2020-04-13T17:39:13.4693492Z 
2020-04-13T17:39:13.4693492Z 
2020-04-13T17:39:13.4694133Z - -         StorageDead(_5);                 // bb1[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:21: 14:22
2020-04-13T17:39:13.4695721Z - -         StorageDead(_4);                 // bb1[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:22: 14:23
2020-04-13T17:39:13.4696724Z - -         StorageLive(_8);                 // bb1[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4697607Z - -         StorageLive(_9);                 // bb1[3]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4698498Z - -         StorageLive(_10);                // bb1[4]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4699381Z - -         StorageLive(_11);                // bb1[5]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4700293Z - -         _11 = const {transmute(0x28): Temp}; // bb1[6]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4701215Z - +         StorageDead(_2);                 // bb1[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:21: 14:22
2020-04-13T17:39:13.4702092Z - +         StorageDead(_1);                 // bb1[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:22: 14:23
2020-04-13T17:39:13.4702984Z - +         StorageLive(_3);                 // bb1[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4703863Z - +         StorageLive(_4);                 // bb1[3]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4704742Z - +         _4 = const 42u8;                 // bb1[4]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4705638Z + -         StorageDead(_4);                 // bb1[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:22: 14:23
2020-04-13T17:39:13.4706509Z + -         StorageLive(_8);                 // bb1[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4707384Z + -         StorageLive(_10);                // bb1[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4708280Z + -         StorageLive(_11);                // bb1[3]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4709190Z + -         _11 = const {transmute(0x28): Temp}; // bb1[4]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4710108Z + +         StorageDead(_1);                 // bb1[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:14:22: 14:23
2020-04-13T17:39:13.4710987Z + +         StorageLive(_2);                 // bb1[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4711874Z + +         _2 = const use_u8(const 42u8) -> bb2; // bb1[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4712360Z 105                                            // ty::Const
2020-04-13T17:39:13.4712844Z 106 -                                          // + ty: Temp
2020-04-13T17:39:13.4713379Z 107 -                                          // + val: Value(Scalar(0x28))
2020-04-13T17:39:13.4714117Z 108 -                                          // mir::Constant
2020-04-13T17:39:13.4714117Z 108 -                                          // mir::Constant
2020-04-13T17:39:13.4714809Z 109 -                                          // + span: $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:28
2020-04-13T17:39:13.4715602Z 110 -                                          // + literal: Const { ty: Temp, val: Value(Scalar(0x28)) }
2020-04-13T17:39:13.4716482Z - -         _10 = const 40u8;                // bb1[7]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4717378Z + -         _10 = const 40u8;                // bb1[5]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4718042Z 112 -                                          // ty::Const
2020-04-13T17:39:13.4718513Z -                                            // + ty: u8
2020-04-13T17:39:13.4718974Z + -                                          // + ty: u8
2020-04-13T17:39:13.4719519Z 114 -                                          // + val: Value(Scalar(0x28))
2020-04-13T17:39:13.4720046Z 115 -                                          // mir::Constant
2020-04-13T17:39:13.4720733Z 116 -                                          // + span: $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:30
2020-04-13T17:39:13.4721079Z 
2020-04-13T17:39:13.4721616Z 117 -                                          // + literal: Const { ty: u8, val: Value(Scalar(0x28)) }
2020-04-13T17:39:13.4722437Z - -         _9 = const 42u8;                 // bb1[8]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4723458Z + -         StorageDead(_10);                // bb1[6]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:33: 16:34
2020-04-13T17:39:13.4724357Z + -         _8 = const use_u8(const 42u8) -> bb2; // bb1[7]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4725008Z 119 -                                          // ty::Const
2020-04-13T17:39:13.4725498Z - -                                          // + ty: u8
2020-04-13T17:39:13.4726016Z -                                            // + val: Value(Scalar(0x2a))
2020-04-13T17:39:13.4726549Z -                                            // mir::Constant
2020-04-13T17:39:13.4727231Z -                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4728002Z -                                            // + literal: Const { ty: u8, val: Value(Scalar(0x2a)) }
2020-04-13T17:39:13.4728834Z - -         StorageDead(_10);                // bb1[9]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:33: 16:34
2020-04-13T17:39:13.4729717Z - -         _8 = const use_u8(move _9) -> bb2; // bb1[10]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4730601Z - +         _3 = const use_u8(move _4) -> bb2; // bb1[5]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:35
2020-04-13T17:39:13.4731263Z -                                            // ty::Const
2020-04-13T17:39:13.4731584Z 129                                            // + ty: fn(u8) {use_u8}
2020-04-13T17:39:13.4731943Z 130                                            // + val: Value(Scalar(<ZST>))
2020-04-13T17:39:13.4732491Z 
2020-04-13T17:39:13.4732491Z 
2020-04-13T17:39:13.4733081Z 132                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:16:5: 16:11
2020-04-13T17:39:13.4733682Z 133                                            // + literal: Const { ty: fn(u8) {use_u8}, val: Value(Scalar(<ZST>)) }
2020-04-13T17:39:13.4734118Z +                                            // ty::Const
2020-04-13T17:39:13.4734403Z +                                            // + ty: u8
2020-04-13T17:39:13.4734738Z +                                            // + val: Value(Scalar(0x2a))
2020-04-13T17:39:13.4735139Z +                                            // mir::Constant
2020-04-13T17:39:13.4735830Z +                                            // + span: $DIR/simplify-locals-removes-unused-consts.rs:16:12: 16:34
2020-04-13T17:39:13.4736392Z +                                            // + literal: Const { ty: u8, val: Value(Scalar(0x2a)) }
2020-04-13T17:39:13.4736850Z 135   
2020-04-13T17:39:13.4737017Z 136       bb2: {
2020-04-13T17:39:13.4737141Z 
2020-04-13T17:39:13.4737141Z 
2020-04-13T17:39:13.4737820Z - -         StorageDead(_9);                 // bb2[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:34: 16:35
2020-04-13T17:39:13.4738728Z - -         StorageDead(_11);                // bb2[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4740010Z - -         StorageDead(_8);                 // bb2[2]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4740917Z - -         return;                          // bb2[3]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:17:2: 17:2
2020-04-13T17:39:13.4741817Z - +         StorageDead(_4);                 // bb2[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:34: 16:35
2020-04-13T17:39:13.4742698Z - +         StorageDead(_3);                 // bb2[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4743571Z - +         return;                          // bb2[2]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:17:2: 17:2
2020-04-13T17:39:13.4744469Z + -         StorageDead(_11);                // bb2[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4745383Z + -         StorageDead(_8);                 // bb2[1]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4746270Z + -         return;                          // bb2[2]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:17:2: 17:2
2020-04-13T17:39:13.4747152Z + +         StorageDead(_2);                 // bb2[0]: scope 1 at $DIR/simplify-locals-removes-unused-consts.rs:16:35: 16:36
2020-04-13T17:39:13.4748023Z + +         return;                          // bb2[1]: scope 0 at $DIR/simplify-locals-removes-unused-consts.rs:17:2: 17:2
2020-04-13T17:39:13.4749296Z 145   }
2020-04-13T17:39:13.4749425Z 146   
2020-04-13T17:39:13.4749541Z 
2020-04-13T17:39:13.4749541Z 
2020-04-13T17:39:13.4750495Z thread '[mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify-locals-removes-unused-consts/rustc.main.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4751446Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
2020-04-13T17:39:13.4752148Z 5       debug x => _1;                       // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:1:8: 1:9
2020-04-13T17:39:13.4752148Z 5       debug x => _1;                       // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:1:8: 1:9
2020-04-13T17:39:13.4753050Z 6       let mut _0: std::option::Option<std::boxed::Box<()>>; // return place in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:1:31: 1:46
2020-04-13T17:39:13.4753940Z 7       let mut _2: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:9: 3:13
2020-04-13T17:39:13.4754753Z - -     let _3: std::boxed::Box<()>;         // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:14: 4:15
2020-04-13T17:39:13.4755576Z +       let _3: std::boxed::Box<()>;         // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:14: 4:15
2020-04-13T17:39:13.4756413Z 9 -     let mut _4: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:25: 4:26
2020-04-13T17:39:13.4757229Z 10 -     let mut _5: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:1: 6:2
2020-04-13T17:39:13.4758169Z 11 -     let mut _6: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:1: 6:2
2020-04-13T17:39:13.4758483Z 
2020-04-13T17:39:13.4759076Z - +     let mut _3: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:25: 4:26
2020-04-13T17:39:13.4760626Z - -         debug x => _4;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:14: 4:15
2020-04-13T17:39:13.4761538Z - +         debug x => _3;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:14: 4:15
2020-04-13T17:39:13.4762358Z +           debug x => _3;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:14: 4:15
2020-04-13T17:39:13.4762701Z 16       }
2020-04-13T17:39:13.4762701Z 16       }
2020-04-13T17:39:13.4762836Z 17   
2020-04-13T17:39:13.4762999Z 18       bb0: {
2020-04-13T17:39:13.4763129Z 
2020-04-13T17:39:13.4764249Z thread '[mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads/rustc.map.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4766247Z ---- [mir-opt] mir-opt/simplify_match.rs stdout ----
2020-04-13T17:39:13.4766247Z ---- [mir-opt] mir-opt/simplify_match.rs stdout ----
2020-04-13T17:39:13.4766613Z 6       let mut _1: bool;                    // in scope 0 at $DIR/simplify_match.rs:6:11: 6:31
2020-04-13T17:39:13.4767049Z 7       let _2: bool;                        // in scope 0 at $DIR/simplify_match.rs:6:17: 6:18
2020-04-13T17:39:13.4768253Z -           debug x => _1;                   // in scope 1 at $DIR/simplify_match.rs:6:17: 6:18
2020-04-13T17:39:13.4768669Z +           debug x => _2;                   // in scope 1 at $DIR/simplify_match.rs:6:17: 6:18
2020-04-13T17:39:13.4768966Z 10       }
2020-04-13T17:39:13.4769097Z 11   
2020-04-13T17:39:13.4769097Z 11   
2020-04-13T17:39:13.4769244Z 12       bb0: {
2020-04-13T17:39:13.4769381Z 
2020-04-13T17:39:13.4769700Z 13           nop;                             // bb0[0]: scope 0 at $DIR/simplify_match.rs:6:11: 6:31
2020-04-13T17:39:13.4770211Z 14           nop;                             // bb0[1]: scope 0 at $DIR/simplify_match.rs:6:17: 6:18
2020-04-13T17:39:13.4770959Z -           _1 = const false;                // bb0[2]: scope 0 at $DIR/simplify_match.rs:6:21: 6:26
2020-04-13T17:39:13.4771543Z -                                            // ty::Const
2020-04-13T17:39:13.4772016Z -                                            // + ty: bool
2020-04-13T17:39:13.4773174Z -                                            // + val: Value(Scalar(0x00))
2020-04-13T17:39:13.4773697Z -                                            // mir::Constant
2020-04-13T17:39:13.4774289Z -                                            // + span: $DIR/simplify_match.rs:6:21: 6:26
2020-04-13T17:39:13.4775018Z -                                            // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
2020-04-13T17:39:13.4775532Z +           nop;                             // bb0[2]: scope 0 at $DIR/simplify_match.rs:6:21: 6:26
2020-04-13T17:39:13.4776035Z 22           nop;                             // bb0[3]: scope 1 at $DIR/simplify_match.rs:6:28: 6:29
2020-04-13T17:39:13.4776555Z 23           nop;                             // bb0[4]: scope 0 at $DIR/simplify_match.rs:6:30: 6:31
2020-04-13T17:39:13.4777350Z -           switchInt(_1) -> [false: bb1, otherwise: bb2]; // bb0[5]: scope 0 at $DIR/simplify_match.rs:7:9: 7:13
2020-04-13T17:39:13.4778222Z + -         switchInt(const false) -> [false: bb1, otherwise: bb2]; // bb0[5]: scope 0 at $DIR/simplify_match.rs:7:9: 7:13
2020-04-13T17:39:13.4778864Z + -                                          // ty::Const
2020-04-13T17:39:13.4779336Z + -                                          // + ty: bool
2020-04-13T17:39:13.4779878Z + -                                          // + val: Value(Scalar(0x00))
2020-04-13T17:39:13.4780542Z + -                                          // mir::Constant
2020-04-13T17:39:13.4781134Z + -                                          // + span: $DIR/simplify_match.rs:6:21: 6:26
2020-04-13T17:39:13.4781857Z + -                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
2020-04-13T17:39:13.4782590Z + +         goto -> bb1;                     // bb0[5]: scope 0 at $DIR/simplify_match.rs:7:9: 7:13
2020-04-13T17:39:13.4783114Z 26   
2020-04-13T17:39:13.4783267Z 27       bb1: {
2020-04-13T17:39:13.4783390Z 
2020-04-13T17:39:13.4783390Z 
2020-04-13T17:39:13.4784238Z thread '[mir-opt] mir-opt/simplify_match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_match/rustc.main.SimplifyBranches-after-copy-prop.diff', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4785075Z ---- [mir-opt] mir-opt/simplify_try.rs stdout ----
2020-04-13T17:39:13.4785075Z ---- [mir-opt] mir-opt/simplify_try.rs stdout ----
2020-04-13T17:39:13.4785423Z 15       let _10: u32;                        // in scope 0 at $DIR/simplify_try.rs:6:13: 6:15
2020-04-13T17:39:13.4785861Z 16       let mut _11: u32;                    // in scope 0 at $DIR/simplify_try.rs:7:8: 7:9
2020-04-13T17:39:13.4786649Z -           debug y => _11;                  // in scope 1 at $DIR/simplify_try.rs:6:9: 6:10
2020-04-13T17:39:13.4787064Z +           debug y => _10;                  // in scope 1 at $DIR/simplify_try.rs:6:9: 6:10
2020-04-13T17:39:13.4787336Z 19       }
2020-04-13T17:39:13.4787487Z 20       scope 2 {
2020-04-13T17:39:13.4787487Z 20       scope 2 {
2020-04-13T17:39:13.4788005Z -           debug err => _8;                 // in scope 2 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4788410Z +           debug err => _6;                 // in scope 2 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4788699Z 22           scope 3 {
2020-04-13T17:39:13.4788896Z 23               scope 7 {
2020-04-13T17:39:13.4789421Z -                   debug t => _8;           // in scope 7 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
2020-04-13T17:39:13.4789836Z +                   debug t => _6;           // in scope 7 at $SRC_DIR/libcore/convert/mod.rs:LL:COL
2020-04-13T17:39:13.4790311Z 26               scope 8 {
2020-04-13T17:39:13.4790311Z 26               scope 8 {
2020-04-13T17:39:13.4790819Z -                   debug v => _8;           // in scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4791237Z +                   debug v => _6;           // in scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4791651Z +                   let mut _12: i32;        // in scope 8 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4792118Z 29           }
2020-04-13T17:39:13.4792262Z 30       }
2020-04-13T17:39:13.4792367Z 
2020-04-13T17:39:13.4792516Z 31       scope 4 {
2020-04-13T17:39:13.4792516Z 31       scope 4 {
2020-04-13T17:39:13.4793299Z -           debug val => _11;                // in scope 4 at $DIR/simplify_try.rs:6:13: 6:15
2020-04-13T17:39:13.4793712Z +           debug val => _10;                // in scope 4 at $DIR/simplify_try.rs:6:13: 6:15
2020-04-13T17:39:13.4794013Z 33           scope 5 {
2020-04-13T17:39:13.4794176Z 34           }
2020-04-13T17:39:13.4794320Z 35       }
2020-04-13T17:39:13.4794425Z 
2020-04-13T17:39:13.4794575Z 36       scope 6 {
2020-04-13T17:39:13.4795094Z -           debug self => _3;                // in scope 6 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4795498Z +           debug self => _1;                // in scope 6 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4795909Z 39   
2020-04-13T17:39:13.4796056Z 40       bb0: {
2020-04-13T17:39:13.4796174Z 
2020-04-13T17:39:13.4796174Z 
2020-04-13T17:39:13.4796721Z -           _3 = _1;                         // bb0[0]: scope 0 at $DIR/simplify_try.rs:6:13: 6:14
2020-04-13T17:39:13.4797436Z -           _5 = discriminant(_3);           // bb0[1]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4798323Z -           switchInt(move _5) -> [0isize: bb1, otherwise: bb2]; // bb0[2]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4798880Z +           _5 = discriminant(_1);           // bb0[0]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4799671Z +           switchInt(move _5) -> [0isize: bb1, otherwise: bb2]; // bb0[1]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4800191Z 45   
2020-04-13T17:39:13.4800337Z 46       bb1: {
2020-04-13T17:39:13.4800528Z 
2020-04-13T17:39:13.4800528Z 
2020-04-13T17:39:13.4801108Z - -         _11 = ((_3 as Ok).0: u32);       // bb1[0]: scope 0 at $DIR/simplify_try.rs:6:13: 6:15
2020-04-13T17:39:13.4801857Z - -         ((_0 as Ok).0: u32) = move _11;  // bb1[1]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4802601Z + -         _10 = ((_1 as Ok).0: u32);       // bb1[0]: scope 0 at $DIR/simplify_try.rs:6:13: 6:15
2020-04-13T17:39:13.4803486Z + -         ((_0 as Ok).0: u32) = move _10;  // bb1[1]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4804225Z 49 -         discriminant(_0) = 0;            // bb1[2]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4804952Z - +         _0 = move _3;                    // bb1[0]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4805440Z + +         _0 = move _1;                    // bb1[0]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4805927Z 51 +         nop;                             // bb1[1]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4807225Z 52 +         nop;                             // bb1[2]: scope 1 at $DIR/simplify_try.rs:7:5: 7:10
2020-04-13T17:39:13.4807978Z 53           goto -> bb3;                     // bb1[3]: scope 0 at $DIR/simplify_try.rs:8:2: 8:2
2020-04-13T17:39:13.4808384Z 54       }
2020-04-13T17:39:13.4808530Z 55   
2020-04-13T17:39:13.4808675Z 56       bb2: {
2020-04-13T17:39:13.4808675Z 56       bb2: {
2020-04-13T17:39:13.4809256Z - -         _8 = ((_3 as Err).0: i32);       // bb2[0]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4810017Z - -         ((_0 as Err).0: i32) = move _8;  // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4810763Z + -         _6 = ((_1 as Err).0: i32);       // bb2[0]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4811518Z + -         ((_0 as Err).0: i32) = move _6;  // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4812253Z 59 -         discriminant(_0) = 1;            // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4812968Z - +         _0 = move _3;                    // bb2[0]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4813473Z + +         _0 = move _1;                    // bb2[0]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4813965Z 61 +         nop;                             // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4814460Z 62 +         nop;                             // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL
2020-04-13T17:39:13.4815194Z 63           goto -> bb3;                     // bb2[3]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-13T17:39:13.4815478Z 
2020-04-13T17:39:13.4816289Z thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try/rustc.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3152:25
2020-04-13T17:39:13.4816838Z 
2020-04-13T17:39:13.4816967Z failures:
2020-04-13T17:39:13.4817328Z     [mir-opt] mir-opt/const_allocation.rs
2020-04-13T17:39:13.4817747Z     [mir-opt] mir-opt/const_allocation2.rs
---
2020-04-13T17:39:13.4824272Z test result: FAILED. 76 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-13T17:39:13.4824502Z 
2020-04-13T17:39:13.4824606Z 
2020-04-13T17:39:13.4824697Z 
2020-04-13T17:39:13.4828467Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-13T17:39:13.4830795Z 
2020-04-13T17:39:13.4830890Z 
2020-04-13T17:39:14.0283837Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-13T17:39:14.0284311Z Build completed unsuccessfully in 1:03:18
2020-04-13T17:39:14.0284311Z Build completed unsuccessfully in 1:03:18
2020-04-13T17:39:14.0284545Z == clock drift check ==
2020-04-13T17:39:14.0284771Z   local time: Mon Apr 13 17:39:13 UTC 2020
2020-04-13T17:39:14.0285062Z   network time: Mon, 13 Apr 2020 17:39:13 GMT
2020-04-13T17:39:15.9772566Z 
2020-04-13T17:39:15.9772566Z 
2020-04-13T17:39:15.9842187Z ##[error]Bash exited with code '1'.
2020-04-13T17:39:15.9877202Z ##[section]Finishing: Run build
2020-04-13T17:39:15.9930441Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T17:39:15.9935439Z Task         : Get sources
2020-04-13T17:39:15.9935747Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:39:15.9936028Z Version      : 1.0.0
2020-04-13T17:39:15.9936251Z Author       : Microsoft
2020-04-13T17:39:15.9936251Z Author       : Microsoft
2020-04-13T17:39:15.9936572Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T17:39:15.9936929Z ==============================================================================
2020-04-13T17:39:16.3230007Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T17:39:16.3283180Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T17:39:16.3371153Z Cleaning up task key
2020-04-13T17:39:16.3372523Z Start cleaning up orphan processes.
2020-04-13T17:39:16.3550929Z Terminate orphan process: pid (5208) (python)
2020-04-13T17:39:16.3712311Z ##[section]Finishing: Finalize Job
