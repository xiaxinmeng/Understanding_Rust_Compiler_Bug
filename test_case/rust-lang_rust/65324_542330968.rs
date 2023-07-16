plain
2019-10-15T16:45:00.8248287Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T16:45:00.8323014Z ##[command]git config gc.auto 0
2019-10-15T16:45:00.8391508Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T16:45:00.8446413Z ##[command]git config --get-all http.proxy
2019-10-15T16:45:00.8597335Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-15T16:54:08.1157295Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-15T16:54:15.7018153Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-15T16:54:21.9028725Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-15T16:54:37.5081859Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-15T16:55:19.1339559Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-15T16:56:02.6868886Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-15T17:00:46.3275780Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-15T17:01:54.3029557Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-15T17:04:56.4420395Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-15T17:05:39.1439826Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
---
2019-10-15T17:16:07.3640744Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-15T17:16:18.0660808Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-15T17:16:25.8121381Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-15T17:16:46.9238385Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-15T17:17:36.0777143Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-15T17:18:24.8259641Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-15T17:23:50.1466314Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-15T17:24:33.1427895Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-15T17:25:21.9331693Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-15T17:26:37.9624209Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
---
2019-10-15T17:41:21.7059009Z .................................................................................................... 1600/9182
2019-10-15T17:41:26.6881837Z .................................................................................................... 1700/9182
2019-10-15T17:41:39.1792784Z ............................i...............i....................................................... 1800/9182
2019-10-15T17:41:46.3326300Z .................................................................................................... 1900/9182
2019-10-15T17:42:00.2415938Z ...................iiiii............................................................................ 2000/9182
2019-10-15T17:42:10.2836116Z .................................................................................................... 2200/9182
2019-10-15T17:42:12.7336158Z .................................................................................................... 2300/9182
2019-10-15T17:42:17.8389197Z .................................................................................................... 2400/9182
2019-10-15T17:42:39.2716635Z .................................................................................................... 2500/9182
---
2019-10-15T17:45:26.2674518Z ...........................i...............i........................................................ 4800/9182
2019-10-15T17:45:37.8279266Z .................................................................................................... 4900/9182
2019-10-15T17:45:43.9900171Z .................................................................................................... 5000/9182
2019-10-15T17:45:52.8785392Z .................................................................................................... 5100/9182
2019-10-15T17:46:00.3331374Z ...........................ii.ii.................................................................... 5200/9182
2019-10-15T17:46:09.2101320Z .................................................................................................... 5400/9182
2019-10-15T17:46:19.2313550Z .............................................................................................i...... 5500/9182
2019-10-15T17:46:27.0045668Z .................................................................................................... 5600/9182
2019-10-15T17:46:31.4791965Z .................................................................................................... 5700/9182
2019-10-15T17:46:31.4791965Z .................................................................................................... 5700/9182
2019-10-15T17:46:41.6878850Z ..........................................................................................ii...i..ii 5800/9182
2019-10-15T17:47:05.7595527Z .................................................................................................... 6000/9182
2019-10-15T17:47:15.0583608Z .................................................................................................... 6100/9182
2019-10-15T17:47:21.2920642Z .................................................................................................i.. 6200/9182
2019-10-15T17:47:34.8311776Z ii.................................................................................................. 6300/9182
---
2019-10-15T17:52:07.4943302Z  finished in 10.114
2019-10-15T17:52:07.5101645Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:07.6496805Z 
2019-10-15T17:52:07.6497203Z running 153 tests
2019-10-15T17:52:10.3809317Z i....iii......iii..iiii...i..............................i.i..................i....i...........ii.i. 100/153
2019-10-15T17:52:12.0928849Z i..iiii..............i.........iii.i.........ii......
2019-10-15T17:52:12.0929515Z 
2019-10-15T17:52:12.0965444Z  finished in 4.583
2019-10-15T17:52:12.1110766Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:12.2519887Z 
---
2019-10-15T17:52:14.0249121Z  finished in 1.913
2019-10-15T17:52:14.0406066Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:14.1777848Z 
2019-10-15T17:52:14.1777950Z running 9 tests
2019-10-15T17:52:14.1778426Z iiiiiiiii
2019-10-15T17:52:14.1778710Z 
2019-10-15T17:52:14.1784163Z  finished in 0.137
2019-10-15T17:52:14.1924292Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:14.3405030Z 
---
2019-10-15T17:52:29.8818091Z  finished in 15.689
2019-10-15T17:52:29.8987344Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:30.0386857Z 
2019-10-15T17:52:30.0387397Z running 123 tests
2019-10-15T17:52:50.5827157Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-15T17:52:54.5855828Z i.i.i......iii.i.....ii
2019-10-15T17:52:54.5856618Z 
2019-10-15T17:52:54.5858479Z  finished in 24.686
2019-10-15T17:52:54.5864640Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:54.5865345Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-15T17:52:54.5865345Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-15T17:52:54.6045330Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T17:52:54.7403804Z 
2019-10-15T17:52:54.7404359Z running 69 tests
2019-10-15T17:53:16.0214265Z F............F....................................F...FFFFFFF..F.....
2019-10-15T17:53:16.0233744Z 
2019-10-15T17:53:16.0234616Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-15T17:53:16.0234723Z 
2019-10-15T17:53:16.0234995Z error: test compilation failed although it shouldn't!
2019-10-15T17:53:16.0234995Z error: test compilation failed although it shouldn't!
2019-10-15T17:53:16.0237152Z status: exit code: 1
2019-10-15T17:53:16.0238206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-15T17:53:16.0238524Z ------------------------------------------
2019-10-15T17:53:16.0238604Z 
2019-10-15T17:53:16.0239023Z ------------------------------------------
2019-10-15T17:53:16.0239165Z stderr:
---
2019-10-15T17:53:16.0239784Z 
2019-10-15T17:53:16.0239815Z error[E0432]: unresolved import `syntax::parse`
2019-10-15T17:53:16.0240029Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:15:13
2019-10-15T17:53:16.0240066Z    |
2019-10-15T17:53:16.0240099Z LL | use syntax::parse::{ParseSess, PResult};
2019-10-15T17:53:16.0240153Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-15T17:53:16.0240210Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-15T17:53:16.0240404Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:17:13
2019-10-15T17:53:16.0240464Z    |
2019-10-15T17:53:16.0240496Z LL | use syntax::parse::parser::Parser;
---
2019-10-15T17:53:16.0241723Z 
2019-10-15T17:53:16.0241755Z error[E0412]: cannot find type `Parser` in this scope
2019-10-15T17:53:16.0241938Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:26:71
2019-10-15T17:53:16.0241994Z    |
2019-10-15T17:53:16.0242186Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-15T17:53:16.0242435Z 
2019-10-15T17:53:16.0242480Z error[E0412]: cannot find type `Parser` in this scope
2019-10-15T17:53:16.0242691Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:31:20
2019-10-15T17:53:16.0242745Z    |
2019-10-15T17:53:16.0242745Z    |
2019-10-15T17:53:16.0242921Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-15T17:53:16.0242982Z 
2019-10-15T17:53:16.0243032Z error: aborting due to 8 previous errors
2019-10-15T17:53:16.0243053Z 
2019-10-15T17:53:16.0243087Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-15T17:53:16.0243087Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-15T17:53:16.0243735Z For more information about an error, try `rustc --explain E0412`.
2019-10-15T17:53:16.0243798Z 
2019-10-15T17:53:16.0244054Z ------------------------------------------
2019-10-15T17:53:16.0244087Z 
2019-10-15T17:53:16.0244113Z 
2019-10-15T17:53:16.0244435Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-15T17:53:16.0244469Z 
2019-10-15T17:53:16.0244882Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T17:53:16.0244947Z status: exit code: 1
2019-10-15T17:53:16.0245848Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-15T17:53:16.0246192Z ------------------------------------------
2019-10-15T17:53:16.0246225Z 
2019-10-15T17:53:16.0246446Z ------------------------------------------
2019-10-15T17:53:16.0246511Z stderr:
2019-10-15T17:53:16.0246511Z stderr:
2019-10-15T17:53:16.0246731Z ------------------------------------------
2019-10-15T17:53:16.0246794Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T17:53:16.0247731Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T17:53:16.0247785Z    |
2019-10-15T17:53:16.0247820Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T17:53:16.0247881Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0247907Z 
2019-10-15T17:53:16.0248342Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0248588Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T17:53:16.0248661Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0248715Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0248749Z    |
2019-10-15T17:53:16.0248784Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0249545Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-15T17:53:16.0249591Z 
2019-10-15T17:53:16.0249769Z error: test compilation failed although it shouldn't!
2019-10-15T17:53:16.0249806Z status: exit code: 1
2019-10-15T17:53:16.0250552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-15T17:53:16.0250811Z ------------------------------------------
2019-10-15T17:53:16.0250836Z 
2019-10-15T17:53:16.0250998Z ------------------------------------------
2019-10-15T17:53:16.0251049Z stderr:
2019-10-15T17:53:16.0251049Z stderr:
2019-10-15T17:53:16.0251213Z ------------------------------------------
2019-10-15T17:53:16.0251251Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-15T17:53:16.0251495Z    |
2019-10-15T17:53:16.0251495Z    |
2019-10-15T17:53:16.0251528Z LL | use syntax::parse::{self, ParseSess};
2019-10-15T17:53:16.0251563Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-15T17:53:16.0251650Z    |             could not find `parse` in `syntax`
2019-10-15T17:53:16.0251773Z 
2019-10-15T17:53:16.0251834Z error: aborting due to previous error
2019-10-15T17:53:16.0251908Z 
2019-10-15T17:53:16.0251908Z 
2019-10-15T17:53:16.0252126Z For more information about this error, try `rustc --explain E0432`.
2019-10-15T17:53:16.0252152Z 
2019-10-15T17:53:16.0252330Z ------------------------------------------
2019-10-15T17:53:16.0252353Z 
2019-10-15T17:53:16.0252373Z 
2019-10-15T17:53:16.0252543Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-15T17:53:16.0252566Z 
2019-10-15T17:53:16.0252789Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T17:53:16.0252830Z status: exit code: 1
2019-10-15T17:53:16.0253931Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-15T17:53:16.0254286Z ------------------------------------------
2019-10-15T17:53:16.0254320Z 
2019-10-15T17:53:16.0254541Z ------------------------------------------
2019-10-15T17:53:16.0254587Z stderr:
2019-10-15T17:53:16.0254587Z stderr:
2019-10-15T17:53:16.0254827Z ------------------------------------------
2019-10-15T17:53:16.0254885Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T17:53:16.0255135Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T17:53:16.0255207Z    |
2019-10-15T17:53:16.0255255Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T17:53:16.0255319Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T17:53:16.0255399Z    |                         |
2019-10-15T17:53:16.0255448Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0255480Z 
2019-10-15T17:53:16.0255557Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T17:53:16.0255825Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T17:53:16.0255875Z    |
2019-10-15T17:53:16.0255943Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T17:53:16.0256001Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T17:53:16.0256127Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T17:53:16.0256188Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0256188Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0256249Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T17:53:16.0256301Z 
2019-10-15T17:53:16.0256820Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0257011Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T17:53:16.0257096Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0257130Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0257178Z    |
2019-10-15T17:53:16.0257213Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0257808Z 
2019-10-15T17:53:16.0257876Z 
2019-10-15T17:53:16.0258070Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-15T17:53:16.0258095Z 
2019-10-15T17:53:16.0258312Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T17:53:16.0258351Z status: exit code: 1
2019-10-15T17:53:16.0258901Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-15T17:53:16.0259159Z ------------------------------------------
2019-10-15T17:53:16.0259207Z 
2019-10-15T17:53:16.0259371Z ------------------------------------------
2019-10-15T17:53:16.0259404Z stderr:
2019-10-15T17:53:16.0259404Z stderr:
2019-10-15T17:53:16.0259560Z ------------------------------------------
2019-10-15T17:53:16.0259620Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T17:53:16.0259804Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T17:53:16.0259840Z    |
2019-10-15T17:53:16.0259893Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T17:53:16.0259935Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T17:53:16.0259971Z    |                         |
2019-10-15T17:53:16.0260025Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0260049Z 
2019-10-15T17:53:16.0260097Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T17:53:16.0260320Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T17:53:16.0260358Z    |
2019-10-15T17:53:16.0260393Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T17:53:16.0260453Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T17:53:16.0260531Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T17:53:16.0260589Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0260589Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0260626Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T17:53:16.0260649Z 
2019-10-15T17:53:16.0260930Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0261132Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T17:53:16.0261219Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0261253Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0261300Z    |
2019-10-15T17:53:16.0261333Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0261826Z 
2019-10-15T17:53:16.0261846Z 
2019-10-15T17:53:16.0262011Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-15T17:53:16.0262034Z 
2019-10-15T17:53:16.0262318Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-15T17:53:16.0262404Z status: exit code: 1
2019-10-15T17:53:16.0263115Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-15T17:53:16.0263758Z ------------------------------------------
2019-10-15T17:53:16.0263809Z 
2019-10-15T17:53:16.0264032Z ------------------------------------------
2019-10-15T17:53:16.0264077Z stderr:
2019-10-15T17:53:16.0264077Z stderr:
2019-10-15T17:53:16.0264318Z ------------------------------------------
2019-10-15T17:53:16.0264378Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-15T17:53:16.0264641Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-15T17:53:16.0264708Z    |
2019-10-15T17:53:16.0264757Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-15T17:53:16.0264813Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-15T17:53:16.0264880Z    |                         |
2019-10-15T17:53:16.0264931Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0264963Z 
2019-10-15T17:53:16.0265021Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-15T17:53:16.0265300Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-15T17:53:16.0265351Z    |
2019-10-15T17:53:16.0265408Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-15T17:53:16.0265493Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T17:53:16.0265598Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-15T17:53:16.0265669Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0265669Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0265721Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-15T17:53:16.0265752Z 
2019-10-15T17:53:16.0266129Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0266386Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-15T17:53:16.0266496Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0266552Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0266775Z    |
2019-10-15T17:53:16.0266827Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0267468Z 
2019-10-15T17:53:16.0267502Z 
2019-10-15T17:53:16.0267673Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-15T17:53:16.0267697Z 
2019-10-15T17:53:16.0267899Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T17:53:16.0267953Z status: exit code: 1
2019-10-15T17:53:16.0268602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-15T17:53:16.0268933Z ------------------------------------------
2019-10-15T17:53:16.0268958Z 
2019-10-15T17:53:16.0269136Z ------------------------------------------
2019-10-15T17:53:16.0269169Z stderr:
2019-10-15T17:53:16.0269169Z stderr:
2019-10-15T17:53:16.0269325Z ------------------------------------------
2019-10-15T17:53:16.0269378Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T17:53:16.0269563Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T17:53:16.0269600Z    |
2019-10-15T17:53:16.0269654Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T17:53:16.0269698Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0269723Z 
2019-10-15T17:53:16.0270006Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0270203Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T17:53:16.0270286Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0270320Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0270352Z    |
2019-10-15T17:53:16.0270401Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0270889Z 
2019-10-15T17:53:16.0270909Z 
2019-10-15T17:53:16.0271098Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-15T17:53:16.0271123Z 
2019-10-15T17:53:16.0271324Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T17:53:16.0271378Z status: exit code: 1
2019-10-15T17:53:16.0271955Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-15T17:53:16.0272209Z ------------------------------------------
2019-10-15T17:53:16.0272233Z 
2019-10-15T17:53:16.0272406Z ------------------------------------------
2019-10-15T17:53:16.0272441Z stderr:
2019-10-15T17:53:16.0272441Z stderr:
2019-10-15T17:53:16.0272600Z ------------------------------------------
2019-10-15T17:53:16.0272639Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T17:53:16.0272843Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T17:53:16.0272879Z    |
2019-10-15T17:53:16.0272912Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T17:53:16.0272966Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0272991Z 
2019-10-15T17:53:16.0273716Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0274122Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T17:53:16.0274326Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0274373Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0274416Z    |
2019-10-15T17:53:16.0274462Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0275155Z 
2019-10-15T17:53:16.0275181Z 
2019-10-15T17:53:16.0275422Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-15T17:53:16.0275456Z 
2019-10-15T17:53:16.0275735Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-15T17:53:16.0275789Z status: exit code: 1
2019-10-15T17:53:16.0276578Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-15T17:53:16.0277183Z ------------------------------------------
2019-10-15T17:53:16.0277208Z 
2019-10-15T17:53:16.0277366Z ------------------------------------------
2019-10-15T17:53:16.0277415Z stderr:
2019-10-15T17:53:16.0277415Z stderr:
2019-10-15T17:53:16.0277572Z ------------------------------------------
2019-10-15T17:53:16.0277611Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-15T17:53:16.0277818Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-15T17:53:16.0277861Z    |
2019-10-15T17:53:16.0277894Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-15T17:53:16.0277947Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-15T17:53:16.0277970Z 
2019-10-15T17:53:16.0278230Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0278440Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-15T17:53:16.0278508Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0278558Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0278590Z    |
2019-10-15T17:53:16.0278623Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0279315Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-15T17:53:16.0279356Z 
2019-10-15T17:53:16.0279521Z error: test compilation failed although it shouldn't!
2019-10-15T17:53:16.0279557Z status: exit code: 1
2019-10-15T17:53:16.0280175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-15T17:53:16.0280489Z ------------------------------------------
2019-10-15T17:53:16.0280514Z 
2019-10-15T17:53:16.0280673Z ------------------------------------------
2019-10-15T17:53:16.0280706Z stderr:
2019-10-15T17:53:16.0280706Z stderr:
2019-10-15T17:53:16.0280881Z ------------------------------------------
2019-10-15T17:53:16.0280919Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-15T17:53:16.0281153Z    |
2019-10-15T17:53:16.0281153Z    |
2019-10-15T17:53:16.0281186Z LL | use syntax::parse::{self, ParseSess};
2019-10-15T17:53:16.0281221Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-15T17:53:16.0281306Z    |             could not find `parse` in `syntax`
2019-10-15T17:53:16.0281329Z 
2019-10-15T17:53:16.0281361Z error: aborting due to previous error
2019-10-15T17:53:16.0281398Z 
2019-10-15T17:53:16.0281398Z 
2019-10-15T17:53:16.0281587Z For more information about this error, try `rustc --explain E0432`.
2019-10-15T17:53:16.0281617Z 
2019-10-15T17:53:16.0281776Z ------------------------------------------
2019-10-15T17:53:16.0281816Z 
2019-10-15T17:53:16.0281835Z 
2019-10-15T17:53:16.0282004Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-15T17:53:16.0282028Z 
2019-10-15T17:53:16.0282243Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-15T17:53:16.0282283Z status: exit code: 1
2019-10-15T17:53:16.0282865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-15T17:53:16.0283581Z ------------------------------------------
2019-10-15T17:53:16.0283621Z 
2019-10-15T17:53:16.0283862Z ------------------------------------------
2019-10-15T17:53:16.0283920Z stderr:
2019-10-15T17:53:16.0283920Z stderr:
2019-10-15T17:53:16.0284153Z ------------------------------------------
2019-10-15T17:53:16.0284206Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-15T17:53:16.0284458Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-15T17:53:16.0284527Z    |
2019-10-15T17:53:16.0284574Z LL | use syntax::parse::token::{self, Token};
2019-10-15T17:53:16.0284622Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-15T17:53:16.0284716Z error[E0432]: unresolved import `syntax::parse`
2019-10-15T17:53:16.0284981Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-15T17:53:16.0285039Z    |
2019-10-15T17:53:16.0285039Z    |
2019-10-15T17:53:16.0285103Z LL | use syntax::parse::token::{self, Token};
2019-10-15T17:53:16.0285152Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-15T17:53:16.0285183Z 
2019-10-15T17:53:16.0285255Z error[E0432]: unresolved imports `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::DummyResult`, `syntax::ext::base::MacEager`
2019-10-15T17:53:16.0285519Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:25
2019-10-15T17:53:16.0285570Z    |
2019-10-15T17:53:16.0285635Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-15T17:53:16.0285692Z    |                         ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-15T17:53:16.0285813Z    |                         |        |          no `DummyResult` in `ext::base`
2019-10-15T17:53:16.0285975Z    |                         |        no `MacResult` in `ext::base`
2019-10-15T17:53:16.0286090Z    |                         no `ExtCtxt` in `ext::base`
2019-10-15T17:53:16.0286121Z 
2019-10-15T17:53:16.0286121Z 
2019-10-15T17:53:16.0286187Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-15T17:53:16.0286469Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:40:26
2019-10-15T17:53:16.0286517Z    |
2019-10-15T17:53:16.0286585Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-15T17:53:16.0286866Z help: possible candidates are found in other modules, you can import them into scope
2019-10-15T17:53:16.0286920Z    |
2019-10-15T17:53:16.0287152Z LL | use syntax::token::Token;
2019-10-15T17:53:16.0287186Z    |
2019-10-15T17:53:16.0287186Z    |
2019-10-15T17:53:16.0287576Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-15T17:53:16.0287618Z    |
2019-10-15T17:53:16.0287637Z 
2019-10-15T17:53:16.0287992Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-15T17:53:16.0288205Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-15T17:53:16.0288287Z LL | #[plugin_registrar]
2019-10-15T17:53:16.0288322Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-15T17:53:16.0288353Z    |
2019-10-15T17:53:16.0288386Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-15T17:53:16.0291071Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-15T17:53:16.0291097Z 
2019-10-15T17:53:16.0291123Z 
2019-10-15T17:53:16.0291156Z 
2019-10-15T17:53:16.0292361Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-15T17:53:16.0292620Z 
2019-10-15T17:53:16.0292642Z 
2019-10-15T17:53:16.0292676Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-15T17:53:16.0292714Z Build completed unsuccessfully in 1:01:55
2019-10-15T17:53:16.0292714Z Build completed unsuccessfully in 1:01:55
2019-10-15T17:53:16.0292961Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-15T17:53:16.0293005Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-15T17:53:16.0293089Z == clock drift check ==
2019-10-15T17:53:16.0318154Z   local time: Tue Oct 15 17:53:16 UTC 2019
2019-10-15T17:53:16.3149649Z   network time: Tue, 15 Oct 2019 17:53:16 GMT
2019-10-15T17:53:16.3149890Z == end clock drift check ==
2019-10-15T17:53:17.1975266Z ##[error]Bash exited with code '1'.
2019-10-15T17:53:17.2021219Z ##[section]Starting: Checkout
2019-10-15T17:53:17.2022835Z ==============================================================================
2019-10-15T17:53:17.2022895Z Task         : Get sources
2019-10-15T17:53:17.2022930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
