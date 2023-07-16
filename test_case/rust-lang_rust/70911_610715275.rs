plain
2020-04-08T01:10:14.9649573Z ========================== Starting Command Output ===========================
2020-04-08T01:10:14.9651923Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec6af01f-33ae-4aba-96bc-fbe0e0ee2386.sh
2020-04-08T01:10:14.9652155Z 
2020-04-08T01:10:14.9655175Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T01:10:14.9672720Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70911/merge to s
2020-04-08T01:10:14.9675844Z Task         : Get sources
2020-04-08T01:10:14.9676104Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T01:10:14.9676353Z Version      : 1.0.0
2020-04-08T01:10:14.9676520Z Author       : Microsoft
---
2020-04-08T01:10:16.2035992Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T01:10:16.2043489Z ##[command]git config gc.auto 0
2020-04-08T01:10:16.2047369Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T01:10:16.2051074Z ##[command]git config --get-all http.proxy
2020-04-08T01:10:16.2057991Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70911/merge:refs/remotes/pull/70911/merge
---
2020-04-08T01:13:29.1052016Z Looks like docker image is the same as before, not uploading
2020-04-08T01:13:37.4085835Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T01:13:37.4412956Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T01:13:37.4440296Z == clock drift check ==
2020-04-08T01:13:37.4449315Z   local time: Wed Apr  8 01:13:37 UTC 2020
2020-04-08T01:13:37.7361119Z   network time: Wed, 08 Apr 2020 01:13:37 GMT
2020-04-08T01:13:37.7386188Z Starting sccache server...
2020-04-08T01:13:37.8224692Z configure: processing command line
2020-04-08T01:13:37.8226775Z configure: 
2020-04-08T01:13:37.8227904Z configure: rust.dist-src        := False
---
2020-04-08T01:19:06.4654423Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T01:19:08.0088755Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T01:19:09.6853234Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T01:19:10.9070864Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T01:19:20.5726365Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T01:19:23.0981658Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T01:19:27.6994005Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T01:19:32.1723861Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T01:19:42.4246307Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T01:43:18.0863884Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T01:43:19.9224288Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T01:43:22.0182756Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T01:43:24.9853357Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T01:43:35.3888855Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T01:43:39.6042449Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T01:43:45.0110387Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T01:43:50.6863663Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T01:44:01.3452314Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T02:10:23.3614353Z .................................................................................................... 1700/9881
2020-04-08T02:10:27.2981360Z .................................................................................................... 1800/9881
2020-04-08T02:10:36.2335402Z ..................................................................................................i. 1900/9881
2020-04-08T02:10:44.1889443Z .................................................................................................... 2000/9881
2020-04-08T02:10:50.5266787Z ........................................................................................iiiii....... 2100/9881
2020-04-08T02:11:11.9107908Z .................................................................................................... 2300/9881
2020-04-08T02:11:13.9953786Z .................................................................................................... 2400/9881
2020-04-08T02:11:16.1027822Z .................................................................................................... 2500/9881
2020-04-08T02:11:22.0388091Z .................................................................................................... 2600/9881
---
2020-04-08T02:14:16.5587940Z ..............................................................i...............i..................... 5000/9881
2020-04-08T02:14:23.7802329Z .................................................................................................... 5100/9881
2020-04-08T02:14:31.3947857Z .................................................................................................... 5200/9881
2020-04-08T02:14:36.8311110Z .......i............................................................................................ 5300/9881
2020-04-08T02:14:46.9202272Z ................................................................................................ii.i 5400/9881
2020-04-08T02:14:51.8193296Z i........i...i...................................................................................... 5500/9881
2020-04-08T02:15:00.2775716Z .........................................i.......................................................... 5700/9881
2020-04-08T02:15:10.3148737Z .............................................................ii..................................... 5800/9881
2020-04-08T02:15:17.3003407Z i................................................................................................... 5900/9881
2020-04-08T02:15:22.5007325Z .................................................................................................... 6000/9881
2020-04-08T02:15:22.5007325Z .................................................................................................... 6000/9881
2020-04-08T02:15:31.9773217Z ...............................................................................................ii... 6100/9881
2020-04-08T02:15:43.7009107Z i..ii...........i................................................................................... 6200/9881
2020-04-08T02:15:59.5037690Z .................................................................................................... 6400/9881
2020-04-08T02:16:04.0324883Z .................................................................................................... 6500/9881
2020-04-08T02:16:16.8847059Z ..........................i.ii...................................................................... 6600/9881
2020-04-08T02:16:23.1496649Z .................................................................................................... 6700/9881
---
2020-04-08T02:18:24.8393082Z .................................................................................................... 7800/9881
2020-04-08T02:18:29.1383055Z .................................................................................................... 7900/9881
2020-04-08T02:18:35.1431638Z .................................................................................................... 8000/9881
2020-04-08T02:18:42.7729246Z .............................i...................................................................... 8100/9881
2020-04-08T02:18:51.0647055Z .............................................................................iiiiii.iiii.i.......... 8200/9881
2020-04-08T02:19:07.1031406Z ......................i......i...................................................................... 8400/9881
2020-04-08T02:19:11.5814696Z .................................................................................................... 8500/9881
2020-04-08T02:19:22.7186734Z .................................................................................................... 8600/9881
2020-04-08T02:19:34.9649853Z .................................................................................................... 8700/9881
---
2020-04-08T02:21:59.7485183Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-08T02:21:59.7664633Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:21:59.9662329Z 
2020-04-08T02:21:59.9663025Z running 185 tests
2020-04-08T02:22:02.6906933Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-08T02:22:05.2682608Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-08T02:22:05.2691117Z 
2020-04-08T02:22:05.2691254Z  finished in 5.502
2020-04-08T02:22:05.2695349Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-08T02:22:05.2875077Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:22:07.3566240Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-08T02:22:07.3733021Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:22:07.5279686Z 
2020-04-08T02:22:07.5280102Z running 9 tests
2020-04-08T02:22:07.5284377Z iiiiiiiii
2020-04-08T02:22:07.5285561Z 
2020-04-08T02:22:07.5290861Z  finished in 0.155
2020-04-08T02:22:07.5296902Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-08T02:22:07.5467687Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:22:27.6129726Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-08T02:22:27.6332011Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:22:27.8147877Z 
2020-04-08T02:22:27.8148668Z running 115 tests
2020-04-08T02:22:41.0125844Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-08T02:22:42.4513401Z ...iiii.....ii.
2020-04-08T02:22:42.4515262Z 
2020-04-08T02:22:42.4520792Z  finished in 14.818
2020-04-08T02:22:42.4528606Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-08T02:22:42.4529259Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:23:21.8641448Z -   --> $DIR/ty_tykind_usage.rs:11:16
2020-04-08T02:23:21.8642054Z + error[E0599]: no variant or associated item named `Str` found for enum `rustc_middle::ty::TyKind<'_>` in the current scope
2020-04-08T02:23:21.8643854Z +   --> $DIR/ty_tykind_usage.rs:21:17
2020-04-08T02:23:21.8644083Z 3    |
2020-04-08T02:23:21.8644518Z - LL |     let kind = TyKind::Bool;
2020-04-08T02:23:21.8645078Z -    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8645873Z - note: the lint level is defined here
2020-04-08T02:23:21.8646298Z -   --> $DIR/ty_tykind_usage.rs:9:8
2020-04-08T02:23:21.8646629Z -    |
2020-04-08T02:23:21.8646629Z -    |
2020-04-08T02:23:21.8647001Z - LL | #[deny(rustc::usage_of_ty_tykind)]
2020-04-08T02:23:21.8648064Z - 
2020-04-08T02:23:21.8648383Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8649069Z -   --> $DIR/ty_tykind_usage.rs:14:9
2020-04-08T02:23:21.8649388Z -    |
2020-04-08T02:23:21.8649388Z -    |
2020-04-08T02:23:21.8649690Z - LL |         TyKind::Bool => (),
2020-04-08T02:23:21.8650150Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8650786Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8651181Z -   --> $DIR/ty_tykind_usage.rs:15:9
2020-04-08T02:23:21.8651463Z -    |
2020-04-08T02:23:21.8651763Z - LL |         TyKind::Char => (),
2020-04-08T02:23:21.8651763Z - LL |         TyKind::Char => (),
2020-04-08T02:23:21.8652215Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8652852Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8653223Z -   --> $DIR/ty_tykind_usage.rs:16:9
2020-04-08T02:23:21.8653522Z -    |
2020-04-08T02:23:21.8653828Z - LL |         TyKind::Int(..) => (),
2020-04-08T02:23:21.8653828Z - LL |         TyKind::Int(..) => (),
2020-04-08T02:23:21.8654268Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8654923Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8655291Z -   --> $DIR/ty_tykind_usage.rs:17:9
2020-04-08T02:23:21.8655590Z -    |
2020-04-08T02:23:21.8655901Z - LL |         TyKind::Uint(..) => (),
2020-04-08T02:23:21.8655901Z - LL |         TyKind::Uint(..) => (),
2020-04-08T02:23:21.8656340Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8656999Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8657366Z -   --> $DIR/ty_tykind_usage.rs:18:9
2020-04-08T02:23:21.8657661Z -    |
2020-04-08T02:23:21.8657974Z - LL |         TyKind::Float(..) => (),
2020-04-08T02:23:21.8657974Z - LL |         TyKind::Float(..) => (),
2020-04-08T02:23:21.8658416Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8659069Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8659436Z -   --> $DIR/ty_tykind_usage.rs:19:9
2020-04-08T02:23:21.8659715Z -    |
2020-04-08T02:23:21.8660042Z - LL |         TyKind::Adt(..) => (),
2020-04-08T02:23:21.8660042Z - LL |         TyKind::Adt(..) => (),
2020-04-08T02:23:21.8660484Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8661322Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8661704Z -   --> $DIR/ty_tykind_usage.rs:20:9
2020-04-08T02:23:21.8662159Z -    |
2020-04-08T02:23:21.8662493Z - LL |         TyKind::Foreign(..) => (),
2020-04-08T02:23:21.8662493Z - LL |         TyKind::Foreign(..) => (),
2020-04-08T02:23:21.8662943Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8663601Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8663971Z -   --> $DIR/ty_tykind_usage.rs:21:9
2020-04-08T02:23:21.8664250Z -    |
2020-04-08T02:23:21.8664437Z 58 LL |         TyKind::Str => (),
2020-04-08T02:23:21.8664437Z 58 LL |         TyKind::Str => (),
2020-04-08T02:23:21.8664875Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8665455Z +    |                 ^^^ variant or associated item not found in `rustc_middle::ty::TyKind<'_>`
2020-04-08T02:23:21.8666065Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8666432Z -   --> $DIR/ty_tykind_usage.rs:22:9
2020-04-08T02:23:21.8666714Z -    |
2020-04-08T02:23:21.8667047Z - LL |         TyKind::Array(..) => (),
2020-04-08T02:23:21.8667047Z - LL |         TyKind::Array(..) => (),
2020-04-08T02:23:21.8667488Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8667947Z 66 
2020-04-08T02:23:21.8668352Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8668720Z -   --> $DIR/ty_tykind_usage.rs:23:9
2020-04-08T02:23:21.8669040Z -    |
2020-04-08T02:23:21.8669040Z -    |
2020-04-08T02:23:21.8669352Z - LL |         TyKind::Slice(..) => (),
2020-04-08T02:23:21.8669794Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8670688Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8671066Z -   --> $DIR/ty_tykind_usage.rs:24:9
2020-04-08T02:23:21.8671528Z -    |
2020-04-08T02:23:21.8671845Z - LL |         TyKind::RawPtr(..) => (),
2020-04-08T02:23:21.8671845Z - LL |         TyKind::RawPtr(..) => (),
2020-04-08T02:23:21.8672289Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8673157Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8673542Z -   --> $DIR/ty_tykind_usage.rs:25:9
2020-04-08T02:23:21.8673813Z -    |
2020-04-08T02:23:21.8674133Z - LL |         TyKind::Ref(..) => (),
2020-04-08T02:23:21.8674133Z - LL |         TyKind::Ref(..) => (),
2020-04-08T02:23:21.8674559Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8675202Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8675558Z -   --> $DIR/ty_tykind_usage.rs:26:9
2020-04-08T02:23:21.8675828Z -    |
2020-04-08T02:23:21.8676147Z - LL |         TyKind::FnDef(..) => (),
2020-04-08T02:23:21.8676147Z - LL |         TyKind::FnDef(..) => (),
2020-04-08T02:23:21.8676798Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8677447Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8677816Z -   --> $DIR/ty_tykind_usage.rs:27:9
2020-04-08T02:23:21.8678094Z -    |
2020-04-08T02:23:21.8678405Z - LL |         TyKind::FnPtr(..) => (),
2020-04-08T02:23:21.8678405Z - LL |         TyKind::FnPtr(..) => (),
2020-04-08T02:23:21.8679022Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8679826Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8680202Z -   --> $DIR/ty_tykind_usage.rs:28:9
2020-04-08T02:23:21.8680472Z -    |
2020-04-08T02:23:21.8680779Z - LL |         TyKind::Dynamic(..) => (),
2020-04-08T02:23:21.8680779Z - LL |         TyKind::Dynamic(..) => (),
2020-04-08T02:23:21.8681237Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8681849Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8682223Z -   --> $DIR/ty_tykind_usage.rs:29:9
2020-04-08T02:23:21.8682495Z -    |
2020-04-08T02:23:21.8682801Z - LL |         TyKind::Closure(..) => (),
2020-04-08T02:23:21.8682801Z - LL |         TyKind::Closure(..) => (),
2020-04-08T02:23:21.8683248Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8683860Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8684214Z -   --> $DIR/ty_tykind_usage.rs:30:9
2020-04-08T02:23:21.8687766Z -    |
2020-04-08T02:23:21.8688134Z - LL |         TyKind::Generator(..) => (),
2020-04-08T02:23:21.8688134Z - LL |         TyKind::Generator(..) => (),
2020-04-08T02:23:21.8688589Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8689253Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8689622Z -   --> $DIR/ty_tykind_usage.rs:31:9
2020-04-08T02:23:21.8689924Z -    |
2020-04-08T02:23:21.8689924Z -    |
2020-04-08T02:23:21.8690280Z - LL |         TyKind::GeneratorWitness(..) => (),
2020-04-08T02:23:21.8690901Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8691535Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8691890Z -   --> $DIR/ty_tykind_usage.rs:32:9
2020-04-08T02:23:21.8692180Z -    |
2020-04-08T02:23:21.8692907Z - LL |         TyKind::Never => (),
2020-04-08T02:23:21.8692907Z - LL |         TyKind::Never => (),
2020-04-08T02:23:21.8693323Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8694542Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8694923Z -   --> $DIR/ty_tykind_usage.rs:33:9
2020-04-08T02:23:21.8695218Z -    |
2020-04-08T02:23:21.8695738Z - LL |         TyKind::Tuple(..) => (),
2020-04-08T02:23:21.8695738Z - LL |         TyKind::Tuple(..) => (),
2020-04-08T02:23:21.8696209Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8697045Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8697597Z -   --> $DIR/ty_tykind_usage.rs:34:9
2020-04-08T02:23:21.8698045Z -    |
2020-04-08T02:23:21.8698386Z - LL |         TyKind::Projection(..) => (),
2020-04-08T02:23:21.8698386Z - LL |         TyKind::Projection(..) => (),
2020-04-08T02:23:21.8700202Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8701042Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8712646Z -   --> $DIR/ty_tykind_usage.rs:35:9
2020-04-08T02:23:21.8713011Z -    |
2020-04-08T02:23:21.8713445Z - LL |         TyKind::UnnormalizedProjection(..) => (),
2020-04-08T02:23:21.8713445Z - LL |         TyKind::UnnormalizedProjection(..) => (),
2020-04-08T02:23:21.8713969Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8715220Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8715730Z -   --> $DIR/ty_tykind_usage.rs:36:9
2020-04-08T02:23:21.8716044Z -    |
2020-04-08T02:23:21.8716396Z - LL |         TyKind::Opaque(..) => (),
2020-04-08T02:23:21.8716396Z - LL |         TyKind::Opaque(..) => (),
2020-04-08T02:23:21.8717058Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8718356Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8719016Z -   --> $DIR/ty_tykind_usage.rs:37:9
2020-04-08T02:23:21.8719363Z -    |
2020-04-08T02:23:21.8719889Z - LL |         TyKind::Param(..) => (),
2020-04-08T02:23:21.8719889Z - LL |         TyKind::Param(..) => (),
2020-04-08T02:23:21.8720380Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8721061Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8721625Z -   --> $DIR/ty_tykind_usage.rs:38:9
2020-04-08T02:23:21.8721922Z -    |
2020-04-08T02:23:21.8722247Z - LL |         TyKind::Bound(..) => (),
2020-04-08T02:23:21.8722247Z - LL |         TyKind::Bound(..) => (),
2020-04-08T02:23:21.8722718Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8723397Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8723778Z -   --> $DIR/ty_tykind_usage.rs:39:9
2020-04-08T02:23:21.8724137Z -    |
2020-04-08T02:23:21.8724492Z - LL |         TyKind::Placeholder(..) => (),
2020-04-08T02:23:21.8724492Z - LL |         TyKind::Placeholder(..) => (),
2020-04-08T02:23:21.8724952Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8725646Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8726027Z -   --> $DIR/ty_tykind_usage.rs:40:9
2020-04-08T02:23:21.8726340Z -    |
2020-04-08T02:23:21.8726668Z - LL |         TyKind::Infer(..) => (),
2020-04-08T02:23:21.8726668Z - LL |         TyKind::Infer(..) => (),
2020-04-08T02:23:21.8727342Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8728061Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8728451Z -   --> $DIR/ty_tykind_usage.rs:41:9
2020-04-08T02:23:21.8728771Z -    |
2020-04-08T02:23:21.8729105Z - LL |         TyKind::Error => (),
2020-04-08T02:23:21.8729105Z - LL |         TyKind::Error => (),
2020-04-08T02:23:21.8729571Z -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8730442Z - error: usage of `ty::TyKind::<kind>`
2020-04-08T02:23:21.8730824Z -   --> $DIR/ty_tykind_usage.rs:46:12
2020-04-08T02:23:21.8731117Z -    |
2020-04-08T02:23:21.8731492Z - LL |     if let TyKind::Int(int_ty) = kind {}
2020-04-08T02:23:21.8731492Z - LL |     if let TyKind::Int(int_ty) = kind {}
2020-04-08T02:23:21.8731974Z -    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-04-08T02:23:21.8732642Z - error: usage of `ty::TyKind`
2020-04-08T02:23:21.8733012Z -   --> $DIR/ty_tykind_usage.rs:48:24
2020-04-08T02:23:21.8733308Z -    |
2020-04-08T02:23:21.8733308Z -    |
2020-04-08T02:23:21.8733715Z - LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {}
2020-04-08T02:23:21.8734578Z -    |
2020-04-08T02:23:21.8734899Z -    = help: try using `Ty` instead
2020-04-08T02:23:21.8735178Z - 
2020-04-08T02:23:21.8735494Z - error: aborting due to 31 previous errors
2020-04-08T02:23:21.8735494Z - error: aborting due to 31 previous errors
2020-04-08T02:23:21.8735776Z - 
2020-04-08T02:23:21.8736169Z + For more information about this error, try `rustc --explain E0599`.
2020-04-08T02:23:21.8736371Z 197 
2020-04-08T02:23:21.8736458Z 
2020-04-08T02:23:21.8736535Z 
2020-04-08T02:23:21.8736719Z The actual stderr differed from the expected stderr.
2020-04-08T02:23:21.8737310Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2020-04-08T02:23:21.8738014Z To update references, rerun the tests and pass the `--bless` flag
2020-04-08T02:23:21.8738540Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2020-04-08T02:23:21.8738904Z error: 1 errors occurred comparing output.
2020-04-08T02:23:21.8739113Z status: exit code: 1
2020-04-08T02:23:21.8739113Z status: exit code: 1
2020-04-08T02:23:21.8740909Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary"
2020-04-08T02:23:21.8742363Z ------------------------------------------
2020-04-08T02:23:21.8742507Z 
2020-04-08T02:23:21.8742834Z ------------------------------------------
2020-04-08T02:23:21.8743000Z stderr:
---
2020-04-08T02:23:21.8748303Z test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-08T02:23:21.8748539Z 
2020-04-08T02:23:21.8748617Z 
2020-04-08T02:23:21.8748693Z 
2020-04-08T02:23:21.8751786Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-08T02:23:21.8754392Z 
2020-04-08T02:23:21.8754472Z 
2020-04-08T02:23:21.8754942Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-08T02:23:21.8755244Z Build completed unsuccessfully in 1:08:07
2020-04-08T02:23:21.8755244Z Build completed unsuccessfully in 1:08:07
2020-04-08T02:23:21.8755492Z == clock drift check ==
2020-04-08T02:23:21.8755709Z   local time: Wed Apr  8 02:23:21 UTC 2020
2020-04-08T02:23:22.1698903Z   network time: Wed, 08 Apr 2020 02:23:22 GMT
2020-04-08T02:23:23.1222606Z 
2020-04-08T02:23:23.1222606Z 
2020-04-08T02:23:23.1295231Z ##[error]Bash exited with code '1'.
2020-04-08T02:23:23.1309306Z ##[section]Finishing: Run build
2020-04-08T02:23:23.1363051Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70911/merge to s
2020-04-08T02:23:23.1367493Z Task         : Get sources
2020-04-08T02:23:23.1367777Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T02:23:23.1368056Z Version      : 1.0.0
2020-04-08T02:23:23.1368239Z Author       : Microsoft
2020-04-08T02:23:23.1368239Z Author       : Microsoft
2020-04-08T02:23:23.1368536Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T02:23:23.1368890Z ==============================================================================
2020-04-08T02:23:23.4800081Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T02:23:23.4845628Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70911/merge to s
2020-04-08T02:23:23.4933400Z Cleaning up task key
2020-04-08T02:23:23.4934456Z Start cleaning up orphan processes.
2020-04-08T02:23:23.5106024Z Terminate orphan process: pid (4050) (python)
2020-04-08T02:23:23.5273729Z ##[section]Finishing: Finalize Job
