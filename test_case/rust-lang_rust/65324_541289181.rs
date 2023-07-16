plain
2019-10-12T04:44:06.0303899Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T04:44:06.0837767Z ##[command]git config gc.auto 0
2019-10-12T04:44:06.0923074Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T04:44:06.0999483Z ##[command]git config --get-all http.proxy
2019-10-12T04:44:06.1165522Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-12T04:54:15.3092832Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-12T04:54:24.1981536Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-12T04:54:25.6640890Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-12T04:54:43.2447730Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-12T04:55:30.3477121Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-12T04:56:16.6751237Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-12T05:01:25.0794726Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-12T05:02:13.4377011Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-12T05:03:27.3904405Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-12T05:06:07.3571582Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
---
2019-10-12T05:18:23.4662952Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-12T05:18:34.5688535Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-12T05:18:42.8419767Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-12T05:19:04.9346951Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-12T05:20:00.9353266Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-12T05:20:56.9292581Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-12T05:27:17.6192804Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-12T05:32:19.2097910Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-12T05:33:47.5775653Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-12T05:34:42.9859324Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
---
2019-10-12T05:46:51.6721256Z .................................................................................................... 1600/9146
2019-10-12T05:46:59.3010257Z .................................................................................................... 1700/9146
2019-10-12T05:47:10.6851014Z .................i...............i.................................................................. 1800/9146
2019-10-12T05:47:17.8947153Z .................................................................................................... 1900/9146
2019-10-12T05:47:33.4645404Z ........iiiii....................................................................................... 2000/9146
2019-10-12T05:47:43.4082509Z .................................................................................................... 2200/9146
2019-10-12T05:47:46.0841691Z .................................................................................................... 2300/9146
2019-10-12T05:47:51.8638442Z .................................................................................................... 2400/9146
2019-10-12T05:47:58.1234716Z .................................................................................................... 2500/9146
---
2019-10-12T05:50:53.7566262Z .................................................................................................... 4700/9146
2019-10-12T05:51:01.1885436Z .i...............i.................................................................................. 4800/9146
2019-10-12T05:51:12.4631232Z .................................................................................................... 4900/9146
2019-10-12T05:51:18.1209451Z .................................................................................................... 5000/9146
2019-10-12T05:51:29.4705898Z ...............................................................................................ii.ii 5100/9146
2019-10-12T05:51:40.1096585Z .................................................................................................... 5300/9146
2019-10-12T05:51:50.1899438Z .................................................................................................... 5400/9146
2019-10-12T05:51:57.1266210Z .............................................................i...................................... 5500/9146
2019-10-12T05:52:04.5613024Z .................................................................................................... 5600/9146
2019-10-12T05:52:04.5613024Z .................................................................................................... 5600/9146
2019-10-12T05:52:12.0760212Z .................................................................................................... 5700/9146
2019-10-12T05:52:22.3829173Z ..........................................................ii...i..ii...........i.................... 5800/9146
2019-10-12T05:52:48.5723776Z .................................................................................................... 6000/9146
2019-10-12T05:52:57.9103167Z .................................................................................................... 6100/9146
2019-10-12T05:52:57.9103167Z .................................................................................................... 6100/9146
2019-10-12T05:53:05.4315516Z ................................................................i..ii............................... 6200/9146
2019-10-12T05:53:34.8290724Z .................................................................................................... 6400/9146
2019-10-12T05:53:36.9806623Z ........................i........................................................................... 6500/9146
2019-10-12T05:53:39.2261736Z .................................................................................................i.. 6600/9146
2019-10-12T05:53:42.0052052Z .................................................................................................... 6700/9146
---
2019-10-12T05:58:24.5814268Z  finished in 5.667
2019-10-12T05:58:24.6003418Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:58:24.7672229Z 
2019-10-12T05:58:24.7672375Z running 153 tests
2019-10-12T05:58:28.0776614Z i....iii......iii..iiii....i..............................i.i..................i....i............ii. 100/153
2019-10-12T05:58:30.0916049Z i.i..iiii..............i.........iii.i.......ii......
2019-10-12T05:58:30.0916779Z 
2019-10-12T05:58:30.0921705Z  finished in 5.492
2019-10-12T05:58:30.1108358Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:58:30.2836029Z 
---
2019-10-12T05:58:32.3745850Z  finished in 2.263
2019-10-12T05:58:32.3924548Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:58:32.5533852Z 
2019-10-12T05:58:32.5534694Z running 9 tests
2019-10-12T05:58:32.5535650Z iiiiiiiii
2019-10-12T05:58:32.5536197Z 
2019-10-12T05:58:32.5536250Z  finished in 0.160
2019-10-12T05:58:32.5738961Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:58:32.7568723Z 
---
2019-10-12T05:58:51.1540312Z  finished in 18.578
2019-10-12T05:58:51.1760236Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:58:51.3484851Z 
2019-10-12T05:58:51.3485199Z running 123 tests
2019-10-12T05:59:16.6716391Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-12T05:59:21.5657807Z i.i.i......iii.i.....ii
2019-10-12T05:59:21.5661064Z 
2019-10-12T05:59:21.5662333Z  finished in 30.390
2019-10-12T05:59:21.5671270Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:59:21.5673920Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-12T05:59:21.5673920Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-12T05:59:21.5884861Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T05:59:21.7508370Z 
2019-10-12T05:59:21.7511081Z running 69 tests
2019-10-12T05:59:47.1059873Z F............F....................................F...FFFFFFF..F.....
2019-10-12T05:59:47.1092998Z 
2019-10-12T05:59:47.1093727Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-12T05:59:47.1104355Z 
2019-10-12T05:59:47.1105218Z error: test compilation failed although it shouldn't!
2019-10-12T05:59:47.1105218Z error: test compilation failed although it shouldn't!
2019-10-12T05:59:47.1105557Z status: exit code: 1
2019-10-12T05:59:47.1106543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-12T05:59:47.1107619Z ------------------------------------------
2019-10-12T05:59:47.1107755Z 
2019-10-12T05:59:47.1108025Z ------------------------------------------
2019-10-12T05:59:47.1109267Z stderr:
---
2019-10-12T05:59:47.1110505Z 
2019-10-12T05:59:47.1110568Z error[E0432]: unresolved import `syntax::parse`
2019-10-12T05:59:47.1111406Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:15:13
2019-10-12T05:59:47.1111471Z    |
2019-10-12T05:59:47.1111542Z LL | use syntax::parse::{ParseSess, PResult};
2019-10-12T05:59:47.1111794Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-12T05:59:47.1111892Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-12T05:59:47.1112463Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:17:13
2019-10-12T05:59:47.1112513Z    |
2019-10-12T05:59:47.1112585Z LL | use syntax::parse::parser::Parser;
---
2019-10-12T05:59:47.1114275Z 
2019-10-12T05:59:47.1114334Z error[E0412]: cannot find type `Parser` in this scope
2019-10-12T05:59:47.1114582Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:26:71
2019-10-12T05:59:47.1114629Z    |
2019-10-12T05:59:47.1114904Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-12T05:59:47.1115019Z 
2019-10-12T05:59:47.1115063Z error[E0412]: cannot find type `Parser` in this scope
2019-10-12T05:59:47.1115329Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:31:20
2019-10-12T05:59:47.1115385Z    |
2019-10-12T05:59:47.1115385Z    |
2019-10-12T05:59:47.1115617Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-12T05:59:47.1115716Z 
2019-10-12T05:59:47.1115757Z error: aborting due to 8 previous errors
2019-10-12T05:59:47.1115785Z 
2019-10-12T05:59:47.1115846Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-12T05:59:47.1115846Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-12T05:59:47.1116092Z For more information about an error, try `rustc --explain E0412`.
2019-10-12T05:59:47.1116126Z 
2019-10-12T05:59:47.1116355Z ------------------------------------------
2019-10-12T05:59:47.1116387Z 
2019-10-12T05:59:47.1116411Z 
2019-10-12T05:59:47.1116634Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-12T05:59:47.1116667Z 
2019-10-12T05:59:47.1116963Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-12T05:59:47.1117025Z status: exit code: 1
2019-10-12T05:59:47.1117784Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-12T05:59:47.1118115Z ------------------------------------------
2019-10-12T05:59:47.1118169Z 
2019-10-12T05:59:47.1118398Z ------------------------------------------
2019-10-12T05:59:47.1118444Z stderr:
2019-10-12T05:59:47.1118444Z stderr:
2019-10-12T05:59:47.1118809Z ------------------------------------------
2019-10-12T05:59:47.1118863Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-12T05:59:47.1119132Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-12T05:59:47.1119203Z    |
2019-10-12T05:59:47.1119246Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-12T05:59:47.1119296Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1119327Z 
2019-10-12T05:59:47.1119801Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1120081Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-12T05:59:47.1120188Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1120235Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1120293Z    |
2019-10-12T05:59:47.1120336Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1121724Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-12T05:59:47.1121757Z 
2019-10-12T05:59:47.1121998Z error: test compilation failed although it shouldn't!
2019-10-12T05:59:47.1122047Z status: exit code: 1
2019-10-12T05:59:47.1122799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-12T05:59:47.1123157Z ------------------------------------------
2019-10-12T05:59:47.1123190Z 
2019-10-12T05:59:47.1123435Z ------------------------------------------
2019-10-12T05:59:47.1123481Z stderr:
2019-10-12T05:59:47.1123481Z stderr:
2019-10-12T05:59:47.1123693Z ------------------------------------------
2019-10-12T05:59:47.1123764Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-12T05:59:47.1124071Z    |
2019-10-12T05:59:47.1124071Z    |
2019-10-12T05:59:47.1124131Z LL | use syntax::parse::{self, ParseSess};
2019-10-12T05:59:47.1124177Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-12T05:59:47.1124283Z    |             could not find `parse` in `syntax`
2019-10-12T05:59:47.1124323Z 
2019-10-12T05:59:47.1124364Z error: aborting due to previous error
2019-10-12T05:59:47.1124392Z 
2019-10-12T05:59:47.1124392Z 
2019-10-12T05:59:47.1124848Z For more information about this error, try `rustc --explain E0432`.
2019-10-12T05:59:47.1124889Z 
2019-10-12T05:59:47.1125145Z ------------------------------------------
2019-10-12T05:59:47.1125178Z 
2019-10-12T05:59:47.1125203Z 
2019-10-12T05:59:47.1125445Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-12T05:59:47.1125478Z 
2019-10-12T05:59:47.1126140Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-12T05:59:47.1126227Z status: exit code: 1
2019-10-12T05:59:47.1126996Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-12T05:59:47.1127549Z ------------------------------------------
2019-10-12T05:59:47.1127584Z 
2019-10-12T05:59:47.1127822Z ------------------------------------------
2019-10-12T05:59:47.1127868Z stderr:
2019-10-12T05:59:47.1127868Z stderr:
2019-10-12T05:59:47.1128079Z ------------------------------------------
2019-10-12T05:59:47.1128153Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-12T05:59:47.1128411Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-12T05:59:47.1128461Z    |
2019-10-12T05:59:47.1128524Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-12T05:59:47.1128591Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-12T05:59:47.1128639Z    |                         |
2019-10-12T05:59:47.1128796Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1128835Z 
2019-10-12T05:59:47.1128893Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-12T05:59:47.1129204Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-12T05:59:47.1129256Z    |
2019-10-12T05:59:47.1129303Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-12T05:59:47.1129376Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-12T05:59:47.1129479Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-12T05:59:47.1129557Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1129557Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1129614Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-12T05:59:47.1129644Z 
2019-10-12T05:59:47.1130033Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1130306Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-12T05:59:47.1130413Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1130457Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1130498Z    |
2019-10-12T05:59:47.1130542Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1132008Z 
2019-10-12T05:59:47.1132033Z 
2019-10-12T05:59:47.1132283Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-12T05:59:47.1132317Z 
2019-10-12T05:59:47.1132593Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-12T05:59:47.1132648Z status: exit code: 1
2019-10-12T05:59:47.1133408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-12T05:59:47.1133934Z ------------------------------------------
2019-10-12T05:59:47.1133968Z 
2019-10-12T05:59:47.1134197Z ------------------------------------------
2019-10-12T05:59:47.1134260Z stderr:
2019-10-12T05:59:47.1134260Z stderr:
2019-10-12T05:59:47.1134474Z ------------------------------------------
2019-10-12T05:59:47.1134532Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-12T05:59:47.1135085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-12T05:59:47.1135145Z    |
2019-10-12T05:59:47.1135191Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-12T05:59:47.1135267Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-12T05:59:47.1135316Z    |                         |
2019-10-12T05:59:47.1135364Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1135421Z 
2019-10-12T05:59:47.1135592Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-12T05:59:47.1135905Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-12T05:59:47.1135977Z    |
2019-10-12T05:59:47.1136024Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-12T05:59:47.1136080Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-12T05:59:47.1136197Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-12T05:59:47.1136249Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1136249Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1136315Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-12T05:59:47.1136345Z 
2019-10-12T05:59:47.1136732Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1137028Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-12T05:59:47.1137118Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1137179Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1137221Z    |
2019-10-12T05:59:47.1137265Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1137920Z 
2019-10-12T05:59:47.1137945Z 
2019-10-12T05:59:47.1138169Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-12T05:59:47.1138218Z 
2019-10-12T05:59:47.1138503Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-12T05:59:47.1138556Z status: exit code: 1
2019-10-12T05:59:47.1139318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-12T05:59:47.1139644Z ------------------------------------------
2019-10-12T05:59:47.1139680Z 
2019-10-12T05:59:47.1139897Z ------------------------------------------
2019-10-12T05:59:47.1140057Z stderr:
2019-10-12T05:59:47.1140057Z stderr:
2019-10-12T05:59:47.1140321Z ------------------------------------------
2019-10-12T05:59:47.1140389Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-12T05:59:47.1140648Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-12T05:59:47.1140722Z    |
2019-10-12T05:59:47.1140768Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-12T05:59:47.1140823Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-12T05:59:47.1141115Z    |                         |
2019-10-12T05:59:47.1141165Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1141195Z 
2019-10-12T05:59:47.1141270Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-12T05:59:47.1141598Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-12T05:59:47.1141663Z    |
2019-10-12T05:59:47.1141728Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-12T05:59:47.1141909Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-12T05:59:47.1142040Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-12T05:59:47.1142092Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1142092Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1142142Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-12T05:59:47.1142190Z 
2019-10-12T05:59:47.1142589Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1142858Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-12T05:59:47.1142977Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1143022Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1143081Z    |
2019-10-12T05:59:47.1143132Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1143791Z 
2019-10-12T05:59:47.1143816Z 
2019-10-12T05:59:47.1144048Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-12T05:59:47.1144082Z 
2019-10-12T05:59:47.1144371Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-12T05:59:47.1144425Z status: exit code: 1
2019-10-12T05:59:47.1145193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-12T05:59:47.1145532Z ------------------------------------------
2019-10-12T05:59:47.1145583Z 
2019-10-12T05:59:47.1145803Z ------------------------------------------
2019-10-12T05:59:47.1145850Z stderr:
2019-10-12T05:59:47.1145850Z stderr:
2019-10-12T05:59:47.1146060Z ------------------------------------------
2019-10-12T05:59:47.1146129Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-12T05:59:47.1146386Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-12T05:59:47.1146565Z    |
2019-10-12T05:59:47.1146609Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-12T05:59:47.1146669Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1146700Z 
2019-10-12T05:59:47.1147098Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1147372Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-12T05:59:47.1147480Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1147524Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1147582Z    |
2019-10-12T05:59:47.1147626Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1148283Z 
2019-10-12T05:59:47.1148325Z 
2019-10-12T05:59:47.1148647Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-12T05:59:47.1148689Z 
2019-10-12T05:59:47.1148995Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-12T05:59:47.1149068Z status: exit code: 1
2019-10-12T05:59:47.1149834Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-12T05:59:47.1150194Z ------------------------------------------
2019-10-12T05:59:47.1150229Z 
2019-10-12T05:59:47.1150472Z ------------------------------------------
2019-10-12T05:59:47.1150519Z stderr:
2019-10-12T05:59:47.1150519Z stderr:
2019-10-12T05:59:47.1150731Z ------------------------------------------
2019-10-12T05:59:47.1150799Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-12T05:59:47.1151349Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-12T05:59:47.1151408Z    |
2019-10-12T05:59:47.1151473Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-12T05:59:47.1151523Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1151554Z 
2019-10-12T05:59:47.1151926Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1152218Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-12T05:59:47.1152327Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1152382Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1152423Z    |
2019-10-12T05:59:47.1152485Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1153134Z 
2019-10-12T05:59:47.1153175Z 
2019-10-12T05:59:47.1153398Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-12T05:59:47.1153429Z 
2019-10-12T05:59:47.1153703Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-12T05:59:47.1153922Z status: exit code: 1
2019-10-12T05:59:47.1154714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-12T05:59:47.1155050Z ------------------------------------------
2019-10-12T05:59:47.1155087Z 
2019-10-12T05:59:47.1155321Z ------------------------------------------
2019-10-12T05:59:47.1155368Z stderr:
2019-10-12T05:59:47.1155368Z stderr:
2019-10-12T05:59:47.1155580Z ------------------------------------------
2019-10-12T05:59:47.1155648Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-12T05:59:47.1155914Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-12T05:59:47.1155965Z    |
2019-10-12T05:59:47.1156108Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-12T05:59:47.1156167Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-12T05:59:47.1156198Z 
2019-10-12T05:59:47.1156600Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1156876Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-12T05:59:47.1156983Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1157027Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1157070Z    |
2019-10-12T05:59:47.1157130Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1158067Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-12T05:59:47.1158101Z 
2019-10-12T05:59:47.1158325Z error: test compilation failed although it shouldn't!
2019-10-12T05:59:47.1158373Z status: exit code: 1
2019-10-12T05:59:47.1159108Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-12T05:59:47.1159449Z ------------------------------------------
2019-10-12T05:59:47.1159484Z 
2019-10-12T05:59:47.1159701Z ------------------------------------------
2019-10-12T05:59:47.1159763Z stderr:
2019-10-12T05:59:47.1159763Z stderr:
2019-10-12T05:59:47.1159977Z ------------------------------------------
2019-10-12T05:59:47.1160029Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-12T05:59:47.1160344Z    |
2019-10-12T05:59:47.1160344Z    |
2019-10-12T05:59:47.1160387Z LL | use syntax::parse::{self, ParseSess};
2019-10-12T05:59:47.1160451Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-12T05:59:47.1160540Z    |             could not find `parse` in `syntax`
2019-10-12T05:59:47.1160568Z 
2019-10-12T05:59:47.1160626Z error: aborting due to previous error
2019-10-12T05:59:47.1160761Z 
2019-10-12T05:59:47.1160761Z 
2019-10-12T05:59:47.1161302Z For more information about this error, try `rustc --explain E0432`.
2019-10-12T05:59:47.1161344Z 
2019-10-12T05:59:47.1161599Z ------------------------------------------
2019-10-12T05:59:47.1161631Z 
2019-10-12T05:59:47.1161655Z 
2019-10-12T05:59:47.1161889Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-12T05:59:47.1161939Z 
2019-10-12T05:59:47.1162215Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-12T05:59:47.1162271Z status: exit code: 1
2019-10-12T05:59:47.1163172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-12T05:59:47.1163559Z ------------------------------------------
2019-10-12T05:59:47.1163594Z 
2019-10-12T05:59:47.1163810Z ------------------------------------------
2019-10-12T05:59:47.1163875Z stderr:
2019-10-12T05:59:47.1163875Z stderr:
2019-10-12T05:59:47.1164090Z ------------------------------------------
2019-10-12T05:59:47.1164143Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-12T05:59:47.1164416Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-12T05:59:47.1164469Z    |
2019-10-12T05:59:47.1164512Z LL | use syntax::parse::token::{self, Token};
2019-10-12T05:59:47.1164559Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-12T05:59:47.1164649Z error[E0432]: unresolved import `syntax::parse`
2019-10-12T05:59:47.1164915Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-12T05:59:47.1164980Z    |
2019-10-12T05:59:47.1164980Z    |
2019-10-12T05:59:47.1165031Z LL | use syntax::parse::token::{self, Token};
2019-10-12T05:59:47.1165078Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-12T05:59:47.1165107Z 
2019-10-12T05:59:47.1165179Z error[E0432]: unresolved imports `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::DummyResult`, `syntax::ext::base::MacEager`
2019-10-12T05:59:47.1165444Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:25
2019-10-12T05:59:47.1165585Z    |
2019-10-12T05:59:47.1166891Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-12T05:59:47.1166964Z    |                         ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-12T05:59:47.1167088Z    |                         |        |          no `DummyResult` in `ext::base`
2019-10-12T05:59:47.1167150Z    |                         |        no `MacResult` in `ext::base`
2019-10-12T05:59:47.1167223Z    |                         no `ExtCtxt` in `ext::base`
2019-10-12T05:59:47.1167253Z 
2019-10-12T05:59:47.1167253Z 
2019-10-12T05:59:47.1167299Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-12T05:59:47.1167664Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:40:26
2019-10-12T05:59:47.1167736Z    |
2019-10-12T05:59:47.1167783Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-12T05:59:47.1167906Z help: possible candidates are found in other modules, you can import them into scope
2019-10-12T05:59:47.1167951Z    |
2019-10-12T05:59:47.1167992Z LL | use syntax::token::Token;
2019-10-12T05:59:47.1168052Z    |
2019-10-12T05:59:47.1168052Z    |
2019-10-12T05:59:47.1168094Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-12T05:59:47.1168303Z    |
2019-10-12T05:59:47.1168328Z 
2019-10-12T05:59:47.1168761Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-12T05:59:47.1169042Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-12T05:59:47.1169148Z LL | #[plugin_registrar]
2019-10-12T05:59:47.1169194Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-12T05:59:47.1169255Z    |
2019-10-12T05:59:47.1169299Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-12T05:59:47.1178161Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-12T05:59:47.1178235Z 
2019-10-12T05:59:47.1178261Z 
2019-10-12T05:59:47.1178285Z 
2019-10-12T05:59:47.1179962Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-12T05:59:47.1180224Z 
2019-10-12T05:59:47.1180252Z 
2019-10-12T05:59:47.1180299Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-12T05:59:47.1180371Z Build completed unsuccessfully in 1:08:39
2019-10-12T05:59:47.1180371Z Build completed unsuccessfully in 1:08:39
2019-10-12T05:59:47.1180418Z == clock drift check ==
2019-10-12T05:59:47.1180722Z   local time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-12T05:59:47.1180803Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-12T05:59:47.1181331Z Sat Oct 12 05:59:47 UTC 2019
2019-10-12T05:59:47.2175501Z   network time: Sat, 12 Oct 2019 05:59:47 GMT
2019-10-12T05:59:47.2179556Z == end clock drift check ==
2019-10-12T05:59:48.1364629Z ##[error]Bash exited with code '1'.
2019-10-12T05:59:48.1420583Z ##[section]Starting: Checkout
2019-10-12T05:59:48.1422846Z ==============================================================================
2019-10-12T05:59:48.1422909Z Task         : Get sources
2019-10-12T05:59:48.1422961Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
