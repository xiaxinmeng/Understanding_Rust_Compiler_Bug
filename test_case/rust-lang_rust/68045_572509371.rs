plain
2020-01-09T09:52:03.3613658Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T09:52:03.3775089Z ##[command]git config gc.auto 0
2020-01-09T09:52:03.3866223Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T09:52:03.3942497Z ##[command]git config --get-all http.proxy
2020-01-09T09:52:03.4095405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68045/merge:refs/remotes/pull/68045/merge
---
2020-01-09T10:45:11.7553610Z .................................................................................................... 1600/9484
2020-01-09T10:45:16.7063226Z .................................................................................................... 1700/9484
2020-01-09T10:45:25.5889781Z .................................................................................................... 1800/9484
2020-01-09T10:45:33.9050770Z .....i.............................................................................................. 1900/9484
2020-01-09T10:45:40.5656640Z .............................................................................................iiiii.. 2000/9484
2020-01-09T10:46:01.9995996Z .................................................................................................... 2200/9484
2020-01-09T10:46:04.4563381Z .................................................................................................... 2300/9484
2020-01-09T10:46:06.8714207Z .................................................................................................... 2400/9484
2020-01-09T10:46:12.6760002Z .................................................................................................... 2500/9484
---
2020-01-09T10:49:07.8618813Z .........................i...............i.......................................................... 4900/9484
2020-01-09T10:49:17.7850392Z .................................................................................................... 5000/9484
2020-01-09T10:49:23.5857850Z ......................................................................i............................. 5100/9484
2020-01-09T10:49:29.9962050Z .................................................................................................... 5200/9484
2020-01-09T10:49:38.6505723Z .....................................ii.ii...........i.............................................. 5300/9484
2020-01-09T10:49:48.1703740Z .................................................................................................... 5500/9484
2020-01-09T10:49:57.6768365Z .................................................................................................... 5600/9484
2020-01-09T10:50:04.9179362Z .....................i.............................................................................. 5700/9484
2020-01-09T10:50:10.8823708Z .................................................................................................... 5800/9484
2020-01-09T10:50:10.8823708Z .................................................................................................... 5800/9484
2020-01-09T10:50:21.8151946Z .................................................................................................... 5900/9484
2020-01-09T10:50:33.2939299Z ...........ii...i..ii...........i................................................................... 6000/9484
2020-01-09T10:50:50.5434113Z .................................................................................................... 6200/9484
2020-01-09T10:50:58.1315955Z .................................................................................................... 6300/9484
2020-01-09T10:50:58.1315955Z .................................................................................................... 6300/9484
2020-01-09T10:51:15.8430348Z ......................................i..ii......................................................... 6400/9484
2020-01-09T10:51:36.4766710Z .................................................................................................... 6600/9484
2020-01-09T10:51:38.6534802Z .............i...................................................................................... 6700/9484
2020-01-09T10:51:40.9112754Z .................................................................................................... 6800/9484
2020-01-09T10:51:43.4031055Z .............i...................................................................................... 6900/9484
---
2020-01-09T10:53:19.1532026Z .................................................................................................... 7500/9484
2020-01-09T10:53:23.2053963Z .................................................................................................... 7600/9484
2020-01-09T10:53:28.5661434Z .................................................................................................... 7700/9484
2020-01-09T10:53:39.0494060Z .................................................................................................... 7800/9484
2020-01-09T10:53:47.5283181Z ..................................................iiii.............................................. 7900/9484
2020-01-09T10:54:02.4947662Z .................................................................................................... 8100/9484
2020-01-09T10:54:08.2974857Z .................................................................................................... 8200/9484
2020-01-09T10:54:23.6035011Z .................................................................................................... 8300/9484
2020-01-09T10:54:30.8553998Z .................................................................................................... 8400/9484
---
2020-01-09T10:56:51.9265607Z  finished in 7.017
2020-01-09T10:56:51.9447557Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:56:52.1625097Z 
2020-01-09T10:56:52.1625700Z running 166 tests
2020-01-09T10:56:55.3070114Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-09T10:56:57.5870091Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-09T10:56:57.5873448Z 
2020-01-09T10:56:57.5873601Z  finished in 5.642
2020-01-09T10:56:57.6061541Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:56:57.7724494Z 
---
2020-01-09T10:56:59.9932856Z  finished in 2.387
2020-01-09T10:57:00.0117952Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:57:00.1762854Z 
2020-01-09T10:57:00.1763238Z running 9 tests
2020-01-09T10:57:00.1764196Z iiiiiiiii
2020-01-09T10:57:00.1764670Z 
2020-01-09T10:57:00.1764714Z  finished in 0.164
2020-01-09T10:57:00.1958368Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:57:00.3950896Z 
---
2020-01-09T10:57:20.5698483Z  finished in 20.374
2020-01-09T10:57:20.5920929Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:57:20.8010952Z 
2020-01-09T10:57:20.8011181Z running 124 tests
2020-01-09T10:57:45.4317420Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-09T10:57:49.5425414Z .i.iii.....iiiiii.....ii
2020-01-09T10:57:49.5427662Z 
2020-01-09T10:57:49.5430326Z  finished in 28.950
2020-01-09T10:57:49.5439150Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:57:49.5439591Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-09T10:57:49.5439591Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-09T10:57:49.5670344Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-09T10:57:49.7372753Z 
2020-01-09T10:57:49.7372877Z running 63 tests
2020-01-09T10:58:22.3251068Z .......................FF.......F..FFFFFFFFFFF.................
2020-01-09T10:58:22.3258752Z 
2020-01-09T10:58:22.3259771Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-01-09T10:58:22.3275522Z 
2020-01-09T10:58:22.3275522Z 
2020-01-09T10:58:22.3276115Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2020-01-09T10:58:22.3276199Z status: exit code: 1
2020-01-09T10:58:22.3277114Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2020-01-09T10:58:22.3277489Z ------------------------------------------
2020-01-09T10:58:22.3277542Z 
2020-01-09T10:58:22.3277775Z ------------------------------------------
2020-01-09T10:58:22.3277821Z stderr:
2020-01-09T10:58:22.3277821Z stderr:
2020-01-09T10:58:22.3278047Z ------------------------------------------
2020-01-09T10:58:22.3278126Z error[E0432]: unresolved imports `rustc::lint::LateContext`, `rustc::lint::LintContext`, `rustc::lint::LateLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3278398Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:13:19
2020-01-09T10:58:22.3278916Z    |
2020-01-09T10:58:22.3278969Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray};
2020-01-09T10:58:22.3279025Z    |                   ^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3279348Z    |                   |            |                      no `LateLintPass` in `lint`
2020-01-09T10:58:22.3279406Z    |                   |            |                      help: a similar name exists in the module: `LintPass`
2020-01-09T10:58:22.3279477Z    |                   |            no `LintContext` in `lint`
2020-01-09T10:58:22.3279525Z    |                   no `LateContext` in `lint`
---
2020-01-09T10:58:22.3280337Z 
2020-01-09T10:58:22.3280397Z warning: unused import: `LintPass`
2020-01-09T10:58:22.3280660Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:13:45
2020-01-09T10:58:22.3280707Z    |
2020-01-09T10:58:22.3280772Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray};
2020-01-09T10:58:22.3280854Z 
2020-01-09T10:58:22.3280854Z 
2020-01-09T10:58:22.3281319Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3281592Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:1
2020-01-09T10:58:22.3281709Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3281758Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3281801Z    |
2020-01-09T10:58:22.3281844Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3282518Z 
2020-01-09T10:58:22.3282543Z 
2020-01-09T10:58:22.3282787Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-01-09T10:58:22.3282819Z 
2020-01-09T10:58:22.3283103Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2020-01-09T10:58:22.3283155Z status: exit code: 1
2020-01-09T10:58:22.3284033Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-01-09T10:58:22.3284370Z ------------------------------------------
2020-01-09T10:58:22.3284404Z 
2020-01-09T10:58:22.3284619Z ------------------------------------------
2020-01-09T10:58:22.3284681Z stderr:
2020-01-09T10:58:22.3284681Z stderr:
2020-01-09T10:58:22.3284892Z ------------------------------------------
2020-01-09T10:58:22.3284949Z error[E0432]: unresolved imports `rustc::lint::LateContext`, `rustc::lint::LintContext`, `rustc::lint::LateLintPass`
2020-01-09T10:58:22.3285237Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:12:19
2020-01-09T10:58:22.3285286Z    |
2020-01-09T10:58:22.3285332Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass};
2020-01-09T10:58:22.3285547Z    |                   |            |                      |
2020-01-09T10:58:22.3285609Z    |                   |            |                      no `LateLintPass` in `lint`
2020-01-09T10:58:22.3285684Z    |                   |            |                      help: a similar name exists in the module: `LintPass`
2020-01-09T10:58:22.3285738Z    |                   |            no `LintContext` in `lint`
---
2020-01-09T10:58:22.3286381Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3286421Z    |
2020-01-09T10:58:22.3286481Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3286512Z 
2020-01-09T10:58:22.3286874Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3287198Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:62:1
2020-01-09T10:58:22.3287294Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3287366Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3287414Z    |
2020-01-09T10:58:22.3287461Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3288186Z 
2020-01-09T10:58:22.3288213Z 
2020-01-09T10:58:22.3288456Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-01-09T10:58:22.3288509Z 
2020-01-09T10:58:22.3288823Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-01-09T10:58:22.3288880Z status: exit code: 1
2020-01-09T10:58:22.3289760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-01-09T10:58:22.3290123Z ------------------------------------------
2020-01-09T10:58:22.3290158Z 
2020-01-09T10:58:22.3290403Z ------------------------------------------
2020-01-09T10:58:22.3290468Z stderr:
2020-01-09T10:58:22.3290468Z stderr:
2020-01-09T10:58:22.3290701Z ------------------------------------------
2020-01-09T10:58:22.3290763Z error[E0432]: unresolved imports `rustc::lint::LateContext`, `rustc::lint::LintArray`, `rustc::lint::LateLintPass`, `rustc::lint::LintContext`
2020-01-09T10:58:22.3291062Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:14:19
2020-01-09T10:58:22.3291113Z    |
2020-01-09T10:58:22.3291165Z LL | use rustc::lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-01-09T10:58:22.3291302Z    |                   |                      |          |
2020-01-09T10:58:22.3291355Z    |                   |                      |          no `LateLintPass` in `lint`
2020-01-09T10:58:22.3291524Z    |                   |                      |          help: a similar name exists in the module: `LintPass`
2020-01-09T10:58:22.3291589Z    |                   |                      no `LintArray` in `lint`
2020-01-09T10:58:22.3291589Z    |                   |                      no `LintArray` in `lint`
2020-01-09T10:58:22.3291639Z    |                   no `LateContext` in `lint`
2020-01-09T10:58:22.3291689Z 
2020-01-09T10:58:22.3291733Z warning: unused `#[macro_use]` import
2020-01-09T10:58:22.3292041Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:4:1
2020-01-09T10:58:22.3292156Z LL | #[macro_use] extern crate rustc;
2020-01-09T10:58:22.3292288Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3292330Z    |
2020-01-09T10:58:22.3292395Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3292395Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3292427Z 
2020-01-09T10:58:22.3292471Z warning: unused import: `LintPass`
2020-01-09T10:58:22.3292777Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:14:32
2020-01-09T10:58:22.3292845Z    |
2020-01-09T10:58:22.3292904Z LL | use rustc::lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-01-09T10:58:22.3293004Z 
2020-01-09T10:58:22.3293004Z 
2020-01-09T10:58:22.3293377Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3293666Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:19:1
2020-01-09T10:58:22.3293777Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3293828Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3293902Z    |
2020-01-09T10:58:22.3293948Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3294655Z 
2020-01-09T10:58:22.3294690Z 
2020-01-09T10:58:22.3294947Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2020-01-09T10:58:22.3294981Z 
2020-01-09T10:58:22.3295299Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3295353Z status: exit code: 1
2020-01-09T10:58:22.3296253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2020-01-09T10:58:22.3296634Z ------------------------------------------
2020-01-09T10:58:22.3296669Z 
2020-01-09T10:58:22.3296903Z ------------------------------------------
2020-01-09T10:58:22.3296950Z stderr:
2020-01-09T10:58:22.3296950Z stderr:
2020-01-09T10:58:22.3297202Z ------------------------------------------
2020-01-09T10:58:22.3297265Z error[E0432]: unresolved imports `rustc::lint::LateContext`, `rustc::lint::LintContext`, `rustc::lint::LateLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3297559Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:19
2020-01-09T10:58:22.3297626Z    |
2020-01-09T10:58:22.3297677Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-01-09T10:58:22.3297737Z    |                   ^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3297958Z    |                   |            |                      no `LateLintPass` in `lint`
2020-01-09T10:58:22.3298017Z    |                   |            |                      help: a similar name exists in the module: `LintPass`
2020-01-09T10:58:22.3298092Z    |                   |            no `LintContext` in `lint`
2020-01-09T10:58:22.3298143Z    |                   no `LateContext` in `lint`
---
2020-01-09T10:58:22.3298908Z 
2020-01-09T10:58:22.3298952Z warning: unused import: `LintPass`
2020-01-09T10:58:22.3299239Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:45
2020-01-09T10:58:22.3299308Z    |
2020-01-09T10:58:22.3299359Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-01-09T10:58:22.3299443Z 
2020-01-09T10:58:22.3299443Z 
2020-01-09T10:58:22.3299828Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3300118Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-01-09T10:58:22.3300242Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3300293Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3300339Z    |
2020-01-09T10:58:22.3300401Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3301100Z 
2020-01-09T10:58:22.3301144Z 
2020-01-09T10:58:22.3301391Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2020-01-09T10:58:22.3301425Z 
2020-01-09T10:58:22.3301727Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3301801Z status: exit code: 1
2020-01-09T10:58:22.3302685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2020-01-09T10:58:22.3303042Z ------------------------------------------
2020-01-09T10:58:22.3303077Z 
2020-01-09T10:58:22.3303327Z ------------------------------------------
2020-01-09T10:58:22.3303374Z stderr:
2020-01-09T10:58:22.3303374Z stderr:
2020-01-09T10:58:22.3303606Z ------------------------------------------
2020-01-09T10:58:22.3303695Z error[E0432]: unresolved imports `rustc::lint::LateContext`, `rustc::lint::LintContext`, `rustc::lint::LateLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3303983Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:19
2020-01-09T10:58:22.3304034Z    |
2020-01-09T10:58:22.3304101Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-01-09T10:58:22.3304244Z    |                   ^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3304380Z    |                   |            |                      no `LateLintPass` in `lint`
2020-01-09T10:58:22.3304439Z    |                   |            |                      help: a similar name exists in the module: `LintPass`
2020-01-09T10:58:22.3304494Z    |                   |            no `LintContext` in `lint`
2020-01-09T10:58:22.3304620Z    |                   no `LateContext` in `lint`
---
2020-01-09T10:58:22.3305309Z 
2020-01-09T10:58:22.3305368Z warning: unused import: `LintPass`
2020-01-09T10:58:22.3305648Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:45
2020-01-09T10:58:22.3305699Z    |
2020-01-09T10:58:22.3305766Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-01-09T10:58:22.3305851Z 
2020-01-09T10:58:22.3305851Z 
2020-01-09T10:58:22.3306251Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3306544Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-01-09T10:58:22.3306655Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3306705Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3306750Z    |
2020-01-09T10:58:22.3306805Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3307510Z 
2020-01-09T10:58:22.3307536Z 
2020-01-09T10:58:22.3307803Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2020-01-09T10:58:22.3307847Z 
2020-01-09T10:58:22.3308139Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3308193Z status: exit code: 1
2020-01-09T10:58:22.3309087Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2020-01-09T10:58:22.3313546Z ------------------------------------------
2020-01-09T10:58:22.3313611Z 
2020-01-09T10:58:22.3313866Z ------------------------------------------
2020-01-09T10:58:22.3313915Z stderr:
2020-01-09T10:58:22.3313915Z stderr:
2020-01-09T10:58:22.3314130Z ------------------------------------------
2020-01-09T10:58:22.3314191Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3314613Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3314680Z    |
2020-01-09T10:58:22.3314747Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3314804Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3314931Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3315076Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3315124Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3315155Z 
---
2020-01-09T10:58:22.3315674Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3315716Z    |
2020-01-09T10:58:22.3315778Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3315809Z 
2020-01-09T10:58:22.3316182Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3316473Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3316561Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3316610Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3316679Z    |
2020-01-09T10:58:22.3316723Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3317387Z 
2020-01-09T10:58:22.3317412Z 
2020-01-09T10:58:22.3317645Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2020-01-09T10:58:22.3317677Z 
2020-01-09T10:58:22.3317970Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3318022Z status: exit code: 1
2020-01-09T10:58:22.3318896Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2020-01-09T10:58:22.3319235Z ------------------------------------------
2020-01-09T10:58:22.3319267Z 
2020-01-09T10:58:22.3319482Z ------------------------------------------
2020-01-09T10:58:22.3319527Z stderr:
2020-01-09T10:58:22.3319527Z stderr:
2020-01-09T10:58:22.3319757Z ------------------------------------------
2020-01-09T10:58:22.3319817Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3320085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3320154Z    |
2020-01-09T10:58:22.3320202Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3320258Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3320477Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3320528Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3320593Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3320623Z 
---
2020-01-09T10:58:22.3321202Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3321242Z    |
2020-01-09T10:58:22.3321284Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3321314Z 
2020-01-09T10:58:22.3321708Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3322009Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3322123Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3322175Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3322221Z    |
2020-01-09T10:58:22.3322286Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3322988Z 
2020-01-09T10:58:22.3323032Z 
2020-01-09T10:58:22.3323283Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-01-09T10:58:22.3323317Z 
2020-01-09T10:58:22.3323613Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3323685Z status: exit code: 1
2020-01-09T10:58:22.3324563Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2020-01-09T10:58:22.3327127Z ------------------------------------------
2020-01-09T10:58:22.3327166Z 
2020-01-09T10:58:22.3327438Z ------------------------------------------
2020-01-09T10:58:22.3327486Z stderr:
2020-01-09T10:58:22.3327486Z stderr:
2020-01-09T10:58:22.3327721Z ------------------------------------------
2020-01-09T10:58:22.3327818Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3328108Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3328161Z    |
2020-01-09T10:58:22.3328231Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3328290Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3328426Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3328480Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3328531Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3328580Z 
---
2020-01-09T10:58:22.3329234Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3329275Z    |
2020-01-09T10:58:22.3329340Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3329372Z 
2020-01-09T10:58:22.3329762Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3330213Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3330308Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3330379Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3330426Z    |
2020-01-09T10:58:22.3330472Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3331198Z 
2020-01-09T10:58:22.3331225Z 
2020-01-09T10:58:22.3331474Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2020-01-09T10:58:22.3331526Z 
2020-01-09T10:58:22.3331824Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3331887Z status: exit code: 1
2020-01-09T10:58:22.3333581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2020-01-09T10:58:22.3334016Z ------------------------------------------
2020-01-09T10:58:22.3334053Z 
2020-01-09T10:58:22.3334275Z ------------------------------------------
2020-01-09T10:58:22.3334339Z stderr:
2020-01-09T10:58:22.3334339Z stderr:
2020-01-09T10:58:22.3334842Z ------------------------------------------
2020-01-09T10:58:22.3334916Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3335660Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3335724Z    |
2020-01-09T10:58:22.3335782Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3335858Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3335964Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3336031Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3336080Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3336119Z 
---
2020-01-09T10:58:22.3336588Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3336628Z    |
2020-01-09T10:58:22.3336671Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3336829Z 
2020-01-09T10:58:22.3337265Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3337539Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3337645Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3337693Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3337753Z    |
2020-01-09T10:58:22.3337795Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3338563Z 
2020-01-09T10:58:22.3338604Z 
2020-01-09T10:58:22.3338835Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-01-09T10:58:22.3338877Z 
2020-01-09T10:58:22.3339154Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3339225Z status: exit code: 1
2020-01-09T10:58:22.3340074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2020-01-09T10:58:22.3340425Z ------------------------------------------
2020-01-09T10:58:22.3340477Z 
2020-01-09T10:58:22.3340696Z ------------------------------------------
2020-01-09T10:58:22.3340740Z stderr:
2020-01-09T10:58:22.3340740Z stderr:
2020-01-09T10:58:22.3340949Z ------------------------------------------
2020-01-09T10:58:22.3341026Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3341286Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3341352Z    |
2020-01-09T10:58:22.3341407Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3341465Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3341588Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3341645Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3341709Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3341740Z 
---
2020-01-09T10:58:22.3342182Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3342238Z    |
2020-01-09T10:58:22.3342281Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3342320Z 
2020-01-09T10:58:22.3342683Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3342947Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3343052Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3343189Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3343242Z    |
2020-01-09T10:58:22.3343285Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3345703Z 
2020-01-09T10:58:22.3345728Z 
2020-01-09T10:58:22.3346150Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2020-01-09T10:58:22.3346185Z 
2020-01-09T10:58:22.3346467Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3346520Z status: exit code: 1
2020-01-09T10:58:22.3347403Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2020-01-09T10:58:22.3347749Z ------------------------------------------
2020-01-09T10:58:22.3347782Z 
2020-01-09T10:58:22.3348001Z ------------------------------------------
2020-01-09T10:58:22.3348064Z stderr:
2020-01-09T10:58:22.3348064Z stderr:
2020-01-09T10:58:22.3348277Z ------------------------------------------
2020-01-09T10:58:22.3348346Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3349672Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3349740Z    |
2020-01-09T10:58:22.3349787Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3349868Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3349972Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3350055Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3350103Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3350132Z 
---
2020-01-09T10:58:22.3350626Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3350666Z    |
2020-01-09T10:58:22.3350709Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3350754Z 
2020-01-09T10:58:22.3351121Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3351393Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3351498Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3351555Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3351618Z    |
2020-01-09T10:58:22.3351661Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3352496Z 
2020-01-09T10:58:22.3352520Z 
2020-01-09T10:58:22.3352744Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2020-01-09T10:58:22.3352776Z 
2020-01-09T10:58:22.3353069Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-01-09T10:58:22.3353122Z status: exit code: 1
2020-01-09T10:58:22.3358095Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2020-01-09T10:58:22.3358676Z ------------------------------------------
2020-01-09T10:58:22.3358731Z 
2020-01-09T10:58:22.3358955Z ------------------------------------------
2020-01-09T10:58:22.3358999Z stderr:
2020-01-09T10:58:22.3358999Z stderr:
2020-01-09T10:58:22.3359229Z ------------------------------------------
2020-01-09T10:58:22.3359289Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::LintContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`
2020-01-09T10:58:22.3359564Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:19
2020-01-09T10:58:22.3359630Z    |
2020-01-09T10:58:22.3359680Z LL | use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-01-09T10:58:22.3359737Z    |                   ^^^^^^^^^^^^  ^^^^^^^^^^^            ^^^^^^^^^^^^^  ^^^^^^^^^ no `LintArray` in `lint`
2020-01-09T10:58:22.3359866Z    |                   |             |                      no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3359917Z    |                   |             no `LintContext` in `lint`
2020-01-09T10:58:22.3359981Z    |                   no `EarlyContext` in `lint`
2020-01-09T10:58:22.3360011Z 
---
2020-01-09T10:58:22.3361319Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3361378Z    |
2020-01-09T10:58:22.3361420Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3361450Z 
2020-01-09T10:58:22.3362262Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3362642Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-01-09T10:58:22.3362750Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3363144Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3363205Z    |
2020-01-09T10:58:22.3363269Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3364000Z 
2020-01-09T10:58:22.3364024Z 
2020-01-09T10:58:22.3364273Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2020-01-09T10:58:22.3364306Z 
2020-01-09T10:58:22.3365127Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-01-09T10:58:22.3365352Z status: exit code: 1
2020-01-09T10:58:22.3366860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2020-01-09T10:58:22.3367413Z ------------------------------------------
2020-01-09T10:58:22.3367449Z 
2020-01-09T10:58:22.3367686Z ------------------------------------------
2020-01-09T10:58:22.3367731Z stderr:
2020-01-09T10:58:22.3367731Z stderr:
2020-01-09T10:58:22.3367952Z ------------------------------------------
2020-01-09T10:58:22.3368031Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`, `rustc::lint::LintContext`
2020-01-09T10:58:22.3368294Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:19
2020-01-09T10:58:22.3368343Z    |
2020-01-09T10:58:22.3368410Z LL | use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-01-09T10:58:22.3368525Z    |                   |             |              |
2020-01-09T10:58:22.3368591Z    |                   |             |              no `LintArray` in `lint`
2020-01-09T10:58:22.3368643Z    |                   |             no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3368690Z    |                   no `EarlyContext` in `lint`
---
2020-01-09T10:58:22.3369187Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3369226Z    |
2020-01-09T10:58:22.3369287Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3369317Z 
2020-01-09T10:58:22.3369677Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3370004Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-01-09T10:58:22.3370097Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3370166Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3370213Z    |
2020-01-09T10:58:22.3370259Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3370979Z 
2020-01-09T10:58:22.3371005Z 
2020-01-09T10:58:22.3371244Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-09T10:58:22.3371277Z 
2020-01-09T10:58:22.3371591Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-01-09T10:58:22.3371654Z status: exit code: 1
2020-01-09T10:58:22.3372617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
2020-01-09T10:58:22.3373015Z ------------------------------------------
2020-01-09T10:58:22.3373052Z 
2020-01-09T10:58:22.3373288Z ------------------------------------------
2020-01-09T10:58:22.3373334Z stderr:
2020-01-09T10:58:22.3373334Z stderr:
2020-01-09T10:58:22.3373703Z ------------------------------------------
2020-01-09T10:58:22.3373767Z error[E0432]: unresolved imports `rustc::lint::EarlyContext`, `rustc::lint::EarlyLintPass`, `rustc::lint::LintArray`, `rustc::lint::LintContext`
2020-01-09T10:58:22.3374043Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:19
2020-01-09T10:58:22.3374116Z    |
2020-01-09T10:58:22.3374177Z LL | use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-01-09T10:58:22.3374308Z    |                   |             |              |
2020-01-09T10:58:22.3374359Z    |                   |             |              no `LintArray` in `lint`
2020-01-09T10:58:22.3374411Z    |                   |             no `EarlyLintPass` in `lint`
2020-01-09T10:58:22.3374479Z    |                   no `EarlyContext` in `lint`
---
2020-01-09T10:58:22.3374991Z    | ^^^^^^^^^^^^
2020-01-09T10:58:22.3375049Z    |
2020-01-09T10:58:22.3375095Z    = note: `#[warn(unused_imports)]` on by default
2020-01-09T10:58:22.3375126Z 
2020-01-09T10:58:22.3375520Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-09T10:58:22.3375806Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-01-09T10:58:22.3375916Z LL | #[plugin_registrar]
2020-01-09T10:58:22.3375967Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-09T10:58:22.3376013Z    |
2020-01-09T10:58:22.3376076Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-09T10:58:22.3381815Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-09T10:58:22.3381892Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-09T10:58:22.3381924Z 
2020-01-09T10:58:22.3381949Z 
2020-01-09T10:58:22.3383631Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-09T10:58:22.3383973Z 
2020-01-09T10:58:22.3384001Z 
2020-01-09T10:58:22.3384071Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-09T10:58:22.3384121Z Build completed unsuccessfully in 1:00:15
2020-01-09T10:58:22.3384121Z Build completed unsuccessfully in 1:00:15
2020-01-09T10:58:22.3384166Z == clock drift check ==
2020-01-09T10:58:22.3384227Z   local time: Thu Jan  9 10:58:22 UTC 2020
2020-01-09T10:58:22.6067277Z   network time: Thu, 09 Jan 2020 10:58:22 GMT
2020-01-09T10:58:22.6072607Z == end clock drift check ==
2020-01-09T10:58:23.6008860Z 
2020-01-09T10:58:23.6110964Z ##[error]Bash exited with code '1'.
2020-01-09T10:58:23.6145958Z ##[section]Starting: Checkout
2020-01-09T10:58:23.6147702Z ==============================================================================
2020-01-09T10:58:23.6147759Z Task         : Get sources
2020-01-09T10:58:23.6147824Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
