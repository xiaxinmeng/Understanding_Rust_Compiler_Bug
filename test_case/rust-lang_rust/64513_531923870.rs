plain
2019-09-16T18:15:49.0042279Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T18:15:49.0226754Z ##[command]git config gc.auto 0
2019-09-16T18:15:49.0305490Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T18:15:49.0359635Z ##[command]git config --get-all http.proxy
2019-09-16T18:15:49.0494407Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64513/merge:refs/remotes/pull/64513/merge
---
2019-09-16T19:17:59.0129312Z .................................................................................................... 1500/9020
2019-09-16T19:18:04.9648910Z .................................................................................................... 1600/9020
2019-09-16T19:18:17.4539716Z .............................................................i...............i...................... 1700/9020
2019-09-16T19:18:24.5293139Z .................................................................................................... 1800/9020
2019-09-16T19:18:39.6852934Z ....................................................iiiii........................................... 1900/9020
2019-09-16T19:18:50.7060588Z .................................................................................................... 2100/9020
2019-09-16T19:18:53.2378943Z .................................................................................................... 2200/9020
2019-09-16T19:18:56.6789870Z .................................................................................................... 2300/9020
2019-09-16T19:19:05.1392966Z .................................................................................................... 2400/9020
---
2019-09-16T19:22:04.6905834Z ........................................i...............i........................................... 4700/9020
2019-09-16T19:22:15.6533670Z .................................................................................................... 4800/9020
2019-09-16T19:22:22.0977568Z .................................................................................................... 4900/9020
2019-09-16T19:22:31.6246523Z .................................................................................................... 5000/9020
2019-09-16T19:22:38.8744833Z ........................ii.ii....................................................................... 5100/9020
2019-09-16T19:22:49.0027390Z .................................................................................................... 5300/9020
2019-09-16T19:22:58.8785148Z ........................................................................................i........... 5400/9020
2019-09-16T19:23:06.8495986Z .................................................................................................... 5500/9020
2019-09-16T19:23:12.0519112Z .................................................................................................... 5600/9020
2019-09-16T19:23:12.0519112Z .................................................................................................... 5600/9020
2019-09-16T19:23:22.2319591Z ...................................................................................ii...i..ii....... 5700/9020
2019-09-16T19:23:47.5704920Z .................................................................................................... 5900/9020
2019-09-16T19:23:57.5338470Z .................................................................................................... 6000/9020
2019-09-16T19:23:57.5338470Z .................................................................................................... 6000/9020
2019-09-16T19:24:03.3957611Z .....................................................................................i..ii.......... 6100/9020
2019-09-16T19:24:33.2411268Z .................................................................................................... 6300/9020
2019-09-16T19:24:36.1881990Z ............................................i....................................................... 6400/9020
2019-09-16T19:24:38.3373750Z .................................................................................................... 6500/9020
2019-09-16T19:24:40.7901724Z ................i................................................................................... 6600/9020
---
2019-09-16T19:29:13.7862279Z  finished in 5.058
2019-09-16T19:29:13.8053213Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:29:13.9683602Z 
2019-09-16T19:29:13.9688698Z running 150 tests
2019-09-16T19:29:17.1940783Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-16T19:29:19.1888723Z ..iiii..............i.........iii.i.......ii......
2019-09-16T19:29:19.1890426Z 
2019-09-16T19:29:19.1895673Z  finished in 5.384
2019-09-16T19:29:19.2082164Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:29:19.7745906Z 
---
2019-09-16T19:29:21.4597654Z  finished in 2.251
2019-09-16T19:29:21.4787003Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:29:21.6390495Z 
2019-09-16T19:29:21.6391570Z running 9 tests
2019-09-16T19:29:21.6392731Z iiiiiiiii
2019-09-16T19:29:21.6393429Z 
2019-09-16T19:29:21.6393641Z  finished in 0.160
2019-09-16T19:29:21.6584050Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:29:21.8346821Z 
---
2019-09-16T19:29:39.9629144Z  finished in 18.304
2019-09-16T19:29:39.9811379Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:29:40.1353923Z 
2019-09-16T19:29:40.1354170Z running 123 tests
2019-09-16T19:30:03.5249792Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-16T19:30:08.7847441Z i.i.i......iii.i.....ii
2019-09-16T19:30:08.7929287Z 
2019-09-16T19:30:08.7929344Z  finished in 28.046
2019-09-16T19:30:08.7929971Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-16T19:30:08.7930315Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-16T19:31:09.3051095Z 
2019-09-16T19:31:09.3052768Z ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
2019-09-16T19:31:09.3053034Z diff of stderr:
2019-09-16T19:31:09.3053176Z 
2019-09-16T19:31:09.3053352Z 1 error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3053802Z -   --> $DIR/ty_tykind_usage.rs:11:15
2019-09-16T19:31:09.3054220Z +   --> $DIR/ty_tykind_usage.rs:11:16
2019-09-16T19:31:09.3054429Z 3    |
2019-09-16T19:31:09.3054581Z 4 LL |     let kind = TyKind::Bool;
2019-09-16T19:31:09.3054976Z -    |               ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3055197Z +    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3055536Z 7 note: lint level defined here
2019-09-16T19:31:09.3056032Z 8   --> $DIR/ty_tykind_usage.rs:9:8
2019-09-16T19:31:09.3056172Z 
2019-09-16T19:31:09.3056275Z 
2019-09-16T19:31:09.3056275Z 
2019-09-16T19:31:09.3056419Z The actual stderr differed from the expected stderr.
2019-09-16T19:31:09.3056812Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2019-09-16T19:31:09.3057527Z To update references, rerun the tests and pass the `--bless` flag
2019-09-16T19:31:09.3058452Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2019-09-16T19:31:09.3058623Z error: 1 errors occurred comparing output.
2019-09-16T19:31:09.3058659Z status: exit code: 1
2019-09-16T19:31:09.3058659Z status: exit code: 1
2019-09-16T19:31:09.3059704Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
2019-09-16T19:31:09.3060908Z ------------------------------------------
2019-09-16T19:31:09.3061092Z 
2019-09-16T19:31:09.3063253Z ------------------------------------------
2019-09-16T19:31:09.3063341Z stderr:
2019-09-16T19:31:09.3063341Z stderr:
2019-09-16T19:31:09.3063590Z ------------------------------------------
2019-09-16T19:31:09.3063641Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3064218Z    |
2019-09-16T19:31:09.3064218Z    |
2019-09-16T19:31:09.3064271Z LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3064345Z    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3064435Z note: lint level defined here
2019-09-16T19:31:09.3064746Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:9:8
2019-09-16T19:31:09.3064815Z    |
2019-09-16T19:31:09.3064815Z    |
2019-09-16T19:31:09.3064860Z LL | #[deny(rustc::usage_of_ty_tykind)]
2019-09-16T19:31:09.3064953Z 
2019-09-16T19:31:09.3064953Z 
2019-09-16T19:31:09.3065007Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3065333Z    |
2019-09-16T19:31:09.3065333Z    |
2019-09-16T19:31:09.3065403Z LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3065456Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3065498Z 
2019-09-16T19:31:09.3065722Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3066157Z    |
2019-09-16T19:31:09.3066157Z    |
2019-09-16T19:31:09.3066377Z LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3066420Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3066445Z 
2019-09-16T19:31:09.3066479Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3066911Z    |
2019-09-16T19:31:09.3066911Z    |
2019-09-16T19:31:09.3067148Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3067204Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3067226Z 
2019-09-16T19:31:09.3067256Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3067514Z    |
2019-09-16T19:31:09.3067514Z    |
2019-09-16T19:31:09.3067549Z LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3067586Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3067626Z 
2019-09-16T19:31:09.3067657Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3067891Z    |
2019-09-16T19:31:09.3067891Z    |
2019-09-16T19:31:09.3067944Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3068068Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3068101Z 
2019-09-16T19:31:09.3068151Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3068412Z    |
2019-09-16T19:31:09.3068412Z    |
2019-09-16T19:31:09.3068464Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3068511Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3068534Z 
2019-09-16T19:31:09.3068564Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3068817Z    |
2019-09-16T19:31:09.3068817Z    |
2019-09-16T19:31:09.3068851Z LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3068906Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3068929Z 
2019-09-16T19:31:09.3068961Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3069387Z    |
2019-09-16T19:31:09.3069387Z    |
2019-09-16T19:31:09.3069767Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3070482Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3070528Z 
2019-09-16T19:31:09.3070586Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3072034Z    |
2019-09-16T19:31:09.3072034Z    |
2019-09-16T19:31:09.3072280Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3072321Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3072347Z 
2019-09-16T19:31:09.3072762Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3073223Z    |
2019-09-16T19:31:09.3073223Z    |
2019-09-16T19:31:09.3073288Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3073361Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3073392Z 
2019-09-16T19:31:09.3073434Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3073775Z    |
2019-09-16T19:31:09.3073775Z    |
2019-09-16T19:31:09.3073834Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3073885Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3073934Z 
2019-09-16T19:31:09.3073977Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3074315Z    |
2019-09-16T19:31:09.3074315Z    |
2019-09-16T19:31:09.3074364Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3074415Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3074455Z 
2019-09-16T19:31:09.3074517Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3074839Z    |
2019-09-16T19:31:09.3074839Z    |
2019-09-16T19:31:09.3074905Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3074957Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3074998Z 
2019-09-16T19:31:09.3075040Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3075380Z    |
2019-09-16T19:31:09.3075380Z    |
2019-09-16T19:31:09.3075428Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3075497Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3075529Z 
2019-09-16T19:31:09.3075571Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3076269Z    |
2019-09-16T19:31:09.3076269Z    |
2019-09-16T19:31:09.3076307Z LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3076347Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3076391Z 
2019-09-16T19:31:09.3076595Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3076911Z    |
2019-09-16T19:31:09.3076911Z    |
2019-09-16T19:31:09.3076951Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3076993Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3077019Z 
2019-09-16T19:31:09.3077071Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3077507Z    |
2019-09-16T19:31:09.3077507Z    |
2019-09-16T19:31:09.3077576Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3077620Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3077645Z 
2019-09-16T19:31:09.3077702Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3078152Z    |
2019-09-16T19:31:09.3078152Z    |
2019-09-16T19:31:09.3078192Z LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3078369Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3078394Z 
2019-09-16T19:31:09.3078635Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3079170Z    |
2019-09-16T19:31:09.3079170Z    |
2019-09-16T19:31:09.3079211Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3079458Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3079484Z 
2019-09-16T19:31:09.3079527Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3079962Z    |
2019-09-16T19:31:09.3079962Z    |
2019-09-16T19:31:09.3080002Z LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3080042Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3080074Z 
2019-09-16T19:31:09.3080125Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3080377Z    |
2019-09-16T19:31:09.3080377Z    |
2019-09-16T19:31:09.3080434Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3080475Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3080499Z 
2019-09-16T19:31:09.3080551Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3080806Z    |
2019-09-16T19:31:09.3080806Z    |
2019-09-16T19:31:09.3080847Z LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3080907Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3080932Z 
2019-09-16T19:31:09.3080964Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3081245Z    |
2019-09-16T19:31:09.3081245Z    |
2019-09-16T19:31:09.3081283Z LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3081343Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3081367Z 
2019-09-16T19:31:09.3081400Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3081668Z    |
2019-09-16T19:31:09.3081668Z    |
2019-09-16T19:31:09.3081944Z LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3081992Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3082034Z 
2019-09-16T19:31:09.3082245Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3082948Z    |
2019-09-16T19:31:09.3082948Z    |
2019-09-16T19:31:09.3083014Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3083078Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3083109Z 
2019-09-16T19:31:09.3083169Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3083491Z    |
2019-09-16T19:31:09.3083491Z    |
2019-09-16T19:31:09.3083560Z LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3083612Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3083643Z 
2019-09-16T19:31:09.3083695Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3084039Z    |
2019-09-16T19:31:09.3084039Z    |
2019-09-16T19:31:09.3084086Z LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3084155Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3084306Z 
2019-09-16T19:31:09.3084349Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3084715Z    |
2019-09-16T19:31:09.3084715Z    |
2019-09-16T19:31:09.3084764Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3084815Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3084865Z 
2019-09-16T19:31:09.3084907Z error: usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3085255Z    |
2019-09-16T19:31:09.3085255Z    |
2019-09-16T19:31:09.3085306Z LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
2019-09-16T19:31:09.3085359Z    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2019-09-16T19:31:09.3085452Z error: usage of `ty::TyKind`
2019-09-16T19:31:09.3085724Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:48:24
2019-09-16T19:31:09.3085783Z    |
2019-09-16T19:31:09.3085783Z    |
2019-09-16T19:31:09.3086228Z LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
2019-09-16T19:31:09.3086496Z    |
2019-09-16T19:31:09.3086532Z    = help: try using `Ty` instead
2019-09-16T19:31:09.3086575Z 
2019-09-16T19:31:09.3086798Z error: aborting due to 31 previous errors
---
2019-09-16T19:31:09.3087745Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-16T19:31:09.3087773Z 
2019-09-16T19:31:09.3087809Z 
2019-09-16T19:31:09.3087835Z 
2019-09-16T19:31:09.3089037Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-16T19:31:09.3089262Z 
2019-09-16T19:31:09.3089283Z 
2019-09-16T19:31:09.3089519Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-16T19:31:09.3089592Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-16T19:31:09.3089592Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-16T19:31:09.3089635Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-16T19:31:09.3089671Z Build completed unsuccessfully in 1:08:13
2019-09-16T19:31:09.3089723Z == clock drift check ==
2019-09-16T19:31:09.3089757Z   local time: Mon Sep 16 19:31:08 UTC 2019
2019-09-16T19:31:09.3089872Z   network time: Mon, 16 Sep 2019 19:31:08 GMT
2019-09-16T19:31:09.3089925Z == end clock drift check ==
2019-09-16T19:31:09.3815418Z ##[error]Bash exited with code '1'.
2019-09-16T19:31:09.3850892Z ##[section]Starting: Checkout
2019-09-16T19:31:09.3853128Z ==============================================================================
2019-09-16T19:31:09.3853185Z Task         : Get sources
2019-09-16T19:31:09.3853234Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
