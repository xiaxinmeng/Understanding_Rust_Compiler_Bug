plain
2020-01-10T14:17:32.0959573Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T14:17:32.0970244Z ##[command]git config gc.auto 0
2020-01-10T14:17:32.0972523Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T14:17:32.0975078Z ##[command]git config --get-all http.proxy
2020-01-10T14:17:32.0977659Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68080/merge:refs/remotes/pull/68080/merge
---
2020-01-10T15:17:08.3155786Z .............................i...............i...................................................... 4900/9503
2020-01-10T15:17:18.4769369Z .................................................................................................... 5000/9503
2020-01-10T15:17:26.0634149Z ..........................................................................i......................... 5100/9503
2020-01-10T15:17:32.6455512Z .................................................................................................... 5200/9503
2020-01-10T15:17:42.3788775Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T15:17:52.1351486Z .................................................................................................... 5500/9503
2020-01-10T15:18:01.7886048Z .................................................................................................... 5600/9503
2020-01-10T15:18:09.0409039Z .........................i.......................................................................... 5700/9503
2020-01-10T15:18:15.3419101Z .................................................................................................... 5800/9503
2020-01-10T15:18:15.3419101Z .................................................................................................... 5800/9503
2020-01-10T15:18:27.1943676Z .................................................................................................... 5900/9503
2020-01-10T15:18:38.3873666Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T15:18:56.8349922Z .................................................................................................... 6200/9503
2020-01-10T15:19:05.3546265Z .................................................................................................... 6300/9503
2020-01-10T15:19:05.3546265Z .................................................................................................... 6300/9503
2020-01-10T15:19:24.6208102Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T15:19:47.6204211Z .................................................................................................... 6600/9503
2020-01-10T15:19:49.7400951Z ..................i................................................................................. 6700/9503
2020-01-10T15:19:52.0298194Z .................................................................................................... 6800/9503
2020-01-10T15:19:54.5924070Z ..................i................................................................................. 6900/9503
---
2020-01-10T15:21:35.1493823Z .................................................................................................... 7500/9503
2020-01-10T15:21:39.0575239Z .................................................................................................... 7600/9503
2020-01-10T15:21:45.4528562Z .................................................................................................... 7700/9503
2020-01-10T15:21:54.6134081Z .................................................................................................... 7800/9503
2020-01-10T15:22:05.6353874Z .....................................................................iiii........................... 7900/9503
2020-01-10T15:22:21.7603718Z ...i......i......................................................................................... 8100/9503
2020-01-10T15:22:27.3286859Z .................................................................................................... 8200/9503
2020-01-10T15:22:42.9529104Z .................................................................................................... 8300/9503
2020-01-10T15:22:51.9138098Z .................................................................................................... 8400/9503
---
2020-01-10T15:25:20.7094598Z  finished in 7.610
2020-01-10T15:25:20.7292981Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:25:20.8985826Z 
2020-01-10T15:25:20.8986786Z running 166 tests
2020-01-10T15:25:24.1210656Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T15:25:26.5051526Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T15:25:26.5053542Z 
2020-01-10T15:25:26.5054375Z  finished in 5.776
2020-01-10T15:25:26.5298422Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:25:26.7000562Z 
---
2020-01-10T15:25:28.7761334Z  finished in 2.246
2020-01-10T15:25:28.7965810Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:25:28.9839719Z 
2020-01-10T15:25:28.9840762Z running 9 tests
2020-01-10T15:25:28.9841925Z iiiiiiiii
2020-01-10T15:25:28.9842902Z 
2020-01-10T15:25:28.9843095Z  finished in 0.187
2020-01-10T15:25:29.0058010Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:25:29.1790213Z 
---
2020-01-10T15:25:49.9658131Z  finished in 20.951
2020-01-10T15:25:49.9763711Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:25:50.1403079Z 
2020-01-10T15:25:50.1405001Z running 124 tests
2020-01-10T15:26:14.9702813Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T15:26:19.1289303Z .i.iii.....iiiiii.....ii
2020-01-10T15:26:19.1290798Z 
2020-01-10T15:26:19.1294068Z  finished in 29.153
2020-01-10T15:26:19.1299578Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:26:19.1300672Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-10T15:26:19.1300672Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-10T15:26:19.1528596Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T15:26:19.3141646Z 
2020-01-10T15:26:19.3143171Z running 63 tests
2020-01-10T15:27:00.4797978Z ..............F.FFFF....................F.F...F................
2020-01-10T15:27:00.4833181Z 
2020-01-10T15:27:00.4833967Z ---- [ui] ui-fulldeps/internal-lints/default_hash_types.rs stdout ----
2020-01-10T15:27:00.4834224Z diff of stderr:
2020-01-10T15:27:00.4834374Z 
---
2020-01-10T15:27:00.4836731Z 
2020-01-10T15:27:00.4836842Z 
2020-01-10T15:27:00.4836975Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4837673Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/default_hash_types.stderr
2020-01-10T15:27:00.4838187Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4838813Z To only update this specific test, also pass `--test-args internal-lints/default_hash_types.rs`
2020-01-10T15:27:00.4839132Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4839262Z status: exit code: 1
2020-01-10T15:27:00.4839262Z status: exit code: 1
2020-01-10T15:27:00.4840392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4841004Z ------------------------------------------
2020-01-10T15:27:00.4841160Z 
2020-01-10T15:27:00.4841527Z ------------------------------------------
2020-01-10T15:27:00.4841812Z stderr:
2020-01-10T15:27:00.4841812Z stderr:
2020-01-10T15:27:00.4842128Z ------------------------------------------
2020-01-10T15:27:00.4842320Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-10T15:27:00.4842883Z    |
2020-01-10T15:27:00.4843021Z LL |     let _map: HashMap<String, String> = HashMap::default();
2020-01-10T15:27:00.4843151Z    |               ^^^^^^^ help: use: `FxHashMap`
2020-01-10T15:27:00.4843288Z    |
2020-01-10T15:27:00.4843288Z    |
2020-01-10T15:27:00.4843426Z note: the lint level is defined here
2020-01-10T15:27:00.4843797Z   --> /checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs:10:8
2020-01-10T15:27:00.4843992Z    |
2020-01-10T15:27:00.4844120Z LL | #[deny(rustc::default_hash_types)]
2020-01-10T15:27:00.4844247Z    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-10T15:27:00.4844397Z    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
2020-01-10T15:27:00.4844507Z 
2020-01-10T15:27:00.4844633Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-10T15:27:00.4845193Z    |
2020-01-10T15:27:00.4845320Z LL |     let _map: HashMap<String, String> = HashMap::default();
2020-01-10T15:27:00.4846028Z    |                                         ^^^^^^^ help: use: `FxHashMap`
2020-01-10T15:27:00.4846171Z    |
2020-01-10T15:27:00.4846171Z    |
2020-01-10T15:27:00.4846339Z    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
2020-01-10T15:27:00.4846454Z 
2020-01-10T15:27:00.4846602Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-10T15:27:00.4847340Z    |
2020-01-10T15:27:00.4847478Z LL |     let _set: HashSet<String> = HashSet::default();
2020-01-10T15:27:00.4847627Z    |               ^^^^^^^ help: use: `FxHashSet`
2020-01-10T15:27:00.4847783Z    |
2020-01-10T15:27:00.4847783Z    |
2020-01-10T15:27:00.4847948Z    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
2020-01-10T15:27:00.4848077Z 
2020-01-10T15:27:00.4848219Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-10T15:27:00.4848855Z    |
2020-01-10T15:27:00.4849015Z LL |     let _set: HashSet<String> = HashSet::default();
2020-01-10T15:27:00.4849162Z    |                                 ^^^^^^^ help: use: `FxHashSet`
2020-01-10T15:27:00.4849316Z    |
---
2020-01-10T15:27:00.4853169Z - note: lint level defined here
2020-01-10T15:27:00.4853337Z + note: the lint level is defined here
2020-01-10T15:27:00.4853700Z 8   --> $DIR/pass_ty_by_ref.rs:4:9
2020-01-10T15:27:00.4853885Z 9    |
2020-01-10T15:27:00.4854026Z 10 LL | #![deny(rustc::ty_pass_by_reference)]
2020-01-10T15:27:00.4854247Z 
2020-01-10T15:27:00.4854374Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4855124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
2020-01-10T15:27:00.4855124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
2020-01-10T15:27:00.4856136Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4856678Z To only update this specific test, also pass `--test-args internal-lints/pass_ty_by_ref.rs`
2020-01-10T15:27:00.4857112Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4857257Z status: exit code: 1
2020-01-10T15:27:00.4857257Z status: exit code: 1
2020-01-10T15:27:00.4858420Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4859148Z ------------------------------------------
2020-01-10T15:27:00.4859298Z 
2020-01-10T15:27:00.4859643Z ------------------------------------------
2020-01-10T15:27:00.4859802Z stderr:
2020-01-10T15:27:00.4859802Z stderr:
2020-01-10T15:27:00.4860120Z ------------------------------------------
2020-01-10T15:27:00.4860533Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4860947Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:13:13
2020-01-10T15:27:00.4861119Z    |
2020-01-10T15:27:00.4861495Z LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-10T15:27:00.4861916Z    |             ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4862230Z note: the lint level is defined here
2020-01-10T15:27:00.4862608Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
2020-01-10T15:27:00.4862780Z    |
2020-01-10T15:27:00.4862780Z    |
2020-01-10T15:27:00.4862927Z LL | #![deny(rustc::ty_pass_by_reference)]
2020-01-10T15:27:00.4863161Z 
2020-01-10T15:27:00.4863504Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4863912Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:15:18
2020-01-10T15:27:00.4864085Z    |
2020-01-10T15:27:00.4864085Z    |
2020-01-10T15:27:00.4864465Z LL |     ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4864987Z    |                  ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4866007Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4866597Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:28
2020-01-10T15:27:00.4895288Z    |
2020-01-10T15:27:00.4895288Z    |
2020-01-10T15:27:00.4899075Z LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-10T15:27:00.4903517Z    |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4903880Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4904171Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:55
2020-01-10T15:27:00.4904224Z    |
2020-01-10T15:27:00.4904224Z    |
2020-01-10T15:27:00.4904514Z LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-10T15:27:00.4904830Z    |                                                       ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4905119Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4905801Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:26:17
2020-01-10T15:27:00.4905864Z    |
2020-01-10T15:27:00.4905864Z    |
2020-01-10T15:27:00.4906197Z LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-10T15:27:00.4906456Z    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4906702Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4906973Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:28:22
2020-01-10T15:27:00.4907021Z    |
2020-01-10T15:27:00.4907021Z    |
2020-01-10T15:27:00.4907272Z LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4907539Z    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4907778Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4908025Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:41
2020-01-10T15:27:00.4908088Z    |
2020-01-10T15:27:00.4908088Z    |
2020-01-10T15:27:00.4908352Z LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
2020-01-10T15:27:00.4909334Z    |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4910148Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4910389Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:68
2020-01-10T15:27:00.4910452Z    |
2020-01-10T15:27:00.4910452Z    |
2020-01-10T15:27:00.4910696Z LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
2020-01-10T15:27:00.4910988Z    |                                                                    ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4911854Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4912113Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:53:17
2020-01-10T15:27:00.4912161Z    |
2020-01-10T15:27:00.4912161Z    |
2020-01-10T15:27:00.4913028Z LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-10T15:27:00.4913496Z    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4913771Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4914025Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:55:22
2020-01-10T15:27:00.4914071Z    |
2020-01-10T15:27:00.4914071Z    |
2020-01-10T15:27:00.4914318Z LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4914587Z    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4914823Z error: passing `Ty<'_>` by reference
2020-01-10T15:27:00.4916340Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:38
2020-01-10T15:27:00.4916402Z    |
2020-01-10T15:27:00.4916402Z    |
2020-01-10T15:27:00.4916718Z LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-10T15:27:00.4916991Z    |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-10T15:27:00.4917493Z error: passing `TyCtxt<'_>` by reference
2020-01-10T15:27:00.4919092Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:65
2020-01-10T15:27:00.4919172Z    |
2020-01-10T15:27:00.4919172Z    |
2020-01-10T15:27:00.4919432Z LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-10T15:27:00.4919730Z    |                                                                 ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-10T15:27:00.4919828Z error: aborting due to 12 previous errors
2020-01-10T15:27:00.4919856Z 
2020-01-10T15:27:00.4919881Z 
2020-01-10T15:27:00.4920113Z ------------------------------------------
---
2020-01-10T15:27:00.4921287Z 
2020-01-10T15:27:00.4921312Z 
2020-01-10T15:27:00.4921371Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4921722Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2020-01-10T15:27:00.4922112Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4922418Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2020-01-10T15:27:00.4922508Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4922568Z status: exit code: 1
2020-01-10T15:27:00.4922568Z status: exit code: 1
2020-01-10T15:27:00.4923535Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4923899Z ------------------------------------------
2020-01-10T15:27:00.4923949Z 
2020-01-10T15:27:00.4924179Z ------------------------------------------
2020-01-10T15:27:00.4924233Z stderr:
2020-01-10T15:27:00.4924233Z stderr:
2020-01-10T15:27:00.4924457Z ------------------------------------------
2020-01-10T15:27:00.4924522Z error: implementing `LintPass` by hand
2020-01-10T15:27:00.4924797Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:21:6
2020-01-10T15:27:00.4924848Z    |
2020-01-10T15:27:00.4924915Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2020-01-10T15:27:00.4925003Z    |
2020-01-10T15:27:00.4925062Z note: the lint level is defined here
2020-01-10T15:27:00.4925336Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2020-01-10T15:27:00.4925830Z    |
2020-01-10T15:27:00.4925830Z    |
2020-01-10T15:27:00.4925894Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2020-01-10T15:27:00.4925944Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-10T15:27:00.4926126Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-10T15:27:00.4926283Z error: implementing `LintPass` by hand
2020-01-10T15:27:00.4926646Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:31:14
2020-01-10T15:27:00.4926695Z    |
2020-01-10T15:27:00.4926695Z    |
2020-01-10T15:27:00.4926763Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2020-01-10T15:27:00.4926851Z ...
2020-01-10T15:27:00.4926851Z ...
2020-01-10T15:27:00.4926908Z LL | custom_lint_pass_macro!();
2020-01-10T15:27:00.4927188Z    |
2020-01-10T15:27:00.4927188Z    |
2020-01-10T15:27:00.4927250Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-10T15:27:00.4927325Z error: aborting due to 2 previous errors
2020-01-10T15:27:00.4927353Z 
2020-01-10T15:27:00.4927378Z 
2020-01-10T15:27:00.4927620Z ------------------------------------------
2020-01-10T15:27:00.4927620Z ------------------------------------------
2020-01-10T15:27:00.4927652Z 
2020-01-10T15:27:00.4927678Z 
2020-01-10T15:27:00.4927917Z ---- [ui] ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs stdout ----
2020-01-10T15:27:00.4927990Z diff of stderr:
2020-01-10T15:27:00.4928017Z 
2020-01-10T15:27:00.4928220Z 4 LL |     ty_q: ty::Ty<'_>,
2020-01-10T15:27:00.4928462Z 5    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
2020-01-10T15:27:00.4928727Z - note: lint level defined here
2020-01-10T15:27:00.4928773Z + note: the lint level is defined here
2020-01-10T15:27:00.4929128Z 8   --> $DIR/qualified_ty_ty_ctxt.rs:4:9
2020-01-10T15:27:00.4929170Z 9    |
2020-01-10T15:27:00.4929170Z 9    |
2020-01-10T15:27:00.4929212Z 10 LL | #![deny(rustc::usage_of_qualified_ty)]
2020-01-10T15:27:00.4929279Z 
2020-01-10T15:27:00.4929321Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4929321Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4929648Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/qualified_ty_ty_ctxt.stderr
2020-01-10T15:27:00.4929914Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4930245Z To only update this specific test, also pass `--test-args internal-lints/qualified_ty_ty_ctxt.rs`
2020-01-10T15:27:00.4930337Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4930378Z status: exit code: 1
2020-01-10T15:27:00.4930378Z status: exit code: 1
2020-01-10T15:27:00.4931337Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4931702Z ------------------------------------------
2020-01-10T15:27:00.4931735Z 
2020-01-10T15:27:00.4931940Z ------------------------------------------
2020-01-10T15:27:00.4931981Z stderr:
2020-01-10T15:27:00.4931981Z stderr:
2020-01-10T15:27:00.4932199Z ------------------------------------------
2020-01-10T15:27:00.4932398Z error: usage of qualified `ty::Ty<'_>`
2020-01-10T15:27:00.4932643Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:25:11
2020-01-10T15:27:00.4932705Z    |
2020-01-10T15:27:00.4932935Z LL |     ty_q: ty::Ty<'_>, //~ ERROR usage of qualified `ty::Ty<'_>`
2020-01-10T15:27:00.4933163Z    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
2020-01-10T15:27:00.4933355Z note: the lint level is defined here
2020-01-10T15:27:00.4933639Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:4:9
2020-01-10T15:27:00.4933782Z    |
2020-01-10T15:27:00.4933782Z    |
2020-01-10T15:27:00.4933824Z LL | #![deny(rustc::usage_of_qualified_ty)]
2020-01-10T15:27:00.4933896Z 
2020-01-10T15:27:00.4934145Z error: usage of qualified `ty::TyCtxt<'_>`
2020-01-10T15:27:00.4934393Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:27:16
2020-01-10T15:27:00.4934438Z    |
2020-01-10T15:27:00.4934438Z    |
2020-01-10T15:27:00.4934688Z LL |     ty_ctxt_q: ty::TyCtxt<'_>, //~ ERROR usage of qualified `ty::TyCtxt<'_>`
2020-01-10T15:27:00.4936332Z    |                ^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
2020-01-10T15:27:00.4936435Z error: aborting due to 2 previous errors
2020-01-10T15:27:00.4936485Z 
2020-01-10T15:27:00.4936512Z 
2020-01-10T15:27:00.4936755Z ------------------------------------------
2020-01-10T15:27:00.4936755Z ------------------------------------------
2020-01-10T15:27:00.4936803Z 
2020-01-10T15:27:00.4936830Z 
2020-01-10T15:27:00.4937107Z ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
2020-01-10T15:27:00.4937167Z diff of stderr:
2020-01-10T15:27:00.4937196Z 
2020-01-10T15:27:00.4937900Z 4 LL |     let kind = TyKind::Bool;
2020-01-10T15:27:00.4937970Z 5    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4938313Z - note: lint level defined here
2020-01-10T15:27:00.4938383Z + note: the lint level is defined here
2020-01-10T15:27:00.4938597Z 8   --> $DIR/ty_tykind_usage.rs:9:8
2020-01-10T15:27:00.4938640Z 9    |
2020-01-10T15:27:00.4938640Z 9    |
2020-01-10T15:27:00.4938697Z 10 LL | #[deny(rustc::usage_of_ty_tykind)]
2020-01-10T15:27:00.4938753Z 
2020-01-10T15:27:00.4938799Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4939137Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2020-01-10T15:27:00.4939137Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2020-01-10T15:27:00.4939400Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4939680Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2020-01-10T15:27:00.4939774Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4939817Z status: exit code: 1
2020-01-10T15:27:00.4939817Z status: exit code: 1
2020-01-10T15:27:00.4940760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4941105Z ------------------------------------------
2020-01-10T15:27:00.4941137Z 
2020-01-10T15:27:00.4941348Z ------------------------------------------
2020-01-10T15:27:00.4941408Z stderr:
2020-01-10T15:27:00.4941408Z stderr:
2020-01-10T15:27:00.4941617Z ------------------------------------------
2020-01-10T15:27:00.4941662Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4941923Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:11:16
2020-01-10T15:27:00.4941972Z    |
2020-01-10T15:27:00.4942019Z LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4942085Z    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4942169Z note: the lint level is defined here
2020-01-10T15:27:00.4942566Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:9:8
2020-01-10T15:27:00.4942627Z    |
2020-01-10T15:27:00.4942627Z    |
2020-01-10T15:27:00.4942668Z LL | #[deny(rustc::usage_of_ty_tykind)]
2020-01-10T15:27:00.4942827Z 
2020-01-10T15:27:00.4942868Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4943163Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:14:9
2020-01-10T15:27:00.4943229Z    |
2020-01-10T15:27:00.4943229Z    |
2020-01-10T15:27:00.4943275Z LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4943325Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4943410Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4943663Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:15:9
2020-01-10T15:27:00.4943710Z    |
2020-01-10T15:27:00.4943710Z    |
2020-01-10T15:27:00.4943771Z LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4943831Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4943902Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4944176Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:16:9
2020-01-10T15:27:00.4944222Z    |
2020-01-10T15:27:00.4944222Z    |
2020-01-10T15:27:00.4944268Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4944338Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4944408Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4944657Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:17:9
2020-01-10T15:27:00.4944720Z    |
2020-01-10T15:27:00.4944720Z    |
2020-01-10T15:27:00.4944767Z LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4944817Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4944902Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4945164Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:18:9
2020-01-10T15:27:00.4945228Z    |
2020-01-10T15:27:00.4945228Z    |
2020-01-10T15:27:00.4945281Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4945331Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4945769Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4946088Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:19:9
2020-01-10T15:27:00.4946135Z    |
2020-01-10T15:27:00.4946135Z    |
2020-01-10T15:27:00.4946198Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4946247Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4946317Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4946582Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:20:9
2020-01-10T15:27:00.4946627Z    |
2020-01-10T15:27:00.4946627Z    |
2020-01-10T15:27:00.4946686Z LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4946752Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4946830Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4947099Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:21:9
2020-01-10T15:27:00.4947147Z    |
2020-01-10T15:27:00.4947147Z    |
2020-01-10T15:27:00.4947193Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4947242Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4947326Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4947577Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:22:9
2020-01-10T15:27:00.4947641Z    |
2020-01-10T15:27:00.4947641Z    |
2020-01-10T15:27:00.4947688Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4947738Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4947951Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4948234Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:23:9
2020-01-10T15:27:00.4948360Z    |
2020-01-10T15:27:00.4948360Z    |
2020-01-10T15:27:00.4948425Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4948476Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4948547Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4948844Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:24:9
2020-01-10T15:27:00.4948892Z    |
2020-01-10T15:27:00.4948892Z    |
2020-01-10T15:27:00.4948938Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4949003Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4949073Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4950689Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:25:9
2020-01-10T15:27:00.4950754Z    |
2020-01-10T15:27:00.4950754Z    |
2020-01-10T15:27:00.4950801Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4950874Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4950945Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4951220Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:26:9
2020-01-10T15:27:00.4951282Z    |
2020-01-10T15:27:00.4951282Z    |
2020-01-10T15:27:00.4951329Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4951377Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4951462Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4951717Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:27:9
2020-01-10T15:27:00.4951764Z    |
2020-01-10T15:27:00.4951764Z    |
2020-01-10T15:27:00.4951834Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4951883Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4951973Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4952226Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:28:9
2020-01-10T15:27:00.4952272Z    |
2020-01-10T15:27:00.4952272Z    |
2020-01-10T15:27:00.4952318Z LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4952606Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4952696Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4953023Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:29:9
2020-01-10T15:27:00.4953069Z    |
2020-01-10T15:27:00.4953069Z    |
2020-01-10T15:27:00.4953115Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4953180Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4954050Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4954917Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:30:9
2020-01-10T15:27:00.4955163Z    |
2020-01-10T15:27:00.4955163Z    |
2020-01-10T15:27:00.4955214Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4955265Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4955678Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4956049Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:31:9
2020-01-10T15:27:00.4956096Z    |
2020-01-10T15:27:00.4956096Z    |
2020-01-10T15:27:00.4956162Z LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4956213Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4956298Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4956704Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:32:9
2020-01-10T15:27:00.4956765Z    |
2020-01-10T15:27:00.4956765Z    |
2020-01-10T15:27:00.4956810Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4956947Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4957018Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4958295Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:33:9
2020-01-10T15:27:00.4958352Z    |
2020-01-10T15:27:00.4958352Z    |
2020-01-10T15:27:00.4958400Z LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4958465Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4958537Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4958796Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:34:9
2020-01-10T15:27:00.4958858Z    |
2020-01-10T15:27:00.4958858Z    |
2020-01-10T15:27:00.4958905Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4958968Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4959062Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4959314Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:35:9
2020-01-10T15:27:00.4959360Z    |
2020-01-10T15:27:00.4959360Z    |
2020-01-10T15:27:00.4959425Z LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4959477Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4959562Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4959815Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:36:9
2020-01-10T15:27:00.4959862Z    |
2020-01-10T15:27:00.4959862Z    |
2020-01-10T15:27:00.4959924Z LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4959974Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4960051Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4960313Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:37:9
2020-01-10T15:27:00.4960367Z    |
2020-01-10T15:27:00.4960367Z    |
2020-01-10T15:27:00.4960439Z LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4960489Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4960576Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4960831Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:38:9
2020-01-10T15:27:00.4960877Z    |
2020-01-10T15:27:00.4960877Z    |
2020-01-10T15:27:00.4960923Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4960991Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4961062Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4961331Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:39:9
2020-01-10T15:27:00.4961385Z    |
2020-01-10T15:27:00.4961385Z    |
2020-01-10T15:27:00.4961433Z LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4961507Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4961577Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4961829Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:40:9
2020-01-10T15:27:00.4961893Z    |
2020-01-10T15:27:00.4961893Z    |
2020-01-10T15:27:00.4961939Z LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4961988Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4962073Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4962325Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:41:9
2020-01-10T15:27:00.4962371Z    |
2020-01-10T15:27:00.4962371Z    |
2020-01-10T15:27:00.4962435Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4962591Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4962746Z error: usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4963040Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:46:12
2020-01-10T15:27:00.4963087Z    |
2020-01-10T15:27:00.4963087Z    |
2020-01-10T15:27:00.4963134Z LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-10T15:27:00.4963205Z    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-10T15:27:00.4963276Z error: usage of `ty::TyKind`
2020-01-10T15:27:00.4963548Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:48:24
2020-01-10T15:27:00.4963595Z    |
2020-01-10T15:27:00.4963595Z    |
2020-01-10T15:27:00.4963853Z LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
2020-01-10T15:27:00.4963966Z    |
2020-01-10T15:27:00.4964008Z    = help: try using `Ty` instead
2020-01-10T15:27:00.4964044Z 
2020-01-10T15:27:00.4964102Z error: aborting due to 31 previous errors
---
2020-01-10T15:27:00.4964453Z 
2020-01-10T15:27:00.4964683Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-01-10T15:27:00.4964730Z diff of stderr:
2020-01-10T15:27:00.4964757Z 
2020-01-10T15:27:00.4964814Z 12 LL | fn lintme() { }
2020-01-10T15:27:00.4964896Z 14    |
2020-01-10T15:27:00.4965115Z - note: lint level defined here
2020-01-10T15:27:00.4965162Z + note: the lint level is defined here
2020-01-10T15:27:00.4965709Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-10T15:27:00.4965709Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-10T15:27:00.4965788Z 17    |
2020-01-10T15:27:00.4965830Z 18 LL | #![deny(test_lint)]
2020-01-10T15:27:00.4965859Z 
2020-01-10T15:27:00.4965884Z 
2020-01-10T15:27:00.4965939Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4966333Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/lint-plugin-deny-attr.stderr
2020-01-10T15:27:00.4966596Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4966874Z To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`
2020-01-10T15:27:00.4966952Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4966995Z status: exit code: 1
2020-01-10T15:27:00.4966995Z status: exit code: 1
2020-01-10T15:27:00.4967890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4968230Z ------------------------------------------
2020-01-10T15:27:00.4968263Z 
2020-01-10T15:27:00.4968472Z ------------------------------------------
2020-01-10T15:27:00.4968534Z stderr:
2020-01-10T15:27:00.4968534Z stderr:
2020-01-10T15:27:00.4968746Z ------------------------------------------
2020-01-10T15:27:00.4969189Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T15:27:00.4969535Z    |
2020-01-10T15:27:00.4969600Z LL | #![plugin(lint_plugin_test)]
2020-01-10T15:27:00.4969653Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-10T15:27:00.4969701Z    |
2020-01-10T15:27:00.4969701Z    |
2020-01-10T15:27:00.4969890Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T15:27:00.4969932Z 
2020-01-10T15:27:00.4970259Z error: item is named 'lintme'
2020-01-10T15:27:00.4970588Z    |
2020-01-10T15:27:00.4970588Z    |
2020-01-10T15:27:00.4970836Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T15:27:00.4970945Z    |
2020-01-10T15:27:00.4970990Z note: the lint level is defined here
2020-01-10T15:27:00.4971252Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
2020-01-10T15:27:00.4971300Z    |
---
2020-01-10T15:27:00.4985841Z 
2020-01-10T15:27:00.4986175Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-01-10T15:27:00.4986236Z diff of stderr:
2020-01-10T15:27:00.4986263Z 
2020-01-10T15:27:00.4986323Z 30 LL | fn lintme() { }
2020-01-10T15:27:00.4987713Z 32    |
2020-01-10T15:27:00.4991816Z - note: lint level defined here
2020-01-10T15:27:00.4991888Z + note: the lint level is defined here
2020-01-10T15:27:00.4992148Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T15:27:00.4992148Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T15:27:00.4992195Z 35    |
2020-01-10T15:27:00.4992256Z 36 LL | #![forbid(test_lint)]
2020-01-10T15:27:00.4992311Z 
2020-01-10T15:27:00.4992355Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.4992708Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-10T15:27:00.4992708Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-10T15:27:00.4992979Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.4993267Z To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
2020-01-10T15:27:00.4993356Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.4993399Z status: exit code: 1
2020-01-10T15:27:00.4993399Z status: exit code: 1
2020-01-10T15:27:00.4994313Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2020-01-10T15:27:00.4994649Z ------------------------------------------
2020-01-10T15:27:00.4995150Z 
2020-01-10T15:27:00.4995935Z ------------------------------------------
2020-01-10T15:27:00.4996016Z stderr:
2020-01-10T15:27:00.4996016Z stderr:
2020-01-10T15:27:00.4996235Z ------------------------------------------
2020-01-10T15:27:00.4996286Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T15:27:00.4997016Z    |
2020-01-10T15:27:00.4997016Z    |
2020-01-10T15:27:00.4997059Z LL | #![forbid(test_lint)]
2020-01-10T15:27:00.4997309Z    |           --------- `forbid` level set here
2020-01-10T15:27:00.4997355Z ...
2020-01-10T15:27:00.4997396Z LL | #[allow(test_lint)]
2020-01-10T15:27:00.4997442Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T15:27:00.4997492Z 
2020-01-10T15:27:00.4997536Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T15:27:00.4998034Z    |
2020-01-10T15:27:00.4998034Z    |
2020-01-10T15:27:00.4998151Z LL | #![forbid(test_lint)]
2020-01-10T15:27:00.4998419Z    |           --------- `forbid` level set here
2020-01-10T15:27:00.4998486Z ...
2020-01-10T15:27:00.4998527Z LL | #[allow(test_lint)]
2020-01-10T15:27:00.4998571Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T15:27:00.4998602Z 
2020-01-10T15:27:00.4998990Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T15:27:00.4999369Z    |
2020-01-10T15:27:00.4999414Z LL | #![plugin(lint_plugin_test)]
2020-01-10T15:27:00.4999467Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-10T15:27:00.4999514Z    |
2020-01-10T15:27:00.4999514Z    |
2020-01-10T15:27:00.4999577Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T15:27:00.4999608Z 
2020-01-10T15:27:00.4999850Z error: item is named 'lintme'
2020-01-10T15:27:00.5000193Z    |
2020-01-10T15:27:00.5000193Z    |
2020-01-10T15:27:00.5000439Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T15:27:00.5000550Z    |
2020-01-10T15:27:00.5000594Z note: the lint level is defined here
2020-01-10T15:27:00.5000863Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T15:27:00.5000929Z    |
2020-01-10T15:27:00.5000929Z    |
2020-01-10T15:27:00.5000974Z LL | #![forbid(test_lint)]
2020-01-10T15:27:00.5001050Z 
2020-01-10T15:27:00.5001050Z 
2020-01-10T15:27:00.5001114Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T15:27:00.5001432Z    |
2020-01-10T15:27:00.5001432Z    |
2020-01-10T15:27:00.5001493Z LL | #![forbid(test_lint)]
2020-01-10T15:27:00.5001745Z    |           --------- `forbid` level set here
2020-01-10T15:27:00.5001794Z ...
2020-01-10T15:27:00.5001854Z LL | #[allow(test_lint)]
2020-01-10T15:27:00.5001910Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T15:27:00.5001986Z error: aborting due to 4 previous errors
2020-01-10T15:27:00.5002032Z 
2020-01-10T15:27:00.5002296Z For more information about this error, try `rustc --explain E0453`.
2020-01-10T15:27:00.5002331Z 
2020-01-10T15:27:00.5002331Z 
2020-01-10T15:27:00.5002563Z ------------------------------------------
2020-01-10T15:27:00.5002612Z 
2020-01-10T15:27:00.5002638Z 
2020-01-10T15:27:00.5003372Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-10T15:27:00.5003444Z diff of stderr:
2020-01-10T15:27:00.5003494Z 
2020-01-10T15:27:00.5003535Z 70 LL | fn lintme() { }
2020-01-10T15:27:00.5003616Z 72    |
2020-01-10T15:27:00.5003928Z - note: lint level defined here
2020-01-10T15:27:00.5003977Z + note: the lint level is defined here
2020-01-10T15:27:00.5004208Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5004208Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5004272Z 75    |
2020-01-10T15:27:00.5004314Z 76 LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5004350Z 
2020-01-10T15:27:00.5004391Z 83 LL |     fn lintmetoo() { }
2020-01-10T15:27:00.5004694Z 85    |
2020-01-10T15:27:00.5004957Z - note: lint level defined here
2020-01-10T15:27:00.5005026Z + note: the lint level is defined here
2020-01-10T15:27:00.5005240Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5005240Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5005828Z 88    |
2020-01-10T15:27:00.5005884Z 89 LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5005936Z 
2020-01-10T15:27:00.5005961Z 
2020-01-10T15:27:00.5006006Z The actual stderr differed from the expected stderr.
2020-01-10T15:27:00.5006804Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2020-01-10T15:27:00.5007093Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T15:27:00.5007483Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2020-01-10T15:27:00.5007655Z error: 1 errors occurred comparing output.
2020-01-10T15:27:00.5007699Z status: exit code: 1
2020-01-10T15:27:00.5007699Z status: exit code: 1
2020-01-10T15:27:00.5008606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-01-10T15:27:00.5008956Z ------------------------------------------
2020-01-10T15:27:00.5009008Z 
2020-01-10T15:27:00.5009228Z ------------------------------------------
2020-01-10T15:27:00.5009280Z stderr:
2020-01-10T15:27:00.5009280Z stderr:
2020-01-10T15:27:00.5009508Z ------------------------------------------
2020-01-10T15:27:00.5009826Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5010139Z    |
2020-01-10T15:27:00.5010139Z    |
2020-01-10T15:27:00.5010181Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T15:27:00.5010231Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T15:27:00.5010340Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-01-10T15:27:00.5010371Z 
2020-01-10T15:27:00.5010371Z 
2020-01-10T15:27:00.5010696Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5011025Z    |
2020-01-10T15:27:00.5011066Z LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5011130Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T15:27:00.5011160Z 
2020-01-10T15:27:00.5011160Z 
2020-01-10T15:27:00.5011479Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5011787Z    |
2020-01-10T15:27:00.5011787Z    |
2020-01-10T15:27:00.5011828Z LL | #[allow(test_group)]
2020-01-10T15:27:00.5011892Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T15:27:00.5011923Z 
2020-01-10T15:27:00.5011966Z warning: unknown lint: `this_lint_does_not_exist`
2020-01-10T15:27:00.5012266Z    |
2020-01-10T15:27:00.5012266Z    |
2020-01-10T15:27:00.5012323Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-01-10T15:27:00.5012435Z    |
2020-01-10T15:27:00.5012478Z    = note: `#[warn(unknown_lints)]` on by default
2020-01-10T15:27:00.5012507Z 
2020-01-10T15:27:00.5012507Z 
2020-01-10T15:27:00.5012821Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5013133Z    |
2020-01-10T15:27:00.5013133Z    |
2020-01-10T15:27:00.5013176Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T15:27:00.5013242Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T15:27:00.5013272Z 
2020-01-10T15:27:00.5013585Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5013993Z    |
2020-01-10T15:27:00.5014090Z LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5014155Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T15:27:00.5014186Z 
2020-01-10T15:27:00.5014186Z 
2020-01-10T15:27:00.5015676Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5016513Z    |
2020-01-10T15:27:00.5016513Z    |
2020-01-10T15:27:00.5016554Z LL | #[allow(test_group)]
2020-01-10T15:27:00.5016601Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T15:27:00.5016650Z 
2020-01-10T15:27:00.5017015Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T15:27:00.5017337Z    |
2020-01-10T15:27:00.5017337Z    |
2020-01-10T15:27:00.5017393Z LL | #![plugin(lint_tool_test)]
2020-01-10T15:27:00.5017512Z    |
2020-01-10T15:27:00.5017556Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T15:27:00.5017586Z 
2020-01-10T15:27:00.5017586Z 
2020-01-10T15:27:00.5017902Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5018214Z    |
2020-01-10T15:27:00.5018214Z    |
2020-01-10T15:27:00.5018255Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T15:27:00.5018322Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T15:27:00.5018354Z 
2020-01-10T15:27:00.5018668Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5018979Z    |
2020-01-10T15:27:00.5019021Z LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5019074Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T15:27:00.5019121Z 
2020-01-10T15:27:00.5019121Z 
2020-01-10T15:27:00.5019329Z error: item is named 'lintme'
2020-01-10T15:27:00.5019624Z    |
2020-01-10T15:27:00.5019624Z    |
2020-01-10T15:27:00.5020335Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T15:27:00.5020438Z    |
2020-01-10T15:27:00.5020500Z note: the lint level is defined here
2020-01-10T15:27:00.5020745Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5020791Z    |
2020-01-10T15:27:00.5020791Z    |
2020-01-10T15:27:00.5020850Z LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5020892Z    |         ^^^^^^^^^^^^
2020-01-10T15:27:00.5020941Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2020-01-10T15:27:00.5021001Z 
2020-01-10T15:27:00.5021212Z error: item is named 'lintmetoo'
2020-01-10T15:27:00.5021934Z    |
2020-01-10T15:27:00.5021934Z    |
2020-01-10T15:27:00.5022193Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2020-01-10T15:27:00.5022282Z    |
2020-01-10T15:27:00.5022341Z note: the lint level is defined here
2020-01-10T15:27:00.5022578Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T15:27:00.5022624Z    |
2020-01-10T15:27:00.5022624Z    |
2020-01-10T15:27:00.5022681Z LL | #![deny(clippy_group)]
2020-01-10T15:27:00.5022724Z    |         ^^^^^^^^^^^^
2020-01-10T15:27:00.5022772Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2020-01-10T15:27:00.5022803Z 
2020-01-10T15:27:00.5023138Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T15:27:00.5023587Z    |
2020-01-10T15:27:00.5023587Z    |
2020-01-10T15:27:00.5023649Z LL | #[allow(test_group)]
2020-01-10T15:27:00.5023766Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T15:27:00.5023838Z error: aborting due to 2 previous errors
2020-01-10T15:27:00.5023883Z 
2020-01-10T15:27:00.5023907Z 
2020-01-10T15:27:00.5024168Z ------------------------------------------
---
2020-01-10T15:27:00.5027457Z test result: FAILED. 55 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-10T15:27:00.5027492Z 
2020-01-10T15:27:00.5027517Z 
2020-01-10T15:27:00.5027560Z 
2020-01-10T15:27:00.5029896Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T15:27:00.5030182Z 
2020-01-10T15:27:00.5030210Z 
2020-01-10T15:27:00.5030263Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T15:27:00.5030328Z Build completed unsuccessfully in 1:03:27
2020-01-10T15:27:00.5030328Z Build completed unsuccessfully in 1:03:27
2020-01-10T15:27:00.5030667Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T15:27:00.5030727Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T15:27:00.5030842Z == clock drift check ==
2020-01-10T15:27:00.5030887Z   local time: Fri Jan 10 15:27:00 UTC 2020
2020-01-10T15:27:00.5701264Z   network time: Fri, 10 Jan 2020 15:27:00 GMT
2020-01-10T15:27:00.5701354Z == end clock drift check ==
2020-01-10T15:27:01.3527063Z 
2020-01-10T15:27:01.3626350Z ##[error]Bash exited with code '1'.
2020-01-10T15:27:01.3684518Z ##[section]Starting: Checkout
2020-01-10T15:27:01.3686825Z ==============================================================================
2020-01-10T15:27:01.3686900Z Task         : Get sources
2020-01-10T15:27:01.3686949Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
