plain
2019-09-25T14:58:41.0691367Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T14:58:41.0937618Z ##[command]git config gc.auto 0
2019-09-25T14:58:41.1023836Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T14:58:41.1087903Z ##[command]git config --get-all http.proxy
2019-09-25T14:58:41.1251526Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64513/merge:refs/remotes/pull/64513/merge
---
2019-09-25T16:03:19.8690773Z .................................................................................................... 1500/9043
2019-09-25T16:03:26.4539264Z .................................................................................................... 1600/9043
2019-09-25T16:03:39.6590551Z .........................................................................i...............i.......... 1700/9043
2019-09-25T16:03:46.9573053Z .................................................................................................... 1800/9043
2019-09-25T16:03:56.1364068Z ................................................................iiiii............................... 1900/9043
2019-09-25T16:04:16.7917669Z .................................................................................................... 2100/9043
2019-09-25T16:04:19.5087665Z .................................................................................................... 2200/9043
2019-09-25T16:04:23.0561102Z .................................................................................................... 2300/9043
2019-09-25T16:04:31.8532664Z .................................................................................................... 2400/9043
---
2019-09-25T16:07:41.0889647Z .......................................................i...............i............................ 4700/9043
2019-09-25T16:07:51.0240355Z .................................................................................................... 4800/9043
2019-09-25T16:08:00.2866407Z .................................................................................................... 4900/9043
2019-09-25T16:08:08.4194462Z .................................................................................................... 5000/9043
2019-09-25T16:08:18.8384296Z ..........................................ii.ii..................................................... 5100/9043
2019-09-25T16:08:29.7736219Z .................................................................................................... 5300/9043
2019-09-25T16:08:40.8694458Z .................................................................................................... 5400/9043
2019-09-25T16:08:48.8647401Z .......i............................................................................................ 5500/9043
2019-09-25T16:08:54.7900690Z .................................................................................................... 5600/9043
2019-09-25T16:08:54.7900690Z .................................................................................................... 5600/9043
2019-09-25T16:09:07.9238303Z .................................................................................................... 5700/9043
2019-09-25T16:09:21.8872575Z ..ii...i...ii..........i............................................................................ 5800/9043
2019-09-25T16:09:44.7397289Z .................................................................................................... 6000/9043
2019-09-25T16:09:54.0333622Z .................................................................................................... 6100/9043
2019-09-25T16:09:54.0333622Z .................................................................................................... 6100/9043
2019-09-25T16:10:08.9838699Z ....i..ii........................................................................................... 6200/9043
2019-09-25T16:10:29.1154204Z ................................................................i................................... 6400/9043
2019-09-25T16:10:31.4647794Z .................................................................................................... 6500/9043
2019-09-25T16:10:34.2391070Z ....................................i............................................................... 6600/9043
2019-09-25T16:10:38.4857089Z .................................................................................................... 6700/9043
---
2019-09-25T16:15:19.8130527Z  finished in 5.473
2019-09-25T16:15:19.8322845Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:15:20.0040835Z 
2019-09-25T16:15:20.0041178Z running 150 tests
2019-09-25T16:15:23.4016770Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-25T16:15:25.4578244Z ..iiii..............i.........iii.i.......ii......
2019-09-25T16:15:25.4578922Z 
2019-09-25T16:15:25.4583226Z  finished in 5.626
2019-09-25T16:15:25.4768480Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:15:25.6386927Z 
---
2019-09-25T16:15:27.7990768Z  finished in 2.322
2019-09-25T16:15:27.8201594Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:15:27.9811697Z 
2019-09-25T16:15:27.9812031Z running 9 tests
2019-09-25T16:15:27.9812904Z iiiiiiiii
2019-09-25T16:15:27.9813264Z 
2019-09-25T16:15:27.9818227Z  finished in 0.161
2019-09-25T16:15:28.0021496Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:15:28.1719968Z 
---
2019-09-25T16:15:46.8993068Z  finished in 18.897
2019-09-25T16:15:46.9209780Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:15:47.0927362Z 
2019-09-25T16:15:47.0928893Z running 123 tests
2019-09-25T16:16:12.9170801Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-25T16:16:17.9078569Z i.i.i......iii.i.....ii
2019-09-25T16:16:17.9080054Z 
2019-09-25T16:16:17.9082471Z  finished in 30.987
2019-09-25T16:16:17.9096938Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T16:16:17.9097711Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-25T16:17:22.5320099Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T16:17:22.5321145Z ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
2019-09-25T16:17:22.5321426Z diff of stderr:
2019-09-25T16:17:22.5321598Z 
2019-09-25T16:17:22.5321759Z 1 error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5322175Z -   --> $DIR/ty_tykind_usage.rs:11:15
2019-09-25T16:17:22.5322955Z +   --> $DIR/ty_tykind_usage.rs:11:16
2019-09-25T16:17:22.5323164Z 3    |
2019-09-25T16:17:22.5323310Z 4 LL |     let kind = TyKind::Bool;
2019-09-25T16:17:22.5323731Z -    |               ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5323951Z +    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5324268Z 7 note: lint level defined here
2019-09-25T16:17:22.5324675Z 8   --> $DIR/ty_tykind_usage.rs:9:8
2019-09-25T16:17:22.5324843Z 
2019-09-25T16:17:22.5324988Z 
2019-09-25T16:17:22.5324988Z 
2019-09-25T16:17:22.5325157Z The actual stderr differed from the expected stderr.
2019-09-25T16:17:22.5325631Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2019-09-25T16:17:22.5326103Z To update references, rerun the tests and pass the `--bless` flag
2019-09-25T16:17:22.5326592Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2019-09-25T16:17:22.5326953Z error: 1 errors occurred comparing output.
2019-09-25T16:17:22.5327099Z status: exit code: 1
2019-09-25T16:17:22.5327099Z status: exit code: 1
2019-09-25T16:17:22.5328231Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
2019-09-25T16:17:22.5329328Z ------------------------------------------
2019-09-25T16:17:22.5329537Z 
2019-09-25T16:17:22.5329908Z ------------------------------------------
2019-09-25T16:17:22.5330592Z stderr:
2019-09-25T16:17:22.5330592Z stderr:
2019-09-25T16:17:22.5331428Z ------------------------------------------
2019-09-25T16:17:22.5331667Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5332308Z    |
2019-09-25T16:17:22.5332308Z    |
2019-09-25T16:17:22.5332460Z LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5332613Z    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5332947Z note: lint level defined here
2019-09-25T16:17:22.5333372Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:9:8
2019-09-25T16:17:22.5333568Z    |
2019-09-25T16:17:22.5333568Z    |
2019-09-25T16:17:22.5333733Z LL | #[deny(rustc::usage_of_ty_tykind)]
2019-09-25T16:17:22.5334003Z 
2019-09-25T16:17:22.5334003Z 
2019-09-25T16:17:22.5334147Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5334797Z    |
2019-09-25T16:17:22.5334797Z    |
2019-09-25T16:17:22.5334950Z LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5335103Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5335247Z 
2019-09-25T16:17:22.5335392Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5335989Z    |
2019-09-25T16:17:22.5335989Z    |
2019-09-25T16:17:22.5336148Z LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5336322Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5336464Z 
2019-09-25T16:17:22.5336612Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5338021Z    |
2019-09-25T16:17:22.5338021Z    |
2019-09-25T16:17:22.5338201Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5338865Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5339058Z 
2019-09-25T16:17:22.5339226Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5339925Z    |
2019-09-25T16:17:22.5339925Z    |
2019-09-25T16:17:22.5340076Z LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5340252Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5340376Z 
2019-09-25T16:17:22.5340547Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5341262Z    |
2019-09-25T16:17:22.5341262Z    |
2019-09-25T16:17:22.5341416Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5341587Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5341728Z 
2019-09-25T16:17:22.5341872Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5342497Z    |
2019-09-25T16:17:22.5342497Z    |
2019-09-25T16:17:22.5342668Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5342820Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5342945Z 
2019-09-25T16:17:22.5343123Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5343866Z    |
2019-09-25T16:17:22.5343866Z    |
2019-09-25T16:17:22.5344068Z LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5344231Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5344358Z 
2019-09-25T16:17:22.5344523Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5345194Z    |
2019-09-25T16:17:22.5345194Z    |
2019-09-25T16:17:22.5345447Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5345626Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5345756Z 
2019-09-25T16:17:22.5345899Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5346542Z    |
2019-09-25T16:17:22.5346542Z    |
2019-09-25T16:17:22.5346706Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5346872Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5346998Z 
2019-09-25T16:17:22.5347139Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5347769Z    |
2019-09-25T16:17:22.5347769Z    |
2019-09-25T16:17:22.5347933Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5348101Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5348224Z 
2019-09-25T16:17:22.5348386Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5349381Z    |
2019-09-25T16:17:22.5349381Z    |
2019-09-25T16:17:22.5349558Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5350385Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5350593Z 
2019-09-25T16:17:22.5350744Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5351476Z    |
2019-09-25T16:17:22.5351476Z    |
2019-09-25T16:17:22.5351646Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5351798Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5351922Z 
2019-09-25T16:17:22.5352082Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5352939Z    |
2019-09-25T16:17:22.5352939Z    |
2019-09-25T16:17:22.5353111Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5353271Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5353397Z 
2019-09-25T16:17:22.5353539Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5354176Z    |
2019-09-25T16:17:22.5354176Z    |
2019-09-25T16:17:22.5354341Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5354494Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5354619Z 
2019-09-25T16:17:22.5354778Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5355388Z    |
2019-09-25T16:17:22.5355388Z    |
2019-09-25T16:17:22.5355537Z LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5355709Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5355834Z 
2019-09-25T16:17:22.5355995Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5357425Z    |
2019-09-25T16:17:22.5357425Z    |
2019-09-25T16:17:22.5357491Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5357679Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5357721Z 
2019-09-25T16:17:22.5357779Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5358778Z    |
2019-09-25T16:17:22.5358778Z    |
2019-09-25T16:17:22.5358834Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5358908Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5358954Z 
2019-09-25T16:17:22.5358996Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5359395Z    |
2019-09-25T16:17:22.5359395Z    |
2019-09-25T16:17:22.5359446Z LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5359514Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5359544Z 
2019-09-25T16:17:22.5359587Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5359915Z    |
2019-09-25T16:17:22.5359915Z    |
2019-09-25T16:17:22.5359962Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5360013Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5360058Z 
2019-09-25T16:17:22.5360102Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5360415Z    |
2019-09-25T16:17:22.5360415Z    |
2019-09-25T16:17:22.5360478Z LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5360529Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5360559Z 
2019-09-25T16:17:22.5360615Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5360916Z    |
2019-09-25T16:17:22.5360916Z    |
2019-09-25T16:17:22.5360972Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5361041Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5361071Z 
2019-09-25T16:17:22.5361114Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5361433Z    |
2019-09-25T16:17:22.5361433Z    |
2019-09-25T16:17:22.5361651Z LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5361720Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5361751Z 
2019-09-25T16:17:22.5361793Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5362154Z    |
2019-09-25T16:17:22.5362154Z    |
2019-09-25T16:17:22.5362203Z LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5362265Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5362310Z 
2019-09-25T16:17:22.5362755Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5365912Z    |
2019-09-25T16:17:22.5365912Z    |
2019-09-25T16:17:22.5366218Z LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5366272Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5366648Z 
2019-09-25T16:17:22.5366722Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5367191Z    |
2019-09-25T16:17:22.5367191Z    |
2019-09-25T16:17:22.5367255Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5367307Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5367338Z 
2019-09-25T16:17:22.5367382Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5367883Z    |
2019-09-25T16:17:22.5367883Z    |
2019-09-25T16:17:22.5367934Z LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5367995Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5368052Z 
2019-09-25T16:17:22.5368095Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5368471Z    |
2019-09-25T16:17:22.5368471Z    |
2019-09-25T16:17:22.5368521Z LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5368844Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5368878Z 
2019-09-25T16:17:22.5369307Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5370007Z    |
2019-09-25T16:17:22.5370007Z    |
2019-09-25T16:17:22.5370090Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5370142Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5370172Z 
2019-09-25T16:17:22.5370213Z error: usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5370548Z    |
2019-09-25T16:17:22.5370548Z    |
2019-09-25T16:17:22.5370606Z LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T16:17:22.5370675Z    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T16:17:22.5370748Z error: usage of `ty::TyKind`
2019-09-25T16:17:22.5371009Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:48:24
2019-09-25T16:17:22.5371076Z    |
2019-09-25T16:17:22.5371076Z    |
2019-09-25T16:17:22.5371342Z LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
2019-09-25T16:17:22.5371455Z    |
2019-09-25T16:17:22.5371507Z    = help: try using `Ty` instead
2019-09-25T16:17:22.5371537Z 
2019-09-25T16:17:22.5371596Z error: aborting due to 31 previous errors
---
2019-09-25T16:17:22.5372789Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-25T16:17:22.5372824Z 
2019-09-25T16:17:22.5372849Z 
2019-09-25T16:17:22.5372874Z 
2019-09-25T16:17:22.5374435Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-25T16:17:22.5374714Z 
2019-09-25T16:17:22.5374760Z 
2019-09-25T16:17:22.5374855Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-25T16:17:22.5374907Z Build completed unsuccessfully in 1:11:06
2019-09-25T16:17:22.5374907Z Build completed unsuccessfully in 1:11:06
2019-09-25T16:17:22.5374970Z == clock drift check ==
2019-09-25T16:17:22.5384051Z   local time: Wed Sep 25 16:17:22 UTC 2019
2019-09-25T16:17:22.8049549Z   network time: Wed, 25 Sep 2019 16:17:22 GMT
2019-09-25T16:17:22.8055648Z == end clock drift check ==
2019-09-25T16:17:23.8021954Z ##[error]Bash exited with code '1'.
2019-09-25T16:17:23.8062829Z ##[section]Starting: Checkout
2019-09-25T16:17:23.8065652Z ==============================================================================
2019-09-25T16:17:23.8065719Z Task         : Get sources
2019-09-25T16:17:23.8065772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
