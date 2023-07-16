plain
2020-01-05T03:24:25.1766579Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T03:24:25.1964195Z ##[command]git config gc.auto 0
2020-01-05T03:24:25.2011847Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T03:24:25.2057768Z ##[command]git config --get-all http.proxy
2020-01-05T03:24:25.2182393Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67886/merge:refs/remotes/pull/67886/merge
---
2020-01-05T04:11:17.6058157Z .................................................................................................... 1500/9475
2020-01-05T04:11:22.7686464Z .................................................................................................... 1600/9475
2020-01-05T04:11:27.1071630Z .................................................................................................... 1700/9475
2020-01-05T04:11:35.4425121Z .................................................................................................... 1800/9475
2020-01-05T04:11:42.5262087Z i................................................................................................... 1900/9475
2020-01-05T04:11:48.3455997Z ........................................................................................iiiii....... 2000/9475
2020-01-05T04:12:07.2770741Z .................................................................................................... 2200/9475
2020-01-05T04:12:09.4051351Z .................................................................................................... 2300/9475
2020-01-05T04:12:11.5504560Z .................................................................................................... 2400/9475
2020-01-05T04:12:16.7720733Z .................................................................................................... 2500/9475
---
2020-01-05T04:14:54.8610776Z ....................i...............i............................................................... 4900/9475
2020-01-05T04:15:03.7120439Z .................................................................................................... 5000/9475
2020-01-05T04:15:08.9852919Z .................................................................i.................................. 5100/9475
2020-01-05T04:15:16.1596721Z .................................................................................................... 5200/9475
2020-01-05T04:15:22.9487758Z ................................ii.ii...........i................................................... 5300/9475
2020-01-05T04:15:31.4293483Z .................................................................................................... 5500/9475
2020-01-05T04:15:40.0102454Z .................................................................................................... 5600/9475
2020-01-05T04:15:46.4123682Z ................i................................................................................... 5700/9475
2020-01-05T04:15:51.8320921Z .................................................................................................... 5800/9475
2020-01-05T04:15:51.8320921Z .................................................................................................... 5800/9475
2020-01-05T04:16:01.4127397Z .................................................................................................... 5900/9475
2020-01-05T04:16:11.5003329Z .....ii...i..ii...........i......................................................................... 6000/9475
2020-01-05T04:16:26.5316656Z .................................................................................................... 6200/9475
2020-01-05T04:16:33.2664647Z .................................................................................................... 6300/9475
2020-01-05T04:16:33.2664647Z .................................................................................................... 6300/9475
2020-01-05T04:16:54.1702885Z ................................i..ii............................................................... 6400/9475
2020-01-05T04:17:10.8689346Z .................................................................................................... 6600/9475
2020-01-05T04:17:12.6256267Z .......i............................................................................................ 6700/9475
2020-01-05T04:17:14.5544684Z .................................................................................................... 6800/9475
2020-01-05T04:17:16.7978907Z ........i........................................................................................... 6900/9475
---
2020-01-05T04:18:41.3088864Z .................................................................................................... 7500/9475
2020-01-05T04:18:44.8843298Z .................................................................................................... 7600/9475
2020-01-05T04:18:49.4758333Z .................................................................................................... 7700/9475
2020-01-05T04:18:59.0177559Z .................................................................................................... 7800/9475
2020-01-05T04:19:06.3224167Z ...........................................iiii..................................................... 7900/9475
2020-01-05T04:19:20.0810936Z .................................................................................................... 8100/9475
2020-01-05T04:19:27.5362695Z .................................................................................................... 8200/9475
2020-01-05T04:19:40.2680962Z .................................................................................................... 8300/9475
2020-01-05T04:19:47.1596781Z .................................................................................................... 8400/9475
---
2020-01-05T04:21:53.6675726Z  finished in 6.308
2020-01-05T04:21:53.6769714Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:21:53.8245219Z 
2020-01-05T04:21:53.8245369Z running 166 tests
2020-01-05T04:21:56.7271624Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-05T04:21:58.8176031Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-05T04:21:58.8176543Z 
2020-01-05T04:21:58.8182866Z  finished in 5.141
2020-01-05T04:21:58.8357380Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:21:58.9820130Z 
---
2020-01-05T04:22:00.9258507Z  finished in 2.090
2020-01-05T04:22:00.9423143Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:22:01.1961259Z 
2020-01-05T04:22:01.1962626Z running 9 tests
2020-01-05T04:22:01.1965694Z iiiiiiiii
2020-01-05T04:22:01.1974630Z 
2020-01-05T04:22:01.1974673Z  finished in 0.254
2020-01-05T04:22:01.2150609Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:22:01.3682321Z 
---
2020-01-05T04:22:19.7735935Z  finished in 18.558
2020-01-05T04:22:19.7940217Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:22:19.9507542Z 
2020-01-05T04:22:19.9508069Z running 124 tests
2020-01-05T04:22:41.3890786Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-05T04:22:44.9430833Z .i.iii.....iiiiii.....ii
2020-01-05T04:22:44.9433687Z 
2020-01-05T04:22:44.9434116Z  finished in 25.150
2020-01-05T04:22:44.9456020Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:22:44.9456678Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-05T04:22:44.9456678Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-01-05T04:22:44.9626953Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T04:22:45.1015968Z 
2020-01-05T04:22:45.1016510Z running 64 tests
2020-01-05T04:23:17.8269923Z .......................F.F.......F.FF...........................
2020-01-05T04:23:17.8270337Z 
2020-01-05T04:23:17.8270610Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-01-05T04:23:17.8270642Z 
2020-01-05T04:23:17.8270642Z 
2020-01-05T04:23:17.8270902Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2020-01-05T04:23:17.8270967Z status: exit code: 1
2020-01-05T04:23:17.8271694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2020-01-05T04:23:17.8271993Z ------------------------------------------
2020-01-05T04:23:17.8272036Z 
2020-01-05T04:23:17.8272226Z ------------------------------------------
2020-01-05T04:23:17.8272265Z stderr:
---
2020-01-05T04:23:17.8273001Z    |
2020-01-05T04:23:17.8273036Z LL | use syntax::ast::Crate;
2020-01-05T04:23:17.8273089Z    |
2020-01-05T04:23:17.8273111Z 
2020-01-05T04:23:17.8273594Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-05T04:23:17.8273884Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:1
2020-01-05T04:23:17.8273963Z LL | #[plugin_registrar]
2020-01-05T04:23:17.8274019Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-05T04:23:17.8274064Z    |
2020-01-05T04:23:17.8274100Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-05T04:23:17.8275062Z 
2020-01-05T04:23:17.8275082Z 
2020-01-05T04:23:17.8275269Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-01-05T04:23:17.8275311Z 
2020-01-05T04:23:17.8275551Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2020-01-05T04:23:17.8275594Z status: exit code: 1
2020-01-05T04:23:17.8276304Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-01-05T04:23:17.8276718Z ------------------------------------------
2020-01-05T04:23:17.8276746Z 
2020-01-05T04:23:17.8276922Z ------------------------------------------
2020-01-05T04:23:17.8276974Z stderr:
2020-01-05T04:23:17.8276974Z stderr:
2020-01-05T04:23:17.8277159Z ------------------------------------------
2020-01-05T04:23:17.8277200Z error[E0412]: cannot find type `Crate` in module `hir`
2020-01-05T04:23:17.8277426Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:28:70
2020-01-05T04:23:17.8277472Z    |
2020-01-05T04:23:17.8277512Z LL |               fn check_crate(&mut self, cx: &LateContext, krate: &hir::Crate) {
2020-01-05T04:23:17.8277573Z    |                                                                        ^^^^^ not found in `hir`
2020-01-05T04:23:17.8277613Z ...
2020-01-05T04:23:17.8277647Z LL | / fake_lint_pass! {
2020-01-05T04:23:17.8277697Z LL | |     PassOkay,
2020-01-05T04:23:17.8277733Z LL | |     Symbol::intern("rustc_crate_okay")
2020-01-05T04:23:17.8277964Z    | |_- in this macro invocation
2020-01-05T04:23:17.8278000Z    |
2020-01-05T04:23:17.8278039Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8278085Z    |
---
2020-01-05T04:23:17.8278499Z    |
2020-01-05T04:23:17.8278539Z LL |               fn check_crate(&mut self, cx: &LateContext, krate: &hir::Crate) {
2020-01-05T04:23:17.8278598Z    |                                                                        ^^^^^ not found in `hir`
2020-01-05T04:23:17.8278634Z ...
2020-01-05T04:23:17.8278668Z LL | / fake_lint_pass! {
2020-01-05T04:23:17.8278718Z LL | |     PassRedBlue,
2020-01-05T04:23:17.8278757Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2020-01-05T04:23:17.8278984Z    | |_- in this macro invocation
2020-01-05T04:23:17.8279019Z    |
2020-01-05T04:23:17.8279135Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8279192Z    |
---
2020-01-05T04:23:17.8279611Z    |
2020-01-05T04:23:17.8279665Z LL |               fn check_crate(&mut self, cx: &LateContext, krate: &hir::Crate) {
2020-01-05T04:23:17.8279709Z    |                                                                        ^^^^^ not found in `hir`
2020-01-05T04:23:17.8279745Z ...
2020-01-05T04:23:17.8279795Z LL | / fake_lint_pass! {
2020-01-05T04:23:17.8279830Z LL | |     PassGreyGreen,
2020-01-05T04:23:17.8279869Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2020-01-05T04:23:17.8280102Z    | |_- in this macro invocation
2020-01-05T04:23:17.8280138Z    |
2020-01-05T04:23:17.8280176Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8280227Z    |
---
2020-01-05T04:23:17.8280788Z    | ^^^^^^^^^^^^
2020-01-05T04:23:17.8280820Z    |
2020-01-05T04:23:17.8280873Z    = note: `#[warn(unused_imports)]` on by default
2020-01-05T04:23:17.8280896Z 
2020-01-05T04:23:17.8281186Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-05T04:23:17.8281471Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:62:1
2020-01-05T04:23:17.8281551Z LL | #[plugin_registrar]
2020-01-05T04:23:17.8281592Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-05T04:23:17.8281645Z    |
2020-01-05T04:23:17.8281688Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-05T04:23:17.8282437Z 
2020-01-05T04:23:17.8282458Z 
2020-01-05T04:23:17.8282648Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-01-05T04:23:17.8282674Z 
2020-01-05T04:23:17.8282932Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-01-05T04:23:17.8282977Z status: exit code: 1
2020-01-05T04:23:17.8283705Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-01-05T04:23:17.8283994Z ------------------------------------------
2020-01-05T04:23:17.8284022Z 
2020-01-05T04:23:17.8284204Z ------------------------------------------
2020-01-05T04:23:17.8284239Z stderr:
2020-01-05T04:23:17.8284239Z stderr:
2020-01-05T04:23:17.8284435Z ------------------------------------------
2020-01-05T04:23:17.8284553Z error[E0432]: unresolved import `rustc::hir::Node`
2020-01-05T04:23:17.8284800Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:10:36
2020-01-05T04:23:17.8284858Z    |
2020-01-05T04:23:17.8284896Z LL | use rustc::hir::{self, intravisit, Node};
2020-01-05T04:23:17.8284944Z    |                                    ^^^^ no `Node` in `hir`
2020-01-05T04:23:17.8285022Z error[E0412]: cannot find type `FnDecl` in module `hir`
2020-01-05T04:23:17.8285022Z error[E0412]: cannot find type `FnDecl` in module `hir`
2020-01-05T04:23:17.8285392Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:34:31
2020-01-05T04:23:17.8285620Z LL |                 _: &'tcx hir::FnDecl,
2020-01-05T04:23:17.8285661Z    |                               ^^^^^^ not found in `hir`
2020-01-05T04:23:17.8285695Z    |
2020-01-05T04:23:17.8285748Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8285748Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8285783Z    |
2020-01-05T04:23:17.8285817Z LL | use syntax::ast::FnDecl;
2020-01-05T04:23:17.8285872Z    |
2020-01-05T04:23:17.8285895Z 
2020-01-05T04:23:17.8285930Z error[E0412]: cannot find type `Body` in module `hir`
2020-01-05T04:23:17.8286142Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:35:31
2020-01-05T04:23:17.8286474Z LL |                 _: &'tcx hir::Body,
2020-01-05T04:23:17.8286514Z    |                               ^^^^ not found in `hir`
2020-01-05T04:23:17.8286562Z    |
2020-01-05T04:23:17.8286601Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8286601Z help: possible candidate is found in another module, you can import it into scope
2020-01-05T04:23:17.8286637Z    |
2020-01-05T04:23:17.8286686Z LL | use rustc::mir::Body;
2020-01-05T04:23:17.8286717Z    |
2020-01-05T04:23:17.8286738Z 
2020-01-05T04:23:17.8286773Z error[E0412]: cannot find type `HirId` in module `hir`
2020-01-05T04:23:17.8287001Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:37:26
2020-01-05T04:23:17.8287072Z LL |                 id: hir::HirId) {
2020-01-05T04:23:17.8287132Z    |                          ^^^^^ not found in `hir`
2020-01-05T04:23:17.8287156Z 
2020-01-05T04:23:17.8287156Z 
2020-01-05T04:23:17.8287441Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-05T04:23:17.8287684Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:16:1
2020-01-05T04:23:17.8287759Z LL | #[plugin_registrar]
2020-01-05T04:23:17.8287814Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-05T04:23:17.8287848Z    |
2020-01-05T04:23:17.8287884Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-05T04:23:17.8288471Z 
2020-01-05T04:23:17.8288491Z 
2020-01-05T04:23:17.8288701Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2020-01-05T04:23:17.8288728Z 
2020-01-05T04:23:17.8288963Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-01-05T04:23:17.8289012Z status: exit code: 1
2020-01-05T04:23:17.8289817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2020-01-05T04:23:17.8290127Z ------------------------------------------
2020-01-05T04:23:17.8290161Z 
2020-01-05T04:23:17.8290355Z ------------------------------------------
2020-01-05T04:23:17.8290391Z stderr:
2020-01-05T04:23:17.8290391Z stderr:
2020-01-05T04:23:17.8290566Z ------------------------------------------
2020-01-05T04:23:17.8290606Z error[E0412]: cannot find type `Item` in module `hir`
2020-01-05T04:23:17.8290837Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:22:58
2020-01-05T04:23:17.8290876Z    |
2020-01-05T04:23:17.8290915Z LL |     fn check_item(&mut self, cx: &LateContext, it: &hir::Item) {
2020-01-05T04:23:17.8290972Z    |                                                          ^^^^ not found in `hir`
2020-01-05T04:23:17.8291007Z    |
2020-01-05T04:23:17.8291226Z help: there is an enum variant `rustc::ty::InstanceDef::Item`; try using the variant's enum
2020-01-05T04:23:17.8291327Z LL |     fn check_item(&mut self, cx: &LateContext, it: &rustc::ty::InstanceDef) {
2020-01-05T04:23:17.8291369Z    |                                                     ^^^^^^^^^^^^^^^^^^^^^^
2020-01-05T04:23:17.8291486Z 
2020-01-05T04:23:17.8291486Z 
2020-01-05T04:23:17.8291790Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-05T04:23:17.8292191Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-01-05T04:23:17.8292267Z LL | #[plugin_registrar]
2020-01-05T04:23:17.8292306Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-05T04:23:17.8292358Z    |
2020-01-05T04:23:17.8292394Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-05T04:23:17.8292962Z 
2020-01-05T04:23:17.8292982Z 
2020-01-05T04:23:17.8293173Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2020-01-05T04:23:17.8293208Z 
2020-01-05T04:23:17.8293464Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-01-05T04:23:17.8293509Z status: exit code: 1
2020-01-05T04:23:17.8294243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2020-01-05T04:23:17.8294534Z ------------------------------------------
2020-01-05T04:23:17.8294561Z 
2020-01-05T04:23:17.8294745Z ------------------------------------------
2020-01-05T04:23:17.8294782Z stderr:
2020-01-05T04:23:17.8294782Z stderr:
2020-01-05T04:23:17.8294977Z ------------------------------------------
2020-01-05T04:23:17.8295018Z error[E0412]: cannot find type `Item` in module `hir`
2020-01-05T04:23:17.8295231Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:22:58
2020-01-05T04:23:17.8295288Z    |
2020-01-05T04:23:17.8295327Z LL |     fn check_item(&mut self, cx: &LateContext, it: &hir::Item) {
2020-01-05T04:23:17.8295371Z    |                                                          ^^^^ not found in `hir`
2020-01-05T04:23:17.8295423Z    |
2020-01-05T04:23:17.8295724Z help: there is an enum variant `rustc::ty::InstanceDef::Item`; try using the variant's enum
2020-01-05T04:23:17.8295827Z LL |     fn check_item(&mut self, cx: &LateContext, it: &rustc::ty::InstanceDef) {
2020-01-05T04:23:17.8295878Z    |                                                     ^^^^^^^^^^^^^^^^^^^^^^
2020-01-05T04:23:17.8295904Z 
2020-01-05T04:23:17.8295904Z 
2020-01-05T04:23:17.8296237Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-05T04:23:17.8296469Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-01-05T04:23:17.8296562Z LL | #[plugin_registrar]
2020-01-05T04:23:17.8296601Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-05T04:23:17.8296637Z    |
2020-01-05T04:23:17.8296688Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-05T04:23:17.8299159Z test result: FAILED. 59 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-05T04:23:17.8299190Z 
2020-01-05T04:23:17.8299212Z 
2020-01-05T04:23:17.8299251Z 
2020-01-05T04:23:17.8300715Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-05T04:23:17.8300957Z 
2020-01-05T04:23:17.8300981Z 
2020-01-05T04:23:17.8301227Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-05T04:23:17.8301295Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T04:23:17.8301295Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T04:23:17.8301408Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-05T04:23:17.8301450Z Build completed unsuccessfully in 0:52:49
2020-01-05T04:23:17.8320108Z == clock drift check ==
2020-01-05T04:23:17.8333159Z   local time: Sun Jan  5 04:23:17 UTC 2020
2020-01-05T04:23:18.1030732Z   network time: Sun, 05 Jan 2020 04:23:18 GMT
2020-01-05T04:23:18.1030822Z == end clock drift check ==
2020-01-05T04:23:19.5241673Z 
2020-01-05T04:23:19.5335637Z ##[error]Bash exited with code '1'.
2020-01-05T04:23:19.5393989Z ##[section]Starting: Checkout
2020-01-05T04:23:19.5396425Z ==============================================================================
2020-01-05T04:23:19.5396474Z Task         : Get sources
2020-01-05T04:23:19.5396517Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
