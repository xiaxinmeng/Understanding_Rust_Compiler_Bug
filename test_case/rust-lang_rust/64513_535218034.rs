plain
2019-09-25T19:39:27.0408332Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T19:39:27.0643234Z ##[command]git config gc.auto 0
2019-09-25T19:39:27.0722945Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T19:39:27.0790246Z ##[command]git config --get-all http.proxy
2019-09-25T19:39:27.0965696Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64513/merge:refs/remotes/pull/64513/merge
---
2019-09-25T20:43:48.4527500Z .................................................................................................... 1500/9044
2019-09-25T20:43:54.5068161Z .................................................................................................... 1600/9044
2019-09-25T20:44:07.0050541Z ..........................................................................i...............i......... 1700/9044
2019-09-25T20:44:13.8379278Z .................................................................................................... 1800/9044
2019-09-25T20:44:22.4547402Z .................................................................iiiii.............................. 1900/9044
2019-09-25T20:44:42.6987453Z .................................................................................................... 2100/9044
2019-09-25T20:44:45.3388788Z .................................................................................................... 2200/9044
2019-09-25T20:44:48.5979483Z .................................................................................................... 2300/9044
2019-09-25T20:44:57.1316644Z .................................................................................................... 2400/9044
---
2019-09-25T20:47:59.6083964Z ........................................................i...............i........................... 4700/9044
2019-09-25T20:48:08.9276971Z .................................................................................................... 4800/9044
2019-09-25T20:48:17.6953136Z .................................................................................................... 4900/9044
2019-09-25T20:48:25.4128195Z .................................................................................................... 5000/9044
2019-09-25T20:48:35.3137127Z ...........................................ii.ii.................................................... 5100/9044
2019-09-25T20:48:45.5807214Z .................................................................................................... 5300/9044
2019-09-25T20:48:56.1184899Z .................................................................................................... 5400/9044
2019-09-25T20:49:03.6603522Z ........i........................................................................................... 5500/9044
2019-09-25T20:49:09.3431753Z .................................................................................................... 5600/9044
2019-09-25T20:49:09.3431753Z .................................................................................................... 5600/9044
2019-09-25T20:49:21.3316791Z .................................................................................................... 5700/9044
2019-09-25T20:49:34.5479368Z ...ii...i..ii...........i........................................................................... 5800/9044
2019-09-25T20:49:56.3783977Z .................................................................................................... 6000/9044
2019-09-25T20:50:05.8532773Z .................................................................................................... 6100/9044
2019-09-25T20:50:05.8532773Z .................................................................................................... 6100/9044
2019-09-25T20:50:23.2173359Z .....i..ii.......................................................................................... 6200/9044
2019-09-25T20:50:43.0531020Z .................................................................i.................................. 6400/9044
2019-09-25T20:50:45.2556320Z .................................................................................................... 6500/9044
2019-09-25T20:50:47.8579793Z .....................................i.............................................................. 6600/9044
2019-09-25T20:50:52.0223676Z .................................................................................................... 6700/9044
---
2019-09-25T20:51:47.7298930Z .................................................................................................... 7200/9044
2019-09-25T20:51:53.2162078Z .................................................................................................... 7300/9044
2019-09-25T20:52:00.7162752Z .................................................................................................... 7400/9044
2019-09-25T20:52:11.0462617Z .................................................................................................... 7500/9044
2019-09-25T20:52:21.3464520Z .........................................................................................ii......i.. 7600/9044
2019-09-25T20:52:32.7475679Z .................................................................................................... 7800/9044
2019-09-25T20:52:50.0941701Z .................................................................................................... 7900/9044
2019-09-25T20:52:58.5310574Z .................................................................................................... 8000/9044
2019-09-25T20:53:08.6391008Z .................................................................................................... 8100/9044
---
2019-09-25T20:55:26.1605801Z  finished in 5.561
2019-09-25T20:55:26.1800021Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:55:26.3484424Z 
2019-09-25T20:55:26.3485160Z running 150 tests
2019-09-25T20:55:29.8063176Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-25T20:55:31.8367361Z ..iiii..............i.........iii.i.......ii......
2019-09-25T20:55:31.8368767Z 
2019-09-25T20:55:31.8374833Z  finished in 5.657
2019-09-25T20:55:31.8567182Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:55:32.0242740Z 
---
2019-09-25T20:55:34.1430482Z  finished in 2.286
2019-09-25T20:55:34.1636973Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:55:34.3295746Z 
2019-09-25T20:55:34.3297039Z running 9 tests
2019-09-25T20:55:34.3298345Z iiiiiiiii
2019-09-25T20:55:34.3299386Z 
2019-09-25T20:55:34.3299760Z  finished in 0.166
2019-09-25T20:55:34.3491267Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:55:34.5177271Z 
---
2019-09-25T20:55:52.9884933Z  finished in 18.639
2019-09-25T20:55:53.0074541Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:55:53.1786622Z 
2019-09-25T20:55:53.1786886Z running 123 tests
2019-09-25T20:56:18.1054789Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-25T20:56:23.0012024Z i.i.i......iii.i.....ii
2019-09-25T20:56:23.0013239Z 
2019-09-25T20:56:23.0017688Z  finished in 29.994
2019-09-25T20:56:23.0029876Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:56:23.0030420Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-25T20:57:25.9101411Z 
2019-09-25T20:57:25.9101872Z ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
2019-09-25T20:57:25.9101949Z diff of stderr:
2019-09-25T20:57:25.9101982Z 
2019-09-25T20:57:25.9109430Z 1 error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9111301Z -   --> $DIR/ty_tykind_usage.rs:11:15
2019-09-25T20:57:25.9113154Z +   --> $DIR/ty_tykind_usage.rs:11:16
2019-09-25T20:57:25.9113253Z 3    |
2019-09-25T20:57:25.9113380Z 4 LL |     let kind = TyKind::Bool;
2019-09-25T20:57:25.9119873Z 5    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9119984Z 
2019-09-25T20:57:25.9120024Z The actual stderr differed from the expected stderr.
2019-09-25T20:57:25.9120929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2019-09-25T20:57:25.9120929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2019-09-25T20:57:25.9121299Z To update references, rerun the tests and pass the `--bless` flag
2019-09-25T20:57:25.9121727Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2019-09-25T20:57:25.9121828Z error: 1 errors occurred comparing output.
2019-09-25T20:57:25.9121884Z status: exit code: 1
2019-09-25T20:57:25.9121884Z status: exit code: 1
2019-09-25T20:57:25.9123247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
2019-09-25T20:57:25.9123670Z ------------------------------------------
2019-09-25T20:57:25.9123722Z 
2019-09-25T20:57:25.9123956Z ------------------------------------------
2019-09-25T20:57:25.9124002Z stderr:
2019-09-25T20:57:25.9124002Z stderr:
2019-09-25T20:57:25.9124245Z ------------------------------------------
2019-09-25T20:57:25.9124306Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9124647Z    |
2019-09-25T20:57:25.9124647Z    |
2019-09-25T20:57:25.9124698Z LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9124748Z    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9124849Z note: lint level defined here
2019-09-25T20:57:25.9125117Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:9:8
2019-09-25T20:57:25.9125175Z    |
2019-09-25T20:57:25.9125175Z    |
2019-09-25T20:57:25.9125236Z LL | #[deny(rustc::usage_of_ty_tykind)]
2019-09-25T20:57:25.9209806Z 
2019-09-25T20:57:25.9209806Z 
2019-09-25T20:57:25.9209934Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9210483Z    |
2019-09-25T20:57:25.9210483Z    |
2019-09-25T20:57:25.9210881Z LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9210931Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9210964Z 
2019-09-25T20:57:25.9211564Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9212670Z    |
2019-09-25T20:57:25.9212670Z    |
2019-09-25T20:57:25.9212742Z LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9212793Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9212839Z 
2019-09-25T20:57:25.9212880Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9213279Z    |
2019-09-25T20:57:25.9213279Z    |
2019-09-25T20:57:25.9213325Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9213380Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9213420Z 
2019-09-25T20:57:25.9213471Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9213793Z    |
2019-09-25T20:57:25.9213793Z    |
2019-09-25T20:57:25.9213852Z LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9213903Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9213933Z 
2019-09-25T20:57:25.9213974Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9214474Z    |
2019-09-25T20:57:25.9214474Z    |
2019-09-25T20:57:25.9214520Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9214584Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9214615Z 
2019-09-25T20:57:25.9214656Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9216788Z    |
2019-09-25T20:57:25.9216788Z    |
2019-09-25T20:57:25.9216832Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9218842Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9218907Z 
2019-09-25T20:57:25.9218949Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9219419Z    |
2019-09-25T20:57:25.9219419Z    |
2019-09-25T20:57:25.9219485Z LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9219537Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9219567Z 
2019-09-25T20:57:25.9219618Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9219943Z    |
2019-09-25T20:57:25.9219943Z    |
2019-09-25T20:57:25.9220002Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9220061Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9220094Z 
2019-09-25T20:57:25.9220134Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9220466Z    |
2019-09-25T20:57:25.9220466Z    |
2019-09-25T20:57:25.9220513Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9220576Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9220606Z 
2019-09-25T20:57:25.9220655Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9221092Z    |
2019-09-25T20:57:25.9221092Z    |
2019-09-25T20:57:25.9221136Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9221309Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9229620Z 
2019-09-25T20:57:25.9229683Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9230520Z    |
2019-09-25T20:57:25.9230520Z    |
2019-09-25T20:57:25.9230566Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9230614Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9230644Z 
2019-09-25T20:57:25.9230694Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9231032Z    |
2019-09-25T20:57:25.9231032Z    |
2019-09-25T20:57:25.9231089Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9231136Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9231164Z 
2019-09-25T20:57:25.9231211Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9231525Z    |
2019-09-25T20:57:25.9231525Z    |
2019-09-25T20:57:25.9231568Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9231627Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9231656Z 
2019-09-25T20:57:25.9231694Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9232275Z    |
2019-09-25T20:57:25.9232275Z    |
2019-09-25T20:57:25.9232319Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9233037Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9233106Z 
2019-09-25T20:57:25.9233151Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9233594Z    |
2019-09-25T20:57:25.9233594Z    |
2019-09-25T20:57:25.9233642Z LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9233705Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9233735Z 
2019-09-25T20:57:25.9233786Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9234106Z    |
2019-09-25T20:57:25.9234106Z    |
2019-09-25T20:57:25.9234163Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9234214Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9234244Z 
2019-09-25T20:57:25.9234294Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9234621Z    |
2019-09-25T20:57:25.9234621Z    |
2019-09-25T20:57:25.9234668Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9234733Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9234763Z 
2019-09-25T20:57:25.9234810Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9235144Z    |
2019-09-25T20:57:25.9235144Z    |
2019-09-25T20:57:25.9235193Z LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9235256Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9235286Z 
2019-09-25T20:57:25.9235326Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9235651Z    |
2019-09-25T20:57:25.9235651Z    |
2019-09-25T20:57:25.9235705Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9235756Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9235797Z 
2019-09-25T20:57:25.9235837Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9236492Z    |
2019-09-25T20:57:25.9236492Z    |
2019-09-25T20:57:25.9236550Z LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9236593Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9236619Z 
2019-09-25T20:57:25.9236665Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9236975Z    |
2019-09-25T20:57:25.9236975Z    |
2019-09-25T20:57:25.9237027Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9237081Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9237108Z 
2019-09-25T20:57:25.9237145Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9237468Z    |
2019-09-25T20:57:25.9237468Z    |
2019-09-25T20:57:25.9237511Z LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9237572Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9237599Z 
2019-09-25T20:57:25.9237634Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9237919Z    |
2019-09-25T20:57:25.9237919Z    |
2019-09-25T20:57:25.9237960Z LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9238003Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9238041Z 
2019-09-25T20:57:25.9238075Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9238458Z    |
2019-09-25T20:57:25.9238458Z    |
2019-09-25T20:57:25.9238500Z LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9238542Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9238569Z 
2019-09-25T20:57:25.9238617Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9239049Z    |
2019-09-25T20:57:25.9239049Z    |
2019-09-25T20:57:25.9239101Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9239148Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9239176Z 
2019-09-25T20:57:25.9239213Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9239518Z    |
2019-09-25T20:57:25.9239518Z    |
2019-09-25T20:57:25.9239571Z LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9239629Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9239657Z 
2019-09-25T20:57:25.9239695Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9240009Z    |
2019-09-25T20:57:25.9240009Z    |
2019-09-25T20:57:25.9240052Z LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9240098Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9240138Z 
2019-09-25T20:57:25.9240175Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9240486Z    |
2019-09-25T20:57:25.9240486Z    |
2019-09-25T20:57:25.9240528Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9240584Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9240611Z 
2019-09-25T20:57:25.9240661Z error: usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9241085Z    |
2019-09-25T20:57:25.9241085Z    |
2019-09-25T20:57:25.9241140Z LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-25T20:57:25.9241191Z    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-25T20:57:25.9241356Z error: usage of `ty::TyKind`
2019-09-25T20:57:25.9241651Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:48:24
2019-09-25T20:57:25.9241699Z    |
2019-09-25T20:57:25.9241699Z    |
2019-09-25T20:57:25.9242127Z LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
2019-09-25T20:57:25.9242330Z    |
2019-09-25T20:57:25.9242370Z    = help: try using `Ty` instead
2019-09-25T20:57:25.9242412Z 
2019-09-25T20:57:25.9242577Z error: aborting due to 31 previous errors
---
2019-09-25T20:57:25.9244437Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-25T20:57:25.9244473Z 
2019-09-25T20:57:25.9262146Z 
2019-09-25T20:57:25.9262255Z 
2019-09-25T20:57:25.9264691Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-25T20:57:25.9266893Z 
2019-09-25T20:57:25.9266925Z 
2019-09-25T20:57:25.9266972Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-25T20:57:25.9267031Z Build completed unsuccessfully in 1:10:32
2019-09-25T20:57:25.9267031Z Build completed unsuccessfully in 1:10:32
2019-09-25T20:57:25.9267092Z == clock drift check ==
2019-09-25T20:57:25.9267137Z   local time: Wed Sep 25 20:57:25 UTC 2019
2019-09-25T20:57:26.0744225Z   network time: Wed, 25 Sep 2019 20:57:26 GMT
2019-09-25T20:57:26.0744909Z == end clock drift check ==
2019-09-25T20:57:27.0782286Z ##[error]Bash exited with code '1'.
2019-09-25T20:57:27.0831682Z ##[section]Starting: Checkout
2019-09-25T20:57:27.0833698Z ==============================================================================
2019-09-25T20:57:27.0833749Z Task         : Get sources
2019-09-25T20:57:27.0833793Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
