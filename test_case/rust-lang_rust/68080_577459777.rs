plain
2020-01-22T23:58:26.8944049Z ========================== Starting Command Output ===========================
2020-01-22T23:58:26.8947525Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5117a569-b97e-451e-a785-7686192621c2.sh
2020-01-22T23:58:26.8947569Z 
2020-01-22T23:58:26.8950882Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T23:58:26.8957973Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68080/merge to s
2020-01-22T23:58:26.8959813Z Task         : Get sources
2020-01-22T23:58:26.8959850Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T23:58:26.8959938Z Version      : 1.0.0
2020-01-22T23:58:26.8959975Z Author       : Microsoft
---
2020-01-22T23:58:27.9546256Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T23:58:27.9562920Z ##[command]git config gc.auto 0
2020-01-22T23:58:27.9567642Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T23:58:27.9569925Z ##[command]git config --get-all http.proxy
2020-01-22T23:58:27.9577386Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68080/merge:refs/remotes/pull/68080/merge
---
2020-01-23T00:55:23.5964279Z .................................................................................................... 1700/9546
2020-01-23T00:55:29.7640115Z .................................................................................................... 1800/9546
2020-01-23T00:55:41.7172406Z .....................i.............................................................................. 1900/9546
2020-01-23T00:55:48.7502704Z .................................................................................................... 2000/9546
2020-01-23T00:56:04.1745163Z ...........iiiii.................................................................................... 2100/9546
2020-01-23T00:56:13.9876342Z .................................................................................................... 2300/9546
2020-01-23T00:56:16.4960420Z .................................................................................................... 2400/9546
2020-01-23T00:56:21.9137564Z .................................................................................................... 2500/9546
2020-01-23T00:56:43.1787211Z .................................................................................................... 2600/9546
---
2020-01-23T00:59:27.7388460Z .......................................................i...............i............................ 4900/9546
2020-01-23T00:59:36.0675050Z .................................................................................................... 5000/9546
2020-01-23T00:59:44.1736207Z ..................................................................................................i. 5100/9546
2020-01-23T00:59:49.3428498Z .................................................................................................... 5200/9546
2020-01-23T01:00:00.3962599Z ......................................................................ii.ii...........i............. 5300/9546
2020-01-23T01:00:09.7163755Z .......i............................................................................................ 5500/9546
2020-01-23T01:00:20.0696103Z .................................................................................................... 5600/9546
2020-01-23T01:00:26.7767204Z ........................................................i........................................... 5700/9546
2020-01-23T01:00:33.9664419Z .................................................................................................... 5800/9546
2020-01-23T01:00:33.9664419Z .................................................................................................... 5800/9546
2020-01-23T01:00:44.0800577Z .................................................................................................... 5900/9546
2020-01-23T01:00:51.0990078Z ...............................................ii...i..ii...........i............................... 6000/9546
2020-01-23T01:01:13.7772072Z .................................................................................................... 6200/9546
2020-01-23T01:01:22.4649455Z .................................................................................................... 6300/9546
2020-01-23T01:01:22.4649455Z .................................................................................................... 6300/9546
2020-01-23T01:01:32.5689118Z ...........................................................................i..ii.................... 6400/9546
2020-01-23T01:02:02.9102722Z .................................................................................................... 6600/9546
2020-01-23T01:02:07.8397834Z ...................................................i................................................ 6700/9546
2020-01-23T01:02:10.1152431Z .................................................................................................... 6800/9546
2020-01-23T01:02:12.4571453Z ..................................................i................................................. 6900/9546
---
2020-01-23T01:03:57.8331810Z .................................................................................................... 7600/9546
2020-01-23T01:04:03.8017732Z .................................................................................................... 7700/9546
2020-01-23T01:04:10.6716972Z .................................................................................................... 7800/9546
2020-01-23T01:04:21.6402044Z .................................................................................................... 7900/9546
2020-01-23T01:04:27.9117390Z ......iiiiiii....................................................................................... 8000/9546
2020-01-23T01:04:42.9459304Z .................................................................................................... 8200/9546
2020-01-23T01:04:54.8508673Z .................................................................................................... 8300/9546
2020-01-23T01:05:07.8166360Z .................................................................................................... 8400/9546
2020-01-23T01:05:14.2960263Z .................................................................................................... 8500/9546
---
2020-01-23T01:07:40.0562256Z  finished in 7.614
2020-01-23T01:07:40.0743635Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:07:40.2823780Z 
2020-01-23T01:07:40.2824419Z running 166 tests
2020-01-23T01:07:43.5658493Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-23T01:07:45.9230538Z i.i.i...iii..ii.iiiii......................iii............ii......
2020-01-23T01:07:45.9235126Z 
2020-01-23T01:07:45.9238954Z  finished in 5.849
2020-01-23T01:07:45.9419878Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:07:46.1126874Z 
---
2020-01-23T01:07:48.1577770Z  finished in 2.215
2020-01-23T01:07:48.1768140Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:07:48.3415482Z 
2020-01-23T01:07:48.3416127Z running 9 tests
2020-01-23T01:07:48.3434414Z iiiiiiiii
2020-01-23T01:07:48.3523591Z 
2020-01-23T01:07:48.3523642Z  finished in 0.166
2020-01-23T01:07:48.3640626Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:07:48.5587699Z 
---
2020-01-23T01:08:09.1633950Z  finished in 20.799
2020-01-23T01:08:09.1851075Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:08:09.3750340Z 
2020-01-23T01:08:09.3750591Z running 116 tests
2020-01-23T01:08:35.2663990Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-23T01:08:38.8430646Z .....iiii.....ii
2020-01-23T01:08:38.8432232Z 
2020-01-23T01:08:38.8435615Z  finished in 29.658
2020-01-23T01:08:38.8440643Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:08:38.8445075Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-23T01:08:38.8445075Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-23T01:08:38.8686165Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T01:08:39.0495316Z 
2020-01-23T01:08:39.0496146Z running 63 tests
2020-01-23T01:09:20.3633309Z ..............F.FFFF....................F..F..F................
2020-01-23T01:09:20.3673219Z 
2020-01-23T01:09:20.3674114Z ---- [ui] ui-fulldeps/internal-lints/default_hash_types.rs stdout ----
2020-01-23T01:09:20.3675110Z diff of stderr:
2020-01-23T01:09:20.3676360Z 
---
2020-01-23T01:09:20.3680683Z 
2020-01-23T01:09:20.3680881Z 
2020-01-23T01:09:20.3681140Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3682252Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/default_hash_types.stderr
2020-01-23T01:09:20.3682901Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3683524Z To only update this specific test, also pass `--test-args internal-lints/default_hash_types.rs`
2020-01-23T01:09:20.3684046Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3684270Z status: exit code: 1
2020-01-23T01:09:20.3684270Z status: exit code: 1
2020-01-23T01:09:20.3685412Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3686661Z ------------------------------------------
2020-01-23T01:09:20.3686956Z 
2020-01-23T01:09:20.3687432Z ------------------------------------------
2020-01-23T01:09:20.3687737Z stderr:
2020-01-23T01:09:20.3687737Z stderr:
2020-01-23T01:09:20.3688198Z ------------------------------------------
2020-01-23T01:09:20.3696978Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-23T01:09:20.3697504Z    |
2020-01-23T01:09:20.3697552Z LL |     let _map: HashMap<String, String> = HashMap::default();
2020-01-23T01:09:20.3697618Z    |               ^^^^^^^ help: use: `FxHashMap`
2020-01-23T01:09:20.3697670Z    |
2020-01-23T01:09:20.3697670Z    |
2020-01-23T01:09:20.3697713Z note: the lint level is defined here
2020-01-23T01:09:20.3697999Z   --> /checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs:10:8
2020-01-23T01:09:20.3698048Z    |
2020-01-23T01:09:20.3698090Z LL | #[deny(rustc::default_hash_types)]
2020-01-23T01:09:20.3698150Z    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-23T01:09:20.3698199Z    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
2020-01-23T01:09:20.3698232Z 
2020-01-23T01:09:20.3698278Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-23T01:09:20.3702831Z    |
2020-01-23T01:09:20.3702879Z LL |     let _map: HashMap<String, String> = HashMap::default();
2020-01-23T01:09:20.3702955Z    |                                         ^^^^^^^ help: use: `FxHashMap`
2020-01-23T01:09:20.3703000Z    |
2020-01-23T01:09:20.3703000Z    |
2020-01-23T01:09:20.3703160Z    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
2020-01-23T01:09:20.3703200Z 
2020-01-23T01:09:20.3703266Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-23T01:09:20.3703749Z    |
2020-01-23T01:09:20.3703810Z LL |     let _set: HashSet<String> = HashSet::default();
2020-01-23T01:09:20.3703858Z    |               ^^^^^^^ help: use: `FxHashSet`
2020-01-23T01:09:20.3703901Z    |
2020-01-23T01:09:20.3703901Z    |
2020-01-23T01:09:20.3703964Z    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
2020-01-23T01:09:20.3703996Z 
2020-01-23T01:09:20.3704054Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-23T01:09:20.3704392Z    |
2020-01-23T01:09:20.3704436Z LL |     let _set: HashSet<String> = HashSet::default();
2020-01-23T01:09:20.3704493Z    |                                 ^^^^^^^ help: use: `FxHashSet`
2020-01-23T01:09:20.3704554Z    |
---
2020-01-23T01:09:20.3706592Z 
2020-01-23T01:09:20.3706618Z 
2020-01-23T01:09:20.3706662Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3707057Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2020-01-23T01:09:20.3707317Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3707621Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2020-01-23T01:09:20.3707711Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3707754Z status: exit code: 1
2020-01-23T01:09:20.3707754Z status: exit code: 1
2020-01-23T01:09:20.3750315Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3750794Z ------------------------------------------
2020-01-23T01:09:20.3751021Z 
2020-01-23T01:09:20.3751319Z ------------------------------------------
2020-01-23T01:09:20.3751400Z stderr:
2020-01-23T01:09:20.3751400Z stderr:
2020-01-23T01:09:20.3751647Z ------------------------------------------
2020-01-23T01:09:20.3751702Z error: implementing `LintPass` by hand
2020-01-23T01:09:20.3752123Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:20:6
2020-01-23T01:09:20.3752194Z    |
2020-01-23T01:09:20.3752251Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2020-01-23T01:09:20.3752369Z    |
2020-01-23T01:09:20.3752417Z note: the lint level is defined here
2020-01-23T01:09:20.3752771Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2020-01-23T01:09:20.3752828Z    |
2020-01-23T01:09:20.3752828Z    |
2020-01-23T01:09:20.3752877Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2020-01-23T01:09:20.3752947Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-23T01:09:20.3753004Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-23T01:09:20.3753099Z error: implementing `LintPass` by hand
2020-01-23T01:09:20.3753423Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:30:14
2020-01-23T01:09:20.3753477Z    |
2020-01-23T01:09:20.3753477Z    |
2020-01-23T01:09:20.3753541Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2020-01-23T01:09:20.3753658Z ...
2020-01-23T01:09:20.3753658Z ...
2020-01-23T01:09:20.3753705Z LL | custom_lint_pass_macro!();
2020-01-23T01:09:20.3754040Z    |
2020-01-23T01:09:20.3754040Z    |
2020-01-23T01:09:20.3754092Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-23T01:09:20.3754192Z error: aborting due to 2 previous errors
2020-01-23T01:09:20.3754223Z 
2020-01-23T01:09:20.3754252Z 
2020-01-23T01:09:20.3754495Z ------------------------------------------
---
2020-01-23T01:09:20.3755769Z - note: lint level defined here
2020-01-23T01:09:20.3755823Z + note: the lint level is defined here
2020-01-23T01:09:20.3756965Z 8   --> $DIR/pass_ty_by_ref.rs:4:9
2020-01-23T01:09:20.3757048Z 9    |
2020-01-23T01:09:20.3757098Z 10 LL | #![deny(rustc::ty_pass_by_reference)]
2020-01-23T01:09:20.3757159Z 
2020-01-23T01:09:20.3757225Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3757591Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
2020-01-23T01:09:20.3757591Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
2020-01-23T01:09:20.3757895Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3758215Z To only update this specific test, also pass `--test-args internal-lints/pass_ty_by_ref.rs`
2020-01-23T01:09:20.3758304Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3758362Z status: exit code: 1
2020-01-23T01:09:20.3758362Z status: exit code: 1
2020-01-23T01:09:20.3759356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3759919Z ------------------------------------------
2020-01-23T01:09:20.3759958Z 
2020-01-23T01:09:20.3760199Z ------------------------------------------
2020-01-23T01:09:20.3760268Z stderr:
2020-01-23T01:09:20.3760268Z stderr:
2020-01-23T01:09:20.3760599Z ------------------------------------------
2020-01-23T01:09:20.3760878Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3761186Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:13:13
2020-01-23T01:09:20.3761241Z    |
2020-01-23T01:09:20.3761510Z LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-23T01:09:20.3761795Z    |             ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3761894Z note: the lint level is defined here
2020-01-23T01:09:20.3762189Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
2020-01-23T01:09:20.3762242Z    |
2020-01-23T01:09:20.3762242Z    |
2020-01-23T01:09:20.3762302Z LL | #![deny(rustc::ty_pass_by_reference)]
2020-01-23T01:09:20.3762401Z 
2020-01-23T01:09:20.3762644Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3762923Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:15:18
2020-01-23T01:09:20.3763003Z    |
2020-01-23T01:09:20.3763003Z    |
2020-01-23T01:09:20.3763286Z LL |     ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3763568Z    |                  ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3763856Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3764135Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:28
2020-01-23T01:09:20.3764188Z    |
2020-01-23T01:09:20.3764188Z    |
2020-01-23T01:09:20.3764477Z LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-23T01:09:20.3764759Z    |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3765061Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3765343Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:55
2020-01-23T01:09:20.3765395Z    |
2020-01-23T01:09:20.3765395Z    |
2020-01-23T01:09:20.3765681Z LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-23T01:09:20.3766296Z    |                                                       ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3766624Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3766929Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:26:17
2020-01-23T01:09:20.3766982Z    |
2020-01-23T01:09:20.3766982Z    |
2020-01-23T01:09:20.3767250Z LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-23T01:09:20.3767539Z    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3767808Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3768118Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:28:22
2020-01-23T01:09:20.3768171Z    |
2020-01-23T01:09:20.3768171Z    |
2020-01-23T01:09:20.3768454Z LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3768740Z    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3769032Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3769342Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:41
2020-01-23T01:09:20.3769395Z    |
2020-01-23T01:09:20.3769395Z    |
2020-01-23T01:09:20.3769680Z LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
2020-01-23T01:09:20.3769997Z    |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3770272Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3770552Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:68
2020-01-23T01:09:20.3770756Z    |
2020-01-23T01:09:20.3770756Z    |
2020-01-23T01:09:20.3771083Z LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
2020-01-23T01:09:20.3771422Z    |                                                                    ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3771801Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3772124Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:53:17
2020-01-23T01:09:20.3772197Z    |
2020-01-23T01:09:20.3772197Z    |
2020-01-23T01:09:20.3772468Z LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
2020-01-23T01:09:20.3772736Z    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3773028Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3773308Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:55:22
2020-01-23T01:09:20.3773359Z    |
2020-01-23T01:09:20.3773359Z    |
2020-01-23T01:09:20.3773661Z LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3773963Z    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3774231Z error: passing `Ty<'_>` by reference
2020-01-23T01:09:20.3774540Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:38
2020-01-23T01:09:20.3774592Z    |
2020-01-23T01:09:20.3774592Z    |
2020-01-23T01:09:20.3774879Z LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-23T01:09:20.3775193Z    |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
2020-01-23T01:09:20.3775464Z error: passing `TyCtxt<'_>` by reference
2020-01-23T01:09:20.3775760Z   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:65
2020-01-23T01:09:20.3775813Z    |
2020-01-23T01:09:20.3775813Z    |
2020-01-23T01:09:20.3776373Z LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
2020-01-23T01:09:20.3776742Z    |                                                                 ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
2020-01-23T01:09:20.3776847Z error: aborting due to 12 previous errors
2020-01-23T01:09:20.3776878Z 
2020-01-23T01:09:20.3776906Z 
2020-01-23T01:09:20.3777169Z ------------------------------------------
2020-01-23T01:09:20.3777169Z ------------------------------------------
2020-01-23T01:09:20.3777213Z 
2020-01-23T01:09:20.3777242Z 
2020-01-23T01:09:20.3777515Z ---- [ui] ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs stdout ----
2020-01-23T01:09:20.3777588Z diff of stderr:
2020-01-23T01:09:20.3777619Z 
2020-01-23T01:09:20.3777849Z 4 LL |     ty_q: ty::Ty<'_>,
2020-01-23T01:09:20.3778166Z 5    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
2020-01-23T01:09:20.3778480Z - note: lint level defined here
2020-01-23T01:09:20.3778533Z + note: the lint level is defined here
2020-01-23T01:09:20.3778799Z 8   --> $DIR/qualified_ty_ty_ctxt.rs:4:9
2020-01-23T01:09:20.3778848Z 9    |
2020-01-23T01:09:20.3778848Z 9    |
2020-01-23T01:09:20.3778907Z 10 LL | #![deny(rustc::usage_of_qualified_ty)]
2020-01-23T01:09:20.3778986Z 
2020-01-23T01:09:20.3779036Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3779036Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3779417Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/qualified_ty_ty_ctxt.stderr
2020-01-23T01:09:20.3779722Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3780086Z To only update this specific test, also pass `--test-args internal-lints/qualified_ty_ty_ctxt.rs`
2020-01-23T01:09:20.3780195Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3780244Z status: exit code: 1
2020-01-23T01:09:20.3780244Z status: exit code: 1
2020-01-23T01:09:20.3781362Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3781861Z ------------------------------------------
2020-01-23T01:09:20.3781900Z 
2020-01-23T01:09:20.3782144Z ------------------------------------------
2020-01-23T01:09:20.3782195Z stderr:
2020-01-23T01:09:20.3782195Z stderr:
2020-01-23T01:09:20.3782453Z ------------------------------------------
2020-01-23T01:09:20.3782690Z error: usage of qualified `ty::Ty<'_>`
2020-01-23T01:09:20.3782979Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:25:11
2020-01-23T01:09:20.3783051Z    |
2020-01-23T01:09:20.3783337Z LL |     ty_q: ty::Ty<'_>, //~ ERROR usage of qualified `ty::Ty<'_>`
2020-01-23T01:09:20.3783611Z    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
2020-01-23T01:09:20.3783727Z note: the lint level is defined here
2020-01-23T01:09:20.3784020Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:4:9
2020-01-23T01:09:20.3784093Z    |
2020-01-23T01:09:20.3784093Z    |
2020-01-23T01:09:20.3784141Z LL | #![deny(rustc::usage_of_qualified_ty)]
2020-01-23T01:09:20.3784225Z 
2020-01-23T01:09:20.3784483Z error: usage of qualified `ty::TyCtxt<'_>`
2020-01-23T01:09:20.3784773Z   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:27:16
2020-01-23T01:09:20.3784828Z    |
2020-01-23T01:09:20.3784828Z    |
2020-01-23T01:09:20.3785126Z LL |     ty_ctxt_q: ty::TyCtxt<'_>, //~ ERROR usage of qualified `ty::TyCtxt<'_>`
2020-01-23T01:09:20.3785417Z    |                ^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
2020-01-23T01:09:20.3785512Z error: aborting due to 2 previous errors
2020-01-23T01:09:20.3785561Z 
2020-01-23T01:09:20.3785589Z 
2020-01-23T01:09:20.3785831Z ------------------------------------------
2020-01-23T01:09:20.3785831Z ------------------------------------------
2020-01-23T01:09:20.3785865Z 
2020-01-23T01:09:20.3785893Z 
2020-01-23T01:09:20.3786471Z ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
2020-01-23T01:09:20.3786536Z diff of stderr:
2020-01-23T01:09:20.3786567Z 
2020-01-23T01:09:20.3786633Z 4 LL |     let kind = TyKind::Bool;
2020-01-23T01:09:20.3786689Z 5    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3786980Z - note: lint level defined here
2020-01-23T01:09:20.3787051Z + note: the lint level is defined here
2020-01-23T01:09:20.3787290Z 8   --> $DIR/ty_tykind_usage.rs:9:8
2020-01-23T01:09:20.3787339Z 9    |
2020-01-23T01:09:20.3787339Z 9    |
2020-01-23T01:09:20.3787403Z 10 LL | #[deny(rustc::usage_of_ty_tykind)]
2020-01-23T01:09:20.3787464Z 
2020-01-23T01:09:20.3787523Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3787904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2020-01-23T01:09:20.3787904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
2020-01-23T01:09:20.3788187Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3788503Z To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
2020-01-23T01:09:20.3788610Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3788657Z status: exit code: 1
2020-01-23T01:09:20.3788657Z status: exit code: 1
2020-01-23T01:09:20.3789650Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3790258Z ------------------------------------------
2020-01-23T01:09:20.3790304Z 
2020-01-23T01:09:20.3790576Z ------------------------------------------
2020-01-23T01:09:20.3790648Z stderr:
2020-01-23T01:09:20.3790648Z stderr:
2020-01-23T01:09:20.3790889Z ------------------------------------------
2020-01-23T01:09:20.3790942Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3791241Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:11:16
2020-01-23T01:09:20.3791298Z    |
2020-01-23T01:09:20.3791353Z LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3791426Z    |                ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3791532Z note: the lint level is defined here
2020-01-23T01:09:20.3791819Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:9:8
2020-01-23T01:09:20.3791889Z    |
2020-01-23T01:09:20.3791889Z    |
2020-01-23T01:09:20.3791936Z LL | #[deny(rustc::usage_of_ty_tykind)]
2020-01-23T01:09:20.3792045Z 
2020-01-23T01:09:20.3792092Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3792377Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:14:9
2020-01-23T01:09:20.3792431Z    |
2020-01-23T01:09:20.3792431Z    |
2020-01-23T01:09:20.3792505Z LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3792561Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3792657Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3792941Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:15:9
2020-01-23T01:09:20.3793003Z    |
2020-01-23T01:09:20.3793003Z    |
2020-01-23T01:09:20.3793074Z LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3793130Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3793209Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3793520Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:16:9
2020-01-23T01:09:20.3793573Z    |
2020-01-23T01:09:20.3793624Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3793624Z LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3793698Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3793777Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3794062Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:17:9
2020-01-23T01:09:20.3794131Z    |
2020-01-23T01:09:20.3794131Z    |
2020-01-23T01:09:20.3794183Z LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3794238Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3794344Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3794628Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:18:9
2020-01-23T01:09:20.3794697Z    |
2020-01-23T01:09:20.3794759Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3794759Z LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3794815Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3794914Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3795197Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:19:9
2020-01-23T01:09:20.3795250Z    |
2020-01-23T01:09:20.3795318Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3795318Z LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3795374Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3795453Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3796657Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:20:9
2020-01-23T01:09:20.3796732Z    |
2020-01-23T01:09:20.3796732Z    |
2020-01-23T01:09:20.3796785Z LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3796868Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3797102Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3797631Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:21:9
2020-01-23T01:09:20.3797711Z    |
2020-01-23T01:09:20.3797763Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3797763Z LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3797819Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3797915Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798203Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:22:9
2020-01-23T01:09:20.3798273Z    |
2020-01-23T01:09:20.3798338Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798338Z LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798394Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3798491Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798786Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:23:9
2020-01-23T01:09:20.3798839Z    |
2020-01-23T01:09:20.3798908Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798908Z LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3798964Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3799043Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3799345Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:24:9
2020-01-23T01:09:20.3799398Z    |
2020-01-23T01:09:20.3799450Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3799450Z LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3799523Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3799613Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3799899Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:25:9
2020-01-23T01:09:20.3799969Z    |
2020-01-23T01:09:20.3800022Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3800022Z LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3800087Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3800184Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3800469Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:26:9
2020-01-23T01:09:20.3800539Z    |
2020-01-23T01:09:20.3800591Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3800591Z LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3800646Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3800741Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3801023Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:27:9
2020-01-23T01:09:20.3801085Z    |
2020-01-23T01:09:20.3801153Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3801153Z LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3801209Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3801296Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3801599Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:28:9
2020-01-23T01:09:20.3801651Z    |
2020-01-23T01:09:20.3801651Z    |
2020-01-23T01:09:20.3801703Z LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3801777Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3801856Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3802155Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:29:9
2020-01-23T01:09:20.3802208Z    |
2020-01-23T01:09:20.3802261Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3802261Z LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3802432Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3802532Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3802850Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:30:9
2020-01-23T01:09:20.3803017Z    |
2020-01-23T01:09:20.3803078Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3803078Z LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3803135Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3803233Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3803548Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:31:9
2020-01-23T01:09:20.3803601Z    |
2020-01-23T01:09:20.3803601Z    |
2020-01-23T01:09:20.3803674Z LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3803732Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3803838Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3804127Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:32:9
2020-01-23T01:09:20.3804180Z    |
2020-01-23T01:09:20.3804231Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3804231Z LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3804312Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3804392Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3804697Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:33:9
2020-01-23T01:09:20.3804750Z    |
2020-01-23T01:09:20.3804750Z    |
2020-01-23T01:09:20.3804801Z LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3804873Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3804951Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3805233Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:34:9
2020-01-23T01:09:20.3805313Z    |
2020-01-23T01:09:20.3805366Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3805366Z LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3805422Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3805526Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3805812Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:35:9
2020-01-23T01:09:20.3805865Z    |
2020-01-23T01:09:20.3805865Z    |
2020-01-23T01:09:20.3806207Z LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3806268Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3806369Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3806720Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:36:9
2020-01-23T01:09:20.3806774Z    |
2020-01-23T01:09:20.3806774Z    |
2020-01-23T01:09:20.3806825Z LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3806912Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3806992Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3807295Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:37:9
2020-01-23T01:09:20.3807358Z    |
2020-01-23T01:09:20.3807358Z    |
2020-01-23T01:09:20.3807411Z LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3807484Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3807563Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3807850Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:38:9
2020-01-23T01:09:20.3807920Z    |
2020-01-23T01:09:20.3807972Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3807972Z LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3808027Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3808244Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3808562Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:39:9
2020-01-23T01:09:20.3808615Z    |
2020-01-23T01:09:20.3808615Z    |
2020-01-23T01:09:20.3808688Z LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3808824Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3808929Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3809241Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:40:9
2020-01-23T01:09:20.3809294Z    |
2020-01-23T01:09:20.3809294Z    |
2020-01-23T01:09:20.3809363Z LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3809420Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3809499Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3809796Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:41:9
2020-01-23T01:09:20.3809859Z    |
2020-01-23T01:09:20.3809911Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3809911Z LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3809984Z    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3810064Z error: usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3810358Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:46:12
2020-01-23T01:09:20.3810428Z    |
2020-01-23T01:09:20.3810428Z    |
2020-01-23T01:09:20.3810481Z LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
2020-01-23T01:09:20.3810540Z    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
2020-01-23T01:09:20.3810636Z error: usage of `ty::TyKind`
2020-01-23T01:09:20.3810921Z   --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:48:24
2020-01-23T01:09:20.3810991Z    |
2020-01-23T01:09:20.3810991Z    |
2020-01-23T01:09:20.3811285Z LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
2020-01-23T01:09:20.3811399Z    |
2020-01-23T01:09:20.3811463Z    = help: try using `Ty` instead
2020-01-23T01:09:20.3811496Z 
2020-01-23T01:09:20.3811542Z error: aborting due to 31 previous errors
---
2020-01-23T01:09:20.3811937Z 
2020-01-23T01:09:20.3812212Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-01-23T01:09:20.3812265Z diff of stderr:
2020-01-23T01:09:20.3812296Z 
2020-01-23T01:09:20.3812341Z 12 LL | fn lintme() { }
2020-01-23T01:09:20.3812449Z 14    |
2020-01-23T01:09:20.3812678Z - note: lint level defined here
2020-01-23T01:09:20.3812731Z + note: the lint level is defined here
2020-01-23T01:09:20.3812995Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-23T01:09:20.3812995Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-23T01:09:20.3813044Z 17    |
2020-01-23T01:09:20.3813100Z 18 LL | #![deny(test_lint)]
2020-01-23T01:09:20.3813149Z 
2020-01-23T01:09:20.3813177Z 
2020-01-23T01:09:20.3813227Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3813585Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/lint-plugin-deny-attr.stderr
2020-01-23T01:09:20.3813894Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3814195Z To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`
2020-01-23T01:09:20.3814301Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3814349Z status: exit code: 1
2020-01-23T01:09:20.3814349Z status: exit code: 1
2020-01-23T01:09:20.3815354Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3815817Z ------------------------------------------
2020-01-23T01:09:20.3815880Z 
2020-01-23T01:09:20.3816426Z ------------------------------------------
2020-01-23T01:09:20.3816482Z stderr:
2020-01-23T01:09:20.3816482Z stderr:
2020-01-23T01:09:20.3816743Z ------------------------------------------
2020-01-23T01:09:20.3817191Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-23T01:09:20.3817561Z    |
2020-01-23T01:09:20.3817609Z LL | #![plugin(lint_plugin_test)]
2020-01-23T01:09:20.3817679Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-23T01:09:20.3817746Z    |
2020-01-23T01:09:20.3817746Z    |
2020-01-23T01:09:20.3817795Z    = note: `#[warn(deprecated)]` on by default
2020-01-23T01:09:20.3817829Z 
2020-01-23T01:09:20.3818080Z error: item is named 'lintme'
2020-01-23T01:09:20.3818415Z    |
2020-01-23T01:09:20.3818415Z    |
2020-01-23T01:09:20.3818669Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-23T01:09:20.3818785Z    |
2020-01-23T01:09:20.3818831Z note: the lint level is defined here
2020-01-23T01:09:20.3819119Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
2020-01-23T01:09:20.3819171Z    |
---
2020-01-23T01:09:20.3819755Z 
2020-01-23T01:09:20.3820014Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-01-23T01:09:20.3820085Z diff of stderr:
2020-01-23T01:09:20.3820116Z 
2020-01-23T01:09:20.3820170Z 30 LL | fn lintme() { }
2020-01-23T01:09:20.3820279Z 32    |
2020-01-23T01:09:20.3820510Z - note: lint level defined here
2020-01-23T01:09:20.3820563Z + note: the lint level is defined here
2020-01-23T01:09:20.3820829Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-23T01:09:20.3820829Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-23T01:09:20.3820879Z 35    |
2020-01-23T01:09:20.3820925Z 36 LL | #![forbid(test_lint)]
2020-01-23T01:09:20.3821000Z 
2020-01-23T01:09:20.3821049Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3821409Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-23T01:09:20.3821409Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-23T01:09:20.3821720Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3822020Z To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
2020-01-23T01:09:20.3822118Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3822184Z status: exit code: 1
2020-01-23T01:09:20.3822184Z status: exit code: 1
2020-01-23T01:09:20.3823115Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3823649Z ------------------------------------------
2020-01-23T01:09:20.3823688Z 
2020-01-23T01:09:20.3823951Z ------------------------------------------
2020-01-23T01:09:20.3824002Z stderr:
2020-01-23T01:09:20.3824002Z stderr:
2020-01-23T01:09:20.3824323Z ------------------------------------------
2020-01-23T01:09:20.3824410Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-23T01:09:20.3824778Z    |
2020-01-23T01:09:20.3824778Z    |
2020-01-23T01:09:20.3824844Z LL | #![forbid(test_lint)]
2020-01-23T01:09:20.3825097Z    |           --------- `forbid` level set here
2020-01-23T01:09:20.3825148Z ...
2020-01-23T01:09:20.3825213Z LL | #[allow(test_lint)]
2020-01-23T01:09:20.3825264Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-23T01:09:20.3825298Z 
2020-01-23T01:09:20.3825348Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-23T01:09:20.3825716Z    |
2020-01-23T01:09:20.3825716Z    |
2020-01-23T01:09:20.3825762Z LL | #![forbid(test_lint)]
2020-01-23T01:09:20.3826355Z    |           --------- `forbid` level set here
2020-01-23T01:09:20.3826416Z ...
2020-01-23T01:09:20.3826475Z LL | #[allow(test_lint)]
2020-01-23T01:09:20.3826549Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-23T01:09:20.3826583Z 
2020-01-23T01:09:20.3826981Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-23T01:09:20.3827347Z    |
2020-01-23T01:09:20.3827395Z LL | #![plugin(lint_plugin_test)]
2020-01-23T01:09:20.3827468Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-23T01:09:20.3827518Z    |
2020-01-23T01:09:20.3827518Z    |
2020-01-23T01:09:20.3827566Z    = note: `#[warn(deprecated)]` on by default
2020-01-23T01:09:20.3827609Z 
2020-01-23T01:09:20.3827857Z error: item is named 'lintme'
2020-01-23T01:09:20.3828182Z    |
2020-01-23T01:09:20.3828182Z    |
2020-01-23T01:09:20.3828449Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-23T01:09:20.3828557Z    |
2020-01-23T01:09:20.3828603Z note: the lint level is defined here
2020-01-23T01:09:20.3828899Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
2020-01-23T01:09:20.3828951Z    |
2020-01-23T01:09:20.3828951Z    |
2020-01-23T01:09:20.3828997Z LL | #![forbid(test_lint)]
2020-01-23T01:09:20.3829095Z 
2020-01-23T01:09:20.3829095Z 
2020-01-23T01:09:20.3829145Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-23T01:09:20.3829494Z    |
2020-01-23T01:09:20.3829494Z    |
2020-01-23T01:09:20.3829540Z LL | #![forbid(test_lint)]
2020-01-23T01:09:20.3829799Z    |           --------- `forbid` level set here
2020-01-23T01:09:20.3829866Z ...
2020-01-23T01:09:20.3829912Z LL | #[allow(test_lint)]
2020-01-23T01:09:20.3829962Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-23T01:09:20.3830059Z error: aborting due to 4 previous errors
2020-01-23T01:09:20.3830100Z 
2020-01-23T01:09:20.3830374Z For more information about this error, try `rustc --explain E0453`.
2020-01-23T01:09:20.3830428Z 
2020-01-23T01:09:20.3830428Z 
2020-01-23T01:09:20.3830668Z ------------------------------------------
2020-01-23T01:09:20.3830704Z 
2020-01-23T01:09:20.3830732Z 
2020-01-23T01:09:20.3830978Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-23T01:09:20.3831047Z diff of stderr:
2020-01-23T01:09:20.3831078Z 
2020-01-23T01:09:20.3831122Z 70 LL | fn lintme() { }
2020-01-23T01:09:20.3831227Z 72    |
2020-01-23T01:09:20.3831457Z - note: lint level defined here
2020-01-23T01:09:20.3831509Z + note: the lint level is defined here
2020-01-23T01:09:20.3831934Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3831934Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3831984Z 75    |
2020-01-23T01:09:20.3832031Z 76 LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3832062Z 
2020-01-23T01:09:20.3832125Z 83 LL |     fn lintmetoo() { }
2020-01-23T01:09:20.3832299Z 85    |
2020-01-23T01:09:20.3832582Z - note: lint level defined here
2020-01-23T01:09:20.3832636Z + note: the lint level is defined here
2020-01-23T01:09:20.3832874Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3832874Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3832942Z 88    |
2020-01-23T01:09:20.3832987Z 89 LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3833019Z 
2020-01-23T01:09:20.3833048Z 
2020-01-23T01:09:20.3833113Z The actual stderr differed from the expected stderr.
2020-01-23T01:09:20.3833450Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2020-01-23T01:09:20.3833726Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T01:09:20.3834039Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2020-01-23T01:09:20.3834124Z error: 1 errors occurred comparing output.
2020-01-23T01:09:20.3834173Z status: exit code: 1
2020-01-23T01:09:20.3834173Z status: exit code: 1
2020-01-23T01:09:20.3835107Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-01-23T01:09:20.3835483Z ------------------------------------------
2020-01-23T01:09:20.3835520Z 
2020-01-23T01:09:20.3835761Z ------------------------------------------
2020-01-23T01:09:20.3835829Z stderr:
2020-01-23T01:09:20.3835829Z stderr:
2020-01-23T01:09:20.3838722Z ------------------------------------------
2020-01-23T01:09:20.3839113Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3839500Z    |
2020-01-23T01:09:20.3839500Z    |
2020-01-23T01:09:20.3839548Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-23T01:09:20.3839621Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-23T01:09:20.3839724Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-01-23T01:09:20.3839760Z 
2020-01-23T01:09:20.3839760Z 
2020-01-23T01:09:20.3840140Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3840510Z    |
2020-01-23T01:09:20.3840557Z LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3840612Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-23T01:09:20.3840646Z 
2020-01-23T01:09:20.3840646Z 
2020-01-23T01:09:20.3841030Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3841363Z    |
2020-01-23T01:09:20.3841363Z    |
2020-01-23T01:09:20.3841427Z LL | #[allow(test_group)]
2020-01-23T01:09:20.3841480Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-23T01:09:20.3841515Z 
2020-01-23T01:09:20.3841564Z warning: unknown lint: `this_lint_does_not_exist`
2020-01-23T01:09:20.3841901Z    |
2020-01-23T01:09:20.3841901Z    |
2020-01-23T01:09:20.3842120Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-01-23T01:09:20.3842242Z    |
2020-01-23T01:09:20.3842291Z    = note: `#[warn(unknown_lints)]` on by default
2020-01-23T01:09:20.3842341Z 
2020-01-23T01:09:20.3842341Z 
2020-01-23T01:09:20.3842839Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3843212Z    |
2020-01-23T01:09:20.3843212Z    |
2020-01-23T01:09:20.3843278Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-23T01:09:20.3843333Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-23T01:09:20.3843369Z 
2020-01-23T01:09:20.3843747Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3844089Z    |
2020-01-23T01:09:20.3844152Z LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3844204Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-23T01:09:20.3844238Z 
2020-01-23T01:09:20.3844238Z 
2020-01-23T01:09:20.3844606Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3844954Z    |
2020-01-23T01:09:20.3844954Z    |
2020-01-23T01:09:20.3844998Z LL | #[allow(test_group)]
2020-01-23T01:09:20.3845068Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-23T01:09:20.3845104Z 
2020-01-23T01:09:20.3845506Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-23T01:09:20.3845860Z    |
2020-01-23T01:09:20.3845860Z    |
2020-01-23T01:09:20.3846180Z LL | #![plugin(lint_tool_test)]
2020-01-23T01:09:20.3846319Z    |
2020-01-23T01:09:20.3846368Z    = note: `#[warn(deprecated)]` on by default
2020-01-23T01:09:20.3846402Z 
2020-01-23T01:09:20.3846402Z 
2020-01-23T01:09:20.3846852Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3847185Z    |
2020-01-23T01:09:20.3847185Z    |
2020-01-23T01:09:20.3847250Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-23T01:09:20.3847305Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-23T01:09:20.3847341Z 
2020-01-23T01:09:20.3847715Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3848053Z    |
2020-01-23T01:09:20.3848117Z LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3848172Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-23T01:09:20.3848206Z 
2020-01-23T01:09:20.3848206Z 
2020-01-23T01:09:20.3848435Z error: item is named 'lintme'
2020-01-23T01:09:20.3848779Z    |
2020-01-23T01:09:20.3848779Z    |
2020-01-23T01:09:20.3849033Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-23T01:09:20.3849149Z    |
2020-01-23T01:09:20.3849195Z note: the lint level is defined here
2020-01-23T01:09:20.3849458Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3849526Z    |
2020-01-23T01:09:20.3849526Z    |
2020-01-23T01:09:20.3849571Z LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3849618Z    |         ^^^^^^^^^^^^
2020-01-23T01:09:20.3849689Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2020-01-23T01:09:20.3849859Z 
2020-01-23T01:09:20.3850127Z error: item is named 'lintmetoo'
2020-01-23T01:09:20.3850524Z    |
2020-01-23T01:09:20.3850524Z    |
2020-01-23T01:09:20.3850791Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2020-01-23T01:09:20.3850996Z    |
2020-01-23T01:09:20.3851051Z note: the lint level is defined here
2020-01-23T01:09:20.3851348Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-23T01:09:20.3851419Z    |
2020-01-23T01:09:20.3851419Z    |
2020-01-23T01:09:20.3851465Z LL | #![deny(clippy_group)]
2020-01-23T01:09:20.3851511Z    |         ^^^^^^^^^^^^
2020-01-23T01:09:20.3851582Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2020-01-23T01:09:20.3851618Z 
2020-01-23T01:09:20.3851976Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-23T01:09:20.3856407Z    |
2020-01-23T01:09:20.3856407Z    |
2020-01-23T01:09:20.3856455Z LL | #[allow(test_group)]
2020-01-23T01:09:20.3856508Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-23T01:09:20.3856610Z error: aborting due to 2 previous errors
2020-01-23T01:09:20.3856650Z 
2020-01-23T01:09:20.3856679Z 
2020-01-23T01:09:20.3857047Z ------------------------------------------
---
2020-01-23T01:09:20.3871070Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-23T01:09:20.3871137Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-23T01:09:20.3871173Z 
2020-01-23T01:09:20.3871219Z 
2020-01-23T01:09:20.3873059Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-23T01:09:20.3873523Z 
2020-01-23T01:09:20.3873557Z 
2020-01-23T01:09:20.3873609Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T01:09:20.3873681Z Build completed unsuccessfully in 1:04:56
2020-01-23T01:09:20.3873681Z Build completed unsuccessfully in 1:04:56
2020-01-23T01:09:20.3873866Z == clock drift check ==
2020-01-23T01:09:20.3873926Z   local time: Thu Jan 23 01:09:20 UTC 2020
2020-01-23T01:09:20.6842309Z   network time: Thu, 23 Jan 2020 01:09:20 GMT
2020-01-23T01:09:20.6842429Z == end clock drift check ==
2020-01-23T01:09:21.7959765Z 
2020-01-23T01:09:21.8099432Z ##[error]Bash exited with code '1'.
2020-01-23T01:09:21.8139581Z ##[section]Finishing: Run build
2020-01-23T01:09:21.8417167Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68080/merge to s
2020-01-23T01:09:21.8419407Z Task         : Get sources
2020-01-23T01:09:21.8419676Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T01:09:21.8419750Z Version      : 1.0.0
2020-01-23T01:09:21.8419796Z Author       : Microsoft
2020-01-23T01:09:21.8419796Z Author       : Microsoft
2020-01-23T01:09:21.8419848Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T01:09:21.8419921Z ==============================================================================
2020-01-23T01:09:22.3561565Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T01:09:22.3622977Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68080/merge to s
2020-01-23T01:09:22.3748368Z Cleaning up task key
2020-01-23T01:09:22.3749125Z Start cleaning up orphan processes.
2020-01-23T01:09:22.3870159Z Terminate orphan process: pid (3446) (python)
2020-01-23T01:09:22.4499145Z ##[section]Finishing: Finalize Job
