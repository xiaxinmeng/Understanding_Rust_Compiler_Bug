plain
2019-10-15T22:26:21.6044141Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T22:26:21.6227996Z ##[command]git config gc.auto 0
2019-10-15T22:26:21.6310535Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T22:26:21.6372175Z ##[command]git config --get-all http.proxy
2019-10-15T22:26:21.6521117Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-15T22:36:34.8891360Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-15T22:36:42.8586242Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-15T22:36:49.9415647Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-15T22:37:07.5045643Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-15T22:37:53.2922241Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-15T22:38:38.2658080Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-15T22:43:39.3473469Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-15T22:44:51.8151594Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-15T22:48:06.3441545Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-15T22:48:53.1411367Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
---
2019-10-15T23:00:07.4098572Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-15T23:00:18.4541835Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-15T23:00:20.0106281Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-15T23:00:41.9374818Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-15T23:01:36.7161082Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-15T23:02:32.0746586Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-15T23:08:46.1015869Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-15T23:09:43.4450619Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-15T23:13:58.7431457Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-15T23:14:46.9715237Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
---
2019-10-15T23:28:00.1478668Z .................................................................................................... 1600/9182
2019-10-15T23:28:05.2592690Z .................................................................................................... 1700/9182
2019-10-15T23:28:17.8551315Z ............................i...............i....................................................... 1800/9182
2019-10-15T23:28:25.1960491Z .................................................................................................... 1900/9182
2019-10-15T23:28:38.9360920Z ...................iiiii............................................................................ 2000/9182
2019-10-15T23:28:49.1886179Z .................................................................................................... 2200/9182
2019-10-15T23:28:51.7584782Z .................................................................................................... 2300/9182
2019-10-15T23:28:57.0476341Z .................................................................................................... 2400/9182
2019-10-15T23:29:18.2519572Z .................................................................................................... 2500/9182
---
2019-10-15T23:32:11.0751024Z ...........................i...............i........................................................ 4800/9182
2019-10-15T23:32:22.5869370Z .................................................................................................... 4900/9182
2019-10-15T23:32:28.9630020Z .................................................................................................... 5000/9182
2019-10-15T23:32:38.1609377Z .................................................................................................... 5100/9182
2019-10-15T23:32:45.6734177Z ...........................ii.ii.................................................................... 5200/9182
2019-10-15T23:32:55.1168215Z .................................................................................................... 5400/9182
2019-10-15T23:33:05.5595046Z .............................................................................................i...... 5500/9182
2019-10-15T23:33:13.7718836Z .................................................................................................... 5600/9182
2019-10-15T23:33:18.4514044Z .................................................................................................... 5700/9182
2019-10-15T23:33:18.4514044Z .................................................................................................... 5700/9182
2019-10-15T23:33:29.6429687Z ..........................................................................................ii...i..ii 5800/9182
2019-10-15T23:33:53.6214639Z .................................................................................................... 6000/9182
2019-10-15T23:34:03.1984271Z .................................................................................................... 6100/9182
2019-10-15T23:34:11.6387038Z .................................................................................................i.. 6200/9182
2019-10-15T23:34:25.9677040Z ii.................................................................................................. 6300/9182
---
2019-10-15T23:39:20.9501210Z  finished in 5.371
2019-10-15T23:39:20.9681469Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:39:21.1473345Z 
2019-10-15T23:39:21.1473563Z running 153 tests
2019-10-15T23:39:24.2309221Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-15T23:39:26.2829515Z i..iiii..............i.........iii.i.........ii......
2019-10-15T23:39:26.2830316Z 
2019-10-15T23:39:26.2835731Z  finished in 5.315
2019-10-15T23:39:26.3004488Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:39:26.4586559Z 
---
2019-10-15T23:39:28.4475928Z  finished in 2.147
2019-10-15T23:39:28.4650709Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:39:28.6079357Z 
2019-10-15T23:39:28.6079463Z running 9 tests
2019-10-15T23:39:28.6080054Z iiiiiiiii
2019-10-15T23:39:28.6080685Z 
2019-10-15T23:39:28.6081086Z  finished in 0.143
2019-10-15T23:39:28.6249288Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:39:28.7918289Z 
---
2019-10-15T23:39:47.0859326Z  finished in 18.457
2019-10-15T23:39:47.1072392Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:39:47.2857232Z 
2019-10-15T23:39:47.2857455Z running 123 tests
2019-10-15T23:40:11.0856245Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-15T23:40:15.6021579Z i.i.i......iii.i.....ii
2019-10-15T23:40:15.6024750Z 
2019-10-15T23:40:15.6025289Z  finished in 28.494
2019-10-15T23:40:15.6060524Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:40:15.6060950Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-15T23:40:15.6060950Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-15T23:40:15.6236398Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T23:40:15.7888444Z 
2019-10-15T23:40:15.7888803Z running 69 tests
2019-10-15T23:40:39.4477784Z F............F....................................F...FFFFFFF..F.....
2019-10-15T23:40:39.4481689Z 
2019-10-15T23:40:39.4483158Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-15T23:40:39.4483592Z 
2019-10-15T23:40:39.4483940Z error: test compilation failed although it shouldn't!
2019-10-15T23:40:39.4483940Z error: test compilation failed although it shouldn't!
2019-10-15T23:40:39.4483998Z status: exit code: 1
2019-10-15T23:40:39.4484837Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-15T23:40:39.4485382Z ------------------------------------------
2019-10-15T23:40:39.4485432Z 
2019-10-15T23:40:39.4485614Z ------------------------------------------
2019-10-15T23:40:39.4485650Z stderr:
---
2019-10-15T23:40:39.4488438Z 
2019-10-15T23:40:39.4488472Z error[E0412]: cannot find type `Parser` in this scope
2019-10-15T23:40:39.4488704Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:27:71
2019-10-15T23:40:39.4488743Z    |
2019-10-15T23:40:39.4488959Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-15T23:40:39.4489054Z 
2019-10-15T23:40:39.4489088Z error[E0412]: cannot find type `Parser` in this scope
2019-10-15T23:40:39.4489287Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:32:20
2019-10-15T23:40:39.4489342Z    |
2019-10-15T23:40:39.4489342Z    |
2019-10-15T23:40:39.4489525Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-15T23:40:39.4489785Z 
2019-10-15T23:40:39.4489819Z error[E0282]: type annotations needed
2019-10-15T23:40:39.4490022Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:48:51
2019-10-15T23:40:39.4490148Z    |
2019-10-15T23:40:39.4490148Z    |
2019-10-15T23:40:39.4490211Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-15T23:40:39.4490257Z    |                                                   ^ consider giving this closure parameter a type
2019-10-15T23:40:39.4490346Z    = note: type must be known at this point
2019-10-15T23:40:39.4490370Z 
2019-10-15T23:40:39.4490404Z error[E0282]: type annotations needed
2019-10-15T23:40:39.4491556Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:54:51
2019-10-15T23:40:39.4491556Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:54:51
2019-10-15T23:40:39.4491632Z    |
2019-10-15T23:40:39.4491679Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-15T23:40:39.4491734Z    |                                                   ^ consider giving this closure parameter a type
2019-10-15T23:40:39.4491841Z    = note: type must be known at this point
2019-10-15T23:40:39.4491872Z 
2019-10-15T23:40:39.4491932Z error[E0282]: type annotations needed
2019-10-15T23:40:39.4492325Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:60:51
2019-10-15T23:40:39.4492325Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:60:51
2019-10-15T23:40:39.4492385Z    |
2019-10-15T23:40:39.4492431Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-15T23:40:39.4492506Z    |                                                   ^ consider giving this closure parameter a type
2019-10-15T23:40:39.4492597Z    = note: type must be known at this point
2019-10-15T23:40:39.4492646Z 
2019-10-15T23:40:39.4492689Z error: aborting due to 11 previous errors
2019-10-15T23:40:39.4492718Z 
---
2019-10-15T23:40:39.4493369Z 
2019-10-15T23:40:39.4493414Z 
2019-10-15T23:40:39.4493650Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-15T23:40:39.4493683Z 
2019-10-15T23:40:39.4494002Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T23:40:39.4494078Z status: exit code: 1
2019-10-15T23:40:39.4495100Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-15T23:40:39.4495535Z ------------------------------------------
2019-10-15T23:40:39.4495563Z 
2019-10-15T23:40:39.4495766Z ------------------------------------------
2019-10-15T23:40:39.4495810Z stderr:
2019-10-15T23:40:39.4495810Z stderr:
2019-10-15T23:40:39.4496155Z ------------------------------------------
2019-10-15T23:40:39.4496246Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T23:40:39.4496456Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T23:40:39.4496494Z    |
2019-10-15T23:40:39.4496546Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T23:40:39.4496587Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4496613Z 
2019-10-15T23:40:39.4497019Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4497243Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T23:40:39.4497334Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4497370Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4497541Z    |
2019-10-15T23:40:39.4497694Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4498870Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-15T23:40:39.4499075Z 
2019-10-15T23:40:39.4499270Z error: test compilation failed although it shouldn't!
2019-10-15T23:40:39.4499329Z status: exit code: 1
2019-10-15T23:40:39.4500240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-15T23:40:39.4500580Z ------------------------------------------
2019-10-15T23:40:39.4500609Z 
2019-10-15T23:40:39.4501180Z ------------------------------------------
2019-10-15T23:40:39.4501239Z stderr:
---
2019-10-15T23:40:39.4502691Z 
2019-10-15T23:40:39.4502736Z 
2019-10-15T23:40:39.4502973Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-15T23:40:39.4503005Z 
2019-10-15T23:40:39.4503289Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T23:40:39.4503363Z status: exit code: 1
2019-10-15T23:40:39.4504133Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-15T23:40:39.4504647Z ------------------------------------------
2019-10-15T23:40:39.4504679Z 
2019-10-15T23:40:39.4505317Z ------------------------------------------
2019-10-15T23:40:39.4505356Z stderr:
2019-10-15T23:40:39.4505356Z stderr:
2019-10-15T23:40:39.4505723Z ------------------------------------------
2019-10-15T23:40:39.4505787Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T23:40:39.4506189Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T23:40:39.4506232Z    |
2019-10-15T23:40:39.4506292Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T23:40:39.4506338Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T23:40:39.4506497Z    |                         |
2019-10-15T23:40:39.4506563Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4506590Z 
2019-10-15T23:40:39.4506637Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T23:40:39.4507149Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T23:40:39.4507193Z    |
2019-10-15T23:40:39.4507234Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T23:40:39.4507304Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T23:40:39.4507396Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T23:40:39.4507460Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4507460Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4507591Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T23:40:39.4507628Z 
2019-10-15T23:40:39.4507998Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4508241Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T23:40:39.4508339Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4508379Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4508417Z    |
2019-10-15T23:40:39.4508473Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4509058Z 
2019-10-15T23:40:39.4509256Z 
2019-10-15T23:40:39.4509484Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-15T23:40:39.4509512Z 
2019-10-15T23:40:39.4509753Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T23:40:39.4509814Z status: exit code: 1
2019-10-15T23:40:39.4511189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-15T23:40:39.4511547Z ------------------------------------------
2019-10-15T23:40:39.4511591Z 
2019-10-15T23:40:39.4511849Z ------------------------------------------
2019-10-15T23:40:39.4511897Z stderr:
2019-10-15T23:40:39.4511897Z stderr:
2019-10-15T23:40:39.4512126Z ------------------------------------------
2019-10-15T23:40:39.4512183Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T23:40:39.4512471Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T23:40:39.4512522Z    |
2019-10-15T23:40:39.4512568Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T23:40:39.4512642Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T23:40:39.4512693Z    |                         |
2019-10-15T23:40:39.4512741Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4512791Z 
2019-10-15T23:40:39.4512856Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T23:40:39.4513282Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T23:40:39.4513353Z    |
2019-10-15T23:40:39.4513400Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T23:40:39.4513457Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T23:40:39.4513580Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T23:40:39.4513633Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4513633Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4513707Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T23:40:39.4513737Z 
2019-10-15T23:40:39.4514100Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4514606Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T23:40:39.4514687Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4514746Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4514781Z    |
2019-10-15T23:40:39.4514816Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4515392Z 
2019-10-15T23:40:39.4515413Z 
2019-10-15T23:40:39.4515600Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-15T23:40:39.4515646Z 
2019-10-15T23:40:39.4515872Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T23:40:39.4515921Z status: exit code: 1
2019-10-15T23:40:39.4516542Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-15T23:40:39.4516821Z ------------------------------------------
2019-10-15T23:40:39.4516847Z 
2019-10-15T23:40:39.4517028Z ------------------------------------------
2019-10-15T23:40:39.4517082Z stderr:
2019-10-15T23:40:39.4517082Z stderr:
2019-10-15T23:40:39.4517261Z ------------------------------------------
2019-10-15T23:40:39.4517313Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T23:40:39.4517546Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T23:40:39.4517586Z    |
2019-10-15T23:40:39.4517623Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T23:40:39.4517684Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T23:40:39.4517724Z    |                         |
2019-10-15T23:40:39.4517762Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4517786Z 
2019-10-15T23:40:39.4517897Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T23:40:39.4518123Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T23:40:39.4518180Z    |
2019-10-15T23:40:39.4518218Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T23:40:39.4518361Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T23:40:39.4518464Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T23:40:39.4518506Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4518506Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4518563Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T23:40:39.4518587Z 
2019-10-15T23:40:39.4522697Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4523774Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T23:40:39.4523909Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4523980Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4524043Z    |
2019-10-15T23:40:39.4524552Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4525743Z 
2019-10-15T23:40:39.4525762Z 
2019-10-15T23:40:39.4525953Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-15T23:40:39.4525979Z 
2019-10-15T23:40:39.4526221Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T23:40:39.4526262Z status: exit code: 1
2019-10-15T23:40:39.4527260Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-15T23:40:39.4527562Z ------------------------------------------
2019-10-15T23:40:39.4527590Z 
2019-10-15T23:40:39.4527784Z ------------------------------------------
2019-10-15T23:40:39.4527821Z stderr:
2019-10-15T23:40:39.4527821Z stderr:
2019-10-15T23:40:39.4528025Z ------------------------------------------
2019-10-15T23:40:39.4528067Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T23:40:39.4528464Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T23:40:39.4528525Z    |
2019-10-15T23:40:39.4528563Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T23:40:39.4528619Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4528666Z 
2019-10-15T23:40:39.4539366Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4539821Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T23:40:39.4540100Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4540137Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4540192Z    |
2019-10-15T23:40:39.4540390Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4541995Z 
2019-10-15T23:40:39.4542022Z 
2019-10-15T23:40:39.4542316Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-15T23:40:39.4542351Z 
2019-10-15T23:40:39.4542661Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T23:40:39.4542717Z status: exit code: 1
2019-10-15T23:40:39.4543507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-15T23:40:39.4543968Z ------------------------------------------
2019-10-15T23:40:39.4544033Z 
2019-10-15T23:40:39.4544434Z ------------------------------------------
2019-10-15T23:40:39.4544472Z stderr:
2019-10-15T23:40:39.4544472Z stderr:
2019-10-15T23:40:39.4544670Z ------------------------------------------
2019-10-15T23:40:39.4544713Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T23:40:39.4544923Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T23:40:39.4544981Z    |
2019-10-15T23:40:39.4545015Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T23:40:39.4545056Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4545082Z 
2019-10-15T23:40:39.4545408Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4545633Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T23:40:39.4545733Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4545776Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4545828Z    |
2019-10-15T23:40:39.4545863Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4546397Z 
2019-10-15T23:40:39.4546435Z 
2019-10-15T23:40:39.4546623Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-15T23:40:39.4546649Z 
2019-10-15T23:40:39.4546877Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T23:40:39.4546938Z status: exit code: 1
2019-10-15T23:40:39.4547555Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-15T23:40:39.4547839Z ------------------------------------------
2019-10-15T23:40:39.4547866Z 
2019-10-15T23:40:39.4548065Z ------------------------------------------
2019-10-15T23:40:39.4548101Z stderr:
2019-10-15T23:40:39.4548101Z stderr:
2019-10-15T23:40:39.4548281Z ------------------------------------------
2019-10-15T23:40:39.4548341Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T23:40:39.4548552Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T23:40:39.4548688Z    |
2019-10-15T23:40:39.4548749Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T23:40:39.4548789Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T23:40:39.4548816Z 
2019-10-15T23:40:39.4549142Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4549364Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T23:40:39.4549454Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4549490Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4549524Z    |
2019-10-15T23:40:39.4549576Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4550463Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-15T23:40:39.4550838Z 
2019-10-15T23:40:39.4551123Z error: test compilation failed although it shouldn't!
2019-10-15T23:40:39.4551192Z status: exit code: 1
2019-10-15T23:40:39.4551939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-15T23:40:39.4552302Z ------------------------------------------
2019-10-15T23:40:39.4552336Z 
2019-10-15T23:40:39.4552585Z ------------------------------------------
2019-10-15T23:40:39.4552630Z stderr:
---
2019-10-15T23:40:39.4554374Z 
2019-10-15T23:40:39.4554412Z 
2019-10-15T23:40:39.4554620Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-15T23:40:39.4554646Z 
2019-10-15T23:40:39.4554874Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-15T23:40:39.4554936Z status: exit code: 1
2019-10-15T23:40:39.4555550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-15T23:40:39.4562114Z ------------------------------------------
2019-10-15T23:40:39.4562377Z 
2019-10-15T23:40:39.4562795Z ------------------------------------------
2019-10-15T23:40:39.4562848Z stderr:
2019-10-15T23:40:39.4562848Z stderr:
2019-10-15T23:40:39.4563079Z ------------------------------------------
2019-10-15T23:40:39.4563155Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-15T23:40:39.4563430Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-15T23:40:39.4563482Z    |
2019-10-15T23:40:39.4563547Z LL | use syntax::parse::token::{self, Token};
2019-10-15T23:40:39.4563596Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-15T23:40:39.4563685Z error[E0432]: unresolved import `syntax::parse`
2019-10-15T23:40:39.4563952Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-15T23:40:39.4564189Z    |
2019-10-15T23:40:39.4564189Z    |
2019-10-15T23:40:39.4564227Z LL | use syntax::parse::token::{self, Token};
2019-10-15T23:40:39.4564267Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-15T23:40:39.4564299Z 
2019-10-15T23:40:39.4564511Z error[E0432]: unresolved imports `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::DummyResult`, `syntax::ext::base::MacEager`
2019-10-15T23:40:39.4564779Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:25
2019-10-15T23:40:39.4564821Z    |
2019-10-15T23:40:39.4564879Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-15T23:40:39.4564924Z    |                         ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T23:40:39.4565499Z    |                         |        |          no `DummyResult` in `ext::base`
2019-10-15T23:40:39.4565545Z    |                         |        no `MacResult` in `ext::base`
2019-10-15T23:40:39.4565589Z    |                         no `ExtCtxt` in `ext::base`
2019-10-15T23:40:39.4565631Z 
2019-10-15T23:40:39.4565631Z 
2019-10-15T23:40:39.4565681Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-15T23:40:39.4566224Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:40:26
2019-10-15T23:40:39.4566295Z    |
2019-10-15T23:40:39.4566336Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-15T23:40:39.4566444Z help: possible candidates are found in other modules, you can import them into scope
2019-10-15T23:40:39.4566484Z    |
2019-10-15T23:40:39.4566519Z LL | use syntax::token::Token;
2019-10-15T23:40:39.4566554Z    |
2019-10-15T23:40:39.4566554Z    |
2019-10-15T23:40:39.4566609Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-15T23:40:39.4566646Z    |
2019-10-15T23:40:39.4566668Z 
2019-10-15T23:40:39.4567028Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T23:40:39.4567271Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-15T23:40:39.4567382Z LL | #[plugin_registrar]
2019-10-15T23:40:39.4567423Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T23:40:39.4567460Z    |
2019-10-15T23:40:39.4567515Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T23:40:39.4572201Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-15T23:40:39.4572238Z 
2019-10-15T23:40:39.4572281Z 
2019-10-15T23:40:39.4572307Z 
2019-10-15T23:40:39.4573930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-15T23:40:39.4574509Z 
2019-10-15T23:40:39.4574533Z 
2019-10-15T23:40:39.4574793Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-15T23:40:39.4574860Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-15T23:40:39.4574860Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-15T23:40:39.4574906Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-15T23:40:39.4574949Z Build completed unsuccessfully in 1:07:10
2019-10-15T23:40:39.4575053Z == clock drift check ==
2019-10-15T23:40:39.4575094Z   local time: Tue Oct 15 23:40:39 UTC 2019
2019-10-15T23:40:39.9930243Z   network time: Tue, 15 Oct 2019 23:40:39 GMT
2019-10-15T23:40:39.9934910Z == end clock drift check ==
2019-10-15T23:40:41.9945192Z ##[error]Bash exited with code '1'.
2019-10-15T23:40:41.9995722Z ##[section]Starting: Checkout
2019-10-15T23:40:41.9997419Z ==============================================================================
2019-10-15T23:40:41.9997464Z Task         : Get sources
2019-10-15T23:40:41.9997505Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
