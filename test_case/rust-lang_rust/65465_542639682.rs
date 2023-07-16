plain
2019-10-16T09:21:29.1401196Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T09:21:29.1561010Z ##[command]git config gc.auto 0
2019-10-16T09:21:29.1634519Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T09:21:29.9781119Z ##[command]git config --get-all http.proxy
2019-10-16T09:21:29.9784468Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65465/merge:refs/remotes/pull/65465/merge
---
2019-10-16T10:24:04.6192790Z .................................................................................................... 1600/9196
2019-10-16T10:24:10.1374292Z .................................................................................................... 1700/9196
2019-10-16T10:24:23.5390597Z .............................i...............i...................................................... 1800/9196
2019-10-16T10:24:31.3736930Z .................................................................................................... 1900/9196
2019-10-16T10:24:46.1301411Z ...................iiiii............................................................................ 2000/9196
2019-10-16T10:24:57.0793263Z .................................................................................................... 2200/9196
2019-10-16T10:24:59.7900826Z .................................................................................................... 2300/9196
2019-10-16T10:25:05.5200965Z .................................................................................................... 2400/9196
2019-10-16T10:25:28.3168737Z .................................................................................................... 2500/9196
---
2019-10-16T10:28:34.3216948Z ......................i...............i............................................................. 4800/9196
2019-10-16T10:28:46.6468963Z .................................................................................................... 4900/9196
2019-10-16T10:28:53.3116344Z .................................................................................................... 5000/9196
2019-10-16T10:29:04.3966416Z .................................................................................................... 5100/9196
2019-10-16T10:29:11.0005403Z ......................ii.ii......................................................................... 5200/9196
2019-10-16T10:29:22.0838582Z .................................................................................................... 5400/9196
2019-10-16T10:29:32.6626692Z ........................................................................................i........... 5500/9196
2019-10-16T10:29:41.3308432Z .................................................................................................... 5600/9196
2019-10-16T10:29:46.8472883Z .................................................................................................... 5700/9196
2019-10-16T10:29:46.8472883Z .................................................................................................... 5700/9196
2019-10-16T10:29:58.1944690Z .....................................................................................ii...i..ii..... 5800/9196
2019-10-16T10:30:26.1528383Z .................................................................................................... 6000/9196
2019-10-16T10:30:36.6518741Z .................................................................................................... 6100/9196
2019-10-16T10:30:42.0314646Z .................................................................................................... 6200/9196
2019-10-16T10:30:42.0314646Z .................................................................................................... 6200/9196
2019-10-16T10:30:56.7061464Z .......i..ii........................................................................................ 6300/9196
2019-10-16T10:31:17.3417138Z ...................................................................i................................ 6500/9196
2019-10-16T10:31:19.6530137Z .................................................................................................... 6600/9196
2019-10-16T10:31:22.3916321Z .........................................i.......................................................... 6700/9196
2019-10-16T10:31:26.5879846Z .................................................................................................... 6800/9196
---
2019-10-16T10:36:11.8424218Z  finished in 5.639
2019-10-16T10:36:11.8645328Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:36:12.0303816Z 
2019-10-16T10:36:12.0305006Z running 153 tests
2019-10-16T10:36:15.4470890Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-16T10:36:17.5197096Z i..iiii..............i.........iii.i.........ii......
2019-10-16T10:36:17.5197638Z 
2019-10-16T10:36:17.5202163Z  finished in 5.655
2019-10-16T10:36:17.5404403Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:36:17.6997041Z 
---
2019-10-16T10:36:19.9484330Z  finished in 2.407
2019-10-16T10:36:19.9712752Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:36:20.1430525Z 
2019-10-16T10:36:20.1431603Z running 9 tests
2019-10-16T10:36:20.1433447Z iiiiiiiii
2019-10-16T10:36:20.1433896Z 
2019-10-16T10:36:20.1434475Z  finished in 0.172
2019-10-16T10:36:20.1629036Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:36:20.3282020Z 
---
2019-10-16T10:36:39.1769928Z  finished in 19.014
2019-10-16T10:36:39.1978226Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:36:39.3626951Z 
2019-10-16T10:36:39.3627979Z running 123 tests
2019-10-16T10:37:04.6144589Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-16T10:37:09.5412443Z i.i.i......iii.i.....ii
2019-10-16T10:37:09.5413531Z 
2019-10-16T10:37:09.5419728Z  finished in 30.344
2019-10-16T10:37:09.5433040Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:37:09.5433877Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-16T10:37:09.5433877Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-16T10:37:09.5672044Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T10:37:09.7342157Z 
2019-10-16T10:37:09.7342464Z running 69 tests
2019-10-16T10:37:47.5149733Z F............F..................F.................F...FFFFFF...F.....
2019-10-16T10:37:47.5173766Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-16T10:37:47.5175618Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-16T10:37:47.5176272Z 
2019-10-16T10:37:47.5177734Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-16T10:37:47.5177734Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-16T10:37:47.5179965Z 
2019-10-16T10:37:47.5180929Z error: test compilation failed although it shouldn't!
2019-10-16T10:37:47.5181282Z status: exit code: 1
2019-10-16T10:37:47.5184728Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-16T10:37:47.5185292Z ------------------------------------------
2019-10-16T10:37:47.5185330Z 
2019-10-16T10:37:47.5185553Z ------------------------------------------
2019-10-16T10:37:47.5185600Z stderr:
2019-10-16T10:37:47.5185600Z stderr:
2019-10-16T10:37:47.5185834Z ------------------------------------------
2019-10-16T10:37:47.5185886Z error[E0432]: unresolved import `syntax::parse::attr`
2019-10-16T10:37:47.5186134Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:21:20
2019-10-16T10:37:47.5186204Z    |
2019-10-16T10:37:47.5186252Z LL | use syntax::parse::attr::*;
2019-10-16T10:37:47.5186301Z    |                    ^^^^ could not find `attr` in `parse`
2019-10-16T10:37:47.5186396Z error[E0624]: method `parse_attribute` is private
2019-10-16T10:37:47.5186642Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:61:11
2019-10-16T10:37:47.5186853Z    |
2019-10-16T10:37:47.5186853Z    |
2019-10-16T10:37:47.5186910Z LL |         p.parse_attribute(true)
2019-10-16T10:37:47.5186988Z 
2019-10-16T10:37:47.5187026Z error: aborting due to 2 previous errors
2019-10-16T10:37:47.5187069Z 
2019-10-16T10:37:47.5187108Z Some errors have detailed explanations: E0432, E0624.
2019-10-16T10:37:47.5187108Z Some errors have detailed explanations: E0432, E0624.
2019-10-16T10:37:47.5187489Z For more information about an error, try `rustc --explain E0432`.
2019-10-16T10:37:47.5187680Z 
2019-10-16T10:37:47.5187874Z ------------------------------------------
2019-10-16T10:37:47.5187899Z 
2019-10-16T10:37:47.5187920Z 
2019-10-16T10:37:47.5188099Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-16T10:37:47.5188143Z 
2019-10-16T10:37:47.5188372Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-16T10:37:47.5188417Z status: exit code: 1
2019-10-16T10:37:47.5189057Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-16T10:37:47.5189576Z ------------------------------------------
2019-10-16T10:37:47.5189604Z 
2019-10-16T10:37:47.5189781Z ------------------------------------------
2019-10-16T10:37:47.5189838Z stderr:
2019-10-16T10:37:47.5189838Z stderr:
2019-10-16T10:37:47.5190012Z ------------------------------------------
2019-10-16T10:37:47.5190056Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5190281Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:13
2019-10-16T10:37:47.5190321Z    |
2019-10-16T10:37:47.5190358Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-16T10:37:47.5190666Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5190736Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5190968Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:20:32
2019-10-16T10:37:47.5191025Z    |
2019-10-16T10:37:47.5191025Z    |
2019-10-16T10:37:47.5191063Z LL |         Symbol::intern("mac"), SyntaxExtension::dummy_bang(reg.sess.edition())
2019-10-16T10:37:47.5191108Z    |                                ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5191157Z 
2019-10-16T10:37:47.5191542Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5191759Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-16T10:37:47.5191850Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5191896Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5191954Z    |
2019-10-16T10:37:47.5191990Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5192531Z 
2019-10-16T10:37:47.5192551Z 
2019-10-16T10:37:47.5192899Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-10-16T10:37:47.5192924Z 
2019-10-16T10:37:47.5193575Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-10-16T10:37:47.5193635Z status: exit code: 1
2019-10-16T10:37:47.5194405Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-10-16T10:37:47.5194746Z ------------------------------------------
2019-10-16T10:37:47.5194798Z 
2019-10-16T10:37:47.5195016Z ------------------------------------------
2019-10-16T10:37:47.5195061Z stderr:
2019-10-16T10:37:47.5195061Z stderr:
2019-10-16T10:37:47.5195291Z ------------------------------------------
2019-10-16T10:37:47.5195344Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5195599Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:11:13
2019-10-16T10:37:47.5195711Z LL | use syntax::ext::base::*;
2019-10-16T10:37:47.5195900Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5195931Z 
2019-10-16T10:37:47.5195931Z 
2019-10-16T10:37:47.5195995Z warning: unused import: `rustc::hir::map as hir_map`
2019-10-16T10:37:47.5196287Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:5
2019-10-16T10:37:47.5196335Z    |
2019-10-16T10:37:47.5196397Z LL | use rustc::hir::map as hir_map;
2019-10-16T10:37:47.5196488Z    |
2019-10-16T10:37:47.5196532Z    = note: `#[warn(unused_imports)]` on by default
2019-10-16T10:37:47.5196582Z 
2019-10-16T10:37:47.5196624Z warning: unused import: `rustc::ty`
2019-10-16T10:37:47.5196624Z warning: unused import: `rustc::ty`
2019-10-16T10:37:47.5197018Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:5
2019-10-16T10:37:47.5197109Z LL | use rustc::ty;
2019-10-16T10:37:47.5197307Z    |     ^^^^^^^^^
2019-10-16T10:37:47.5197329Z 
2019-10-16T10:37:47.5197379Z warning: unused import: `ast`
2019-10-16T10:37:47.5197379Z warning: unused import: `ast`
2019-10-16T10:37:47.5197649Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:14
2019-10-16T10:37:47.5197699Z    |
2019-10-16T10:37:47.5197751Z LL | use syntax::{ast, source_map};
2019-10-16T10:37:47.5197808Z 
2019-10-16T10:37:47.5197808Z 
2019-10-16T10:37:47.5198295Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5198507Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:23:1
2019-10-16T10:37:47.5198596Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5198633Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5198666Z    |
2019-10-16T10:37:47.5198717Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5199451Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-16T10:37:47.5199477Z 
2019-10-16T10:37:47.5199648Z error: test compilation failed although it shouldn't!
2019-10-16T10:37:47.5199684Z status: exit code: 1
2019-10-16T10:37:47.5200451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-16T10:37:47.5200729Z ------------------------------------------
2019-10-16T10:37:47.5200755Z 
2019-10-16T10:37:47.5200925Z ------------------------------------------
2019-10-16T10:37:47.5200979Z stderr:
2019-10-16T10:37:47.5200979Z stderr:
2019-10-16T10:37:47.5201150Z ------------------------------------------
2019-10-16T10:37:47.5201189Z error[E0624]: method `parse_crate_mod` is private
2019-10-16T10:37:47.5201581Z   --> /checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs:29:20
2019-10-16T10:37:47.5201621Z    |
2019-10-16T10:37:47.5201658Z LL |     let _ = parser.parse_crate_mod();
2019-10-16T10:37:47.5201736Z 
2019-10-16T10:37:47.5201771Z error: aborting due to previous error
2019-10-16T10:37:47.5201794Z 
2019-10-16T10:37:47.5202010Z For more information about this error, try `rustc --explain E0624`.
2019-10-16T10:37:47.5202010Z For more information about this error, try `rustc --explain E0624`.
2019-10-16T10:37:47.5202037Z 
2019-10-16T10:37:47.5202210Z ------------------------------------------
2019-10-16T10:37:47.5202235Z 
2019-10-16T10:37:47.5202350Z 
2019-10-16T10:37:47.5202933Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-16T10:37:47.5203142Z 
2019-10-16T10:37:47.5203781Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-16T10:37:47.5203861Z status: exit code: 1
2019-10-16T10:37:47.5204617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-16T10:37:47.5205055Z ------------------------------------------
2019-10-16T10:37:47.5205108Z 
2019-10-16T10:37:47.5205380Z ------------------------------------------
2019-10-16T10:37:47.5205426Z stderr:
2019-10-16T10:37:47.5205426Z stderr:
2019-10-16T10:37:47.5205640Z ------------------------------------------
2019-10-16T10:37:47.5205710Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5205960Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:13
2019-10-16T10:37:47.5206009Z    |
2019-10-16T10:37:47.5206105Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-16T10:37:47.5206155Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5225695Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5226242Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:13
2019-10-16T10:37:47.5226306Z    |
2019-10-16T10:37:47.5226306Z    |
2019-10-16T10:37:47.5226362Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-16T10:37:47.5226455Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5226499Z 
2019-10-16T10:37:47.5226551Z error[E0433]: failed to resolve: use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5227124Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:32:9
2019-10-16T10:37:47.5227186Z    |
2019-10-16T10:37:47.5227225Z LL |         MacEager::expr(ecx.expr_str(sp, Symbol::intern(&args)))
2019-10-16T10:37:47.5227266Z    |         ^^^^^^^^ use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5227348Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5227550Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:66
2019-10-16T10:37:47.5227607Z    |
2019-10-16T10:37:47.5227607Z    |
2019-10-16T10:37:47.5227647Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-16T10:37:47.5227705Z    |                                                                  ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5227799Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5228000Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:40:9
2019-10-16T10:37:47.5228056Z    |
2019-10-16T10:37:47.5228056Z    |
2019-10-16T10:37:47.5228095Z LL |         SyntaxExtensionKind::LegacyBang(Box::new(Expander { args })), reg.sess.edition()
2019-10-16T10:37:47.5228140Z    |         ^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5228219Z error[E0405]: cannot find trait `TTMacroExpander` in this scope
2019-10-16T10:37:47.5228414Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:25:6
2019-10-16T10:37:47.5228451Z    |
2019-10-16T10:37:47.5228451Z    |
2019-10-16T10:37:47.5228504Z LL | impl TTMacroExpander for Expander {
2019-10-16T10:37:47.5228567Z 
2019-10-16T10:37:47.5228951Z error[E0412]: cannot find type `ExtCtxt` in this scope
2019-10-16T10:37:47.5229395Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:27:34
2019-10-16T10:37:47.5229436Z    |
2019-10-16T10:37:47.5229436Z    |
2019-10-16T10:37:47.5229622Z LL |                    ecx: &'cx mut ExtCtxt,
2019-10-16T10:37:47.5229713Z 
2019-10-16T10:37:47.5229751Z error[E0405]: cannot find trait `MacResult` in this scope
2019-10-16T10:37:47.5229977Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:29:47
2019-10-16T10:37:47.5230017Z    |
2019-10-16T10:37:47.5230017Z    |
2019-10-16T10:37:47.5230218Z LL |                    _: TokenStream) -> Box<dyn MacResult+'cx> {
2019-10-16T10:37:47.5230310Z 
2019-10-16T10:37:47.5230310Z 
2019-10-16T10:37:47.5230628Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5230969Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-16T10:37:47.5231054Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5231094Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5231149Z    |
2019-10-16T10:37:47.5231187Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5231826Z 
2019-10-16T10:37:47.5231847Z 
2019-10-16T10:37:47.5232035Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-16T10:37:47.5232080Z 
2019-10-16T10:37:47.5232317Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-16T10:37:47.5232367Z status: exit code: 1
2019-10-16T10:37:47.5233757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-16T10:37:47.5234126Z ------------------------------------------
2019-10-16T10:37:47.5234159Z 
2019-10-16T10:37:47.5234376Z ------------------------------------------
2019-10-16T10:37:47.5234440Z stderr:
2019-10-16T10:37:47.5234440Z stderr:
2019-10-16T10:37:47.5234664Z ------------------------------------------
2019-10-16T10:37:47.5234724Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5234995Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:13
2019-10-16T10:37:47.5235045Z    |
2019-10-16T10:37:47.5235092Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-16T10:37:47.5235160Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5235238Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5235485Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:13
2019-10-16T10:37:47.5235552Z    |
2019-10-16T10:37:47.5235552Z    |
2019-10-16T10:37:47.5235600Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-16T10:37:47.5235651Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5235682Z 
2019-10-16T10:37:47.5235745Z error[E0433]: failed to resolve: use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5236153Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:32:9
2019-10-16T10:37:47.5236202Z    |
2019-10-16T10:37:47.5236269Z LL |         MacEager::expr(ecx.expr_str(sp, Symbol::intern(&args)))
2019-10-16T10:37:47.5236320Z    |         ^^^^^^^^ use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5236454Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5236717Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:66
2019-10-16T10:37:47.5236763Z    |
2019-10-16T10:37:47.5236763Z    |
2019-10-16T10:37:47.5237045Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-16T10:37:47.5237100Z    |                                                                  ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5237189Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5237486Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:40:9
2019-10-16T10:37:47.5237532Z    |
2019-10-16T10:37:47.5237532Z    |
2019-10-16T10:37:47.5237594Z LL |         SyntaxExtensionKind::LegacyBang(Box::new(Expander { args })), reg.sess.edition()
2019-10-16T10:37:47.5237640Z    |         ^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5237702Z error[E0405]: cannot find trait `TTMacroExpander` in this scope
2019-10-16T10:37:47.5237944Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:25:6
2019-10-16T10:37:47.5237983Z    |
2019-10-16T10:37:47.5237983Z    |
2019-10-16T10:37:47.5238018Z LL | impl TTMacroExpander for Expander {
2019-10-16T10:37:47.5238101Z 
2019-10-16T10:37:47.5238137Z error[E0412]: cannot find type `ExtCtxt` in this scope
2019-10-16T10:37:47.5238356Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:27:34
2019-10-16T10:37:47.5238396Z    |
2019-10-16T10:37:47.5238396Z    |
2019-10-16T10:37:47.5238591Z LL |                    ecx: &'cx mut ExtCtxt,
2019-10-16T10:37:47.5238680Z 
2019-10-16T10:37:47.5238716Z error[E0405]: cannot find trait `MacResult` in this scope
2019-10-16T10:37:47.5238917Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:29:47
2019-10-16T10:37:47.5238976Z    |
2019-10-16T10:37:47.5238976Z    |
2019-10-16T10:37:47.5239171Z LL |                    _: TokenStream) -> Box<dyn MacResult+'cx> {
2019-10-16T10:37:47.5239243Z 
2019-10-16T10:37:47.5239243Z 
2019-10-16T10:37:47.5239547Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5239761Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-16T10:37:47.5239853Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5239897Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5239939Z    |
2019-10-16T10:37:47.5239992Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5240580Z 
2019-10-16T10:37:47.5240601Z 
2019-10-16T10:37:47.5240783Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-16T10:37:47.5240809Z 
2019-10-16T10:37:47.5241049Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-16T10:37:47.5241092Z status: exit code: 1
2019-10-16T10:37:47.5242058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-16T10:37:47.5242438Z ------------------------------------------
2019-10-16T10:37:47.5242467Z 
2019-10-16T10:37:47.5242815Z ------------------------------------------
2019-10-16T10:37:47.5242854Z stderr:
2019-10-16T10:37:47.5242854Z stderr:
2019-10-16T10:37:47.5243216Z ------------------------------------------
2019-10-16T10:37:47.5243453Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5243740Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:13
2019-10-16T10:37:47.5243922Z    |
2019-10-16T10:37:47.5243975Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-16T10:37:47.5244024Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5244117Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5244398Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:13
2019-10-16T10:37:47.5244446Z    |
2019-10-16T10:37:47.5244446Z    |
2019-10-16T10:37:47.5244517Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-16T10:37:47.5244567Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5244597Z 
2019-10-16T10:37:47.5244660Z error[E0433]: failed to resolve: use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5244909Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:32:9
2019-10-16T10:37:47.5244957Z    |
2019-10-16T10:37:47.5245002Z LL |         MacEager::expr(ecx.expr_str(sp, Symbol::intern(&args)))
2019-10-16T10:37:47.5245089Z    |         ^^^^^^^^ use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5245168Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5245436Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:66
2019-10-16T10:37:47.5245485Z    |
2019-10-16T10:37:47.5245485Z    |
2019-10-16T10:37:47.5245534Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-16T10:37:47.5245619Z    |                                                                  ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5245708Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5245974Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:40:9
2019-10-16T10:37:47.5246022Z    |
2019-10-16T10:37:47.5246022Z    |
2019-10-16T10:37:47.5246080Z LL |         SyntaxExtensionKind::LegacyBang(Box::new(Expander { args })), reg.sess.edition()
2019-10-16T10:37:47.5246162Z    |         ^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtensionKind`
2019-10-16T10:37:47.5246240Z error[E0405]: cannot find trait `TTMacroExpander` in this scope
2019-10-16T10:37:47.5246486Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:25:6
2019-10-16T10:37:47.5246552Z    |
2019-10-16T10:37:47.5246552Z    |
2019-10-16T10:37:47.5246595Z LL | impl TTMacroExpander for Expander {
2019-10-16T10:37:47.5246688Z 
2019-10-16T10:37:47.5246892Z error[E0412]: cannot find type `ExtCtxt` in this scope
2019-10-16T10:37:47.5247089Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:27:34
2019-10-16T10:37:47.5247144Z    |
2019-10-16T10:37:47.5247144Z    |
2019-10-16T10:37:47.5247318Z LL |                    ecx: &'cx mut ExtCtxt,
2019-10-16T10:37:47.5247386Z 
2019-10-16T10:37:47.5247446Z error[E0405]: cannot find trait `MacResult` in this scope
2019-10-16T10:37:47.5247739Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:29:47
2019-10-16T10:37:47.5247777Z    |
2019-10-16T10:37:47.5247777Z    |
2019-10-16T10:37:47.5247982Z LL |                    _: TokenStream) -> Box<dyn MacResult+'cx> {
2019-10-16T10:37:47.5248053Z 
2019-10-16T10:37:47.5248053Z 
2019-10-16T10:37:47.5248351Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5248559Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-16T10:37:47.5248648Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5248685Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5248718Z    |
2019-10-16T10:37:47.5248753Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5249431Z 
2019-10-16T10:37:47.5249469Z 
2019-10-16T10:37:47.5249654Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-16T10:37:47.5249680Z 
2019-10-16T10:37:47.5249897Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-16T10:37:47.5249957Z status: exit code: 1
2019-10-16T10:37:47.5250571Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-16T10:37:47.5250850Z ------------------------------------------
2019-10-16T10:37:47.5250875Z 
2019-10-16T10:37:47.5251067Z ------------------------------------------
2019-10-16T10:37:47.5251103Z stderr:
2019-10-16T10:37:47.5251103Z stderr:
2019-10-16T10:37:47.5251272Z ------------------------------------------
2019-10-16T10:37:47.5251330Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5251532Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:13
2019-10-16T10:37:47.5251571Z    |
2019-10-16T10:37:47.5251625Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-16T10:37:47.5251809Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5251888Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5252322Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:20:32
2019-10-16T10:37:47.5252361Z    |
2019-10-16T10:37:47.5252361Z    |
2019-10-16T10:37:47.5252400Z LL |         Symbol::intern("mac"), SyntaxExtension::dummy_bang(reg.sess.edition())
2019-10-16T10:37:47.5252466Z    |                                ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5252496Z 
2019-10-16T10:37:47.5253137Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5253618Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-16T10:37:47.5253731Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5253777Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5253820Z    |
2019-10-16T10:37:47.5257403Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5258041Z 
2019-10-16T10:37:47.5258062Z 
2019-10-16T10:37:47.5258264Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-16T10:37:47.5258289Z 
2019-10-16T10:37:47.5258508Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-16T10:37:47.5258549Z status: exit code: 1
2019-10-16T10:37:47.5259308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-16T10:37:47.5259628Z ------------------------------------------
2019-10-16T10:37:47.5259654Z 
2019-10-16T10:37:47.5259827Z ------------------------------------------
2019-10-16T10:37:47.5259880Z stderr:
2019-10-16T10:37:47.5259880Z stderr:
2019-10-16T10:37:47.5260051Z ------------------------------------------
2019-10-16T10:37:47.5260091Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5260311Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:13
2019-10-16T10:37:47.5260351Z    |
2019-10-16T10:37:47.5260385Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-16T10:37:47.5260453Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5260516Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5260744Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:20:32
2019-10-16T10:37:47.5260784Z    |
2019-10-16T10:37:47.5260784Z    |
2019-10-16T10:37:47.5260823Z LL |         Symbol::intern("mac"), SyntaxExtension::dummy_bang(reg.sess.edition())
2019-10-16T10:37:47.5260868Z    |                                ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5260913Z 
2019-10-16T10:37:47.5261204Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5261434Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-16T10:37:47.5261504Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5261566Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5261606Z    |
2019-10-16T10:37:47.5261640Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5262170Z 
2019-10-16T10:37:47.5262189Z 
2019-10-16T10:37:47.5262367Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-16T10:37:47.5262392Z 
2019-10-16T10:37:47.5262798Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-16T10:37:47.5262842Z status: exit code: 1
2019-10-16T10:37:47.5264017Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-16T10:37:47.5264513Z ------------------------------------------
2019-10-16T10:37:47.5264546Z 
2019-10-16T10:37:47.5264762Z ------------------------------------------
2019-10-16T10:37:47.5264807Z stderr:
2019-10-16T10:37:47.5264807Z stderr:
2019-10-16T10:37:47.5265039Z ------------------------------------------
2019-10-16T10:37:47.5265090Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5265343Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:13
2019-10-16T10:37:47.5265411Z    |
2019-10-16T10:37:47.5265543Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-16T10:37:47.5265607Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5265706Z error[E0433]: failed to resolve: use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5265988Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:20:32
2019-10-16T10:37:47.5266036Z    |
2019-10-16T10:37:47.5266036Z    |
2019-10-16T10:37:47.5266104Z LL |         Symbol::intern("mac"), SyntaxExtension::dummy_bang(reg.sess.edition())
2019-10-16T10:37:47.5266159Z    |                                ^^^^^^^^^^^^^^^ use of undeclared type or module `SyntaxExtension`
2019-10-16T10:37:47.5266196Z 
2019-10-16T10:37:47.5266718Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5267102Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-16T10:37:47.5267191Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5267234Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5267290Z    |
2019-10-16T10:37:47.5267326Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5267837Z 
2019-10-16T10:37:47.5267875Z 
2019-10-16T10:37:47.5268057Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-16T10:37:47.5268083Z 
2019-10-16T10:37:47.5268301Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-16T10:37:47.5268361Z status: exit code: 1
2019-10-16T10:37:47.5268968Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-16T10:37:47.5269423Z ------------------------------------------
2019-10-16T10:37:47.5269450Z 
2019-10-16T10:37:47.5269647Z ------------------------------------------
2019-10-16T10:37:47.5269683Z stderr:
2019-10-16T10:37:47.5269683Z stderr:
2019-10-16T10:37:47.5269857Z ------------------------------------------
2019-10-16T10:37:47.5269918Z error[E0433]: failed to resolve: could not find `ext` in `syntax`
2019-10-16T10:37:47.5270126Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:13
2019-10-16T10:37:47.5270167Z    |
2019-10-16T10:37:47.5276661Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-16T10:37:47.5276889Z    |             ^^^ could not find `ext` in `syntax`
2019-10-16T10:37:47.5276976Z error[E0433]: failed to resolve: use of undeclared type or module `DummyResult`
2019-10-16T10:37:47.5277321Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:36:16
2019-10-16T10:37:47.5277362Z    |
2019-10-16T10:37:47.5277362Z    |
2019-10-16T10:37:47.5277398Z LL |         return DummyResult::any(sp);
2019-10-16T10:37:47.5277481Z 
2019-10-16T10:37:47.5277518Z error[E0433]: failed to resolve: use of undeclared type or module `DummyResult`
2019-10-16T10:37:47.5277739Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:43:20
2019-10-16T10:37:47.5277778Z    |
2019-10-16T10:37:47.5277778Z    |
2019-10-16T10:37:47.5277814Z LL |             return DummyResult::any(sp);
2019-10-16T10:37:47.5278066Z 
2019-10-16T10:37:47.5278104Z error[E0433]: failed to resolve: use of undeclared type or module `DummyResult`
2019-10-16T10:37:47.5278342Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:57:24
2019-10-16T10:37:47.5278400Z    |
2019-10-16T10:37:47.5278400Z    |
2019-10-16T10:37:47.5278435Z LL |                 return DummyResult::any(sp);
2019-10-16T10:37:47.5278524Z 
2019-10-16T10:37:47.5278524Z 
2019-10-16T10:37:47.5278566Z error[E0433]: failed to resolve: use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5278767Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:62:5
2019-10-16T10:37:47.5278822Z    |
2019-10-16T10:37:47.5278857Z LL |     MacEager::expr(cx.expr_usize(sp, total))
2019-10-16T10:37:47.5278896Z    |     ^^^^^^^^ use of undeclared type or module `MacEager`
2019-10-16T10:37:47.5278980Z error[E0412]: cannot find type `ExtCtxt` in this scope
2019-10-16T10:37:47.5279185Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:23:23
2019-10-16T10:37:47.5279223Z    |
2019-10-16T10:37:47.5279223Z    |
2019-10-16T10:37:47.5279277Z LL | fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: TokenStream)
2019-10-16T10:37:47.5279342Z 
2019-10-16T10:37:47.5279377Z error[E0405]: cannot find trait `MacResult` in this scope
2019-10-16T10:37:47.5279589Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:24:20
2019-10-16T10:37:47.5279626Z    |
2019-10-16T10:37:47.5279626Z    |
2019-10-16T10:37:47.5279799Z LL |         -> Box<dyn MacResult + 'static> {
2019-10-16T10:37:47.5279882Z 
2019-10-16T10:37:47.5279882Z 
2019-10-16T10:37:47.5280183Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-16T10:37:47.5280448Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-16T10:37:47.5280533Z LL | #[plugin_registrar]
2019-10-16T10:37:47.5280591Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-16T10:37:47.5280626Z    |
2019-10-16T10:37:47.5280840Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-16T10:37:47.5286174Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-16T10:37:47.5286209Z 
2019-10-16T10:37:47.5286235Z 
2019-10-16T10:37:47.5286260Z 
2019-10-16T10:37:47.5288201Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-16T10:37:47.5288449Z 
2019-10-16T10:37:47.5288474Z 
2019-10-16T10:37:47.5288513Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-16T10:37:47.5288555Z Build completed unsuccessfully in 1:09:24
2019-10-16T10:37:47.5288555Z Build completed unsuccessfully in 1:09:24
2019-10-16T10:37:47.5288649Z == clock drift check ==
2019-10-16T10:37:47.5301783Z   local time: Wed Oct 16 10:37:47 UTC 2019
2019-10-16T10:37:47.6940958Z   network time: Wed, 16 Oct 2019 10:37:47 GMT
2019-10-16T10:37:47.6947324Z == end clock drift check ==
2019-10-16T10:37:49.4170369Z ##[error]Bash exited with code '1'.
2019-10-16T10:37:49.4227630Z ##[section]Starting: Checkout
2019-10-16T10:37:49.4229429Z ==============================================================================
2019-10-16T10:37:49.4229479Z Task         : Get sources
2019-10-16T10:37:49.4229537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
