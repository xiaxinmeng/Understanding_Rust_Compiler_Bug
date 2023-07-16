plain
2019-10-13T03:44:41.9888898Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T03:44:42.0009502Z ##[command]git config gc.auto 0
2019-10-13T03:44:42.0071846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T03:44:42.0121139Z ##[command]git config --get-all http.proxy
2019-10-13T03:44:42.0299667Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-13T03:54:00.0353391Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-13T03:54:08.9628167Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-13T03:54:10.4277046Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-13T03:54:27.6678368Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-13T03:55:14.9077539Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-13T03:56:00.3154272Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-13T04:01:11.2795306Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-13T04:02:00.0266858Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-13T04:02:42.3988761Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-13T04:05:55.4319520Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
---
2019-10-13T04:18:13.4572963Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-13T04:18:24.5794870Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-13T04:18:32.8430243Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-13T04:18:54.8725437Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-13T04:19:51.5115552Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-13T04:20:48.2342381Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-13T04:27:07.7078052Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-13T04:27:59.2125757Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-13T04:28:56.4818351Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-13T04:30:24.4597142Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
---
2019-10-13T04:47:07.8803962Z .................................................................................................... 1600/9171
2019-10-13T04:47:14.5529500Z .................................................................................................... 1700/9171
2019-10-13T04:47:27.0893742Z ......................i...............i............................................................. 1800/9171
2019-10-13T04:47:34.5539813Z .................................................................................................... 1900/9171
2019-10-13T04:47:49.4614209Z .............iiiii.................................................................................. 2000/9171
2019-10-13T04:48:00.2106907Z .................................................................................................... 2200/9171
2019-10-13T04:48:02.9331197Z .................................................................................................... 2300/9171
2019-10-13T04:48:08.7185582Z .................................................................................................... 2400/9171
2019-10-13T04:48:31.2599396Z .................................................................................................... 2500/9171
---
2019-10-13T04:51:34.4753580Z ....................i...............i............................................................... 4800/9171
2019-10-13T04:51:46.4528693Z .................................................................................................... 4900/9171
2019-10-13T04:51:52.9853231Z .................................................................................................... 5000/9171
2019-10-13T04:52:03.9208881Z .................................................................................................... 5100/9171
2019-10-13T04:52:10.3504546Z ....................ii.ii........................................................................... 5200/9171
2019-10-13T04:52:21.3666620Z .................................................................................................... 5400/9171
2019-10-13T04:52:31.8159240Z ......................................................................................i............. 5500/9171
2019-10-13T04:52:40.2053487Z .................................................................................................... 5600/9171
2019-10-13T04:52:45.9014636Z .................................................................................................... 5700/9171
2019-10-13T04:52:45.9014636Z .................................................................................................... 5700/9171
2019-10-13T04:52:56.8048600Z ...................................................................................ii...i..ii....... 5800/9171
2019-10-13T04:53:22.9450455Z .................................................................................................... 6000/9171
2019-10-13T04:53:33.2564122Z .................................................................................................... 6100/9171
2019-10-13T04:53:33.2564122Z .................................................................................................... 6100/9171
2019-10-13T04:53:41.3530181Z .........................................................................................i..ii...... 6200/9171
2019-10-13T04:54:10.7984248Z .................................................................................................... 6400/9171
2019-10-13T04:54:15.2099334Z .................................................i.................................................. 6500/9171
2019-10-13T04:54:17.5475035Z .................................................................................................... 6600/9171
2019-10-13T04:54:20.1256891Z ......................i............................................................................. 6700/9171
---
2019-10-13T04:59:05.6908162Z  finished in 5.837
2019-10-13T04:59:05.7100825Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T04:59:05.8685491Z 
2019-10-13T04:59:05.8685886Z running 153 tests
2019-10-13T04:59:09.2511347Z i....iii......iii..iiii....i.............................i..i..................i....i............ii. 100/153
2019-10-13T04:59:11.3173555Z i.i..iiii..............i.........iii.i.......ii......
2019-10-13T04:59:11.3174180Z 
2019-10-13T04:59:11.3178055Z  finished in 5.607
2019-10-13T04:59:11.3376564Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T04:59:11.5036840Z 
---
2019-10-13T04:59:13.6377229Z  finished in 2.300
2019-10-13T04:59:13.6586298Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T04:59:13.8358271Z 
2019-10-13T04:59:13.8359294Z running 9 tests
2019-10-13T04:59:13.8360357Z iiiiiiiii
2019-10-13T04:59:13.8361140Z 
2019-10-13T04:59:13.8361189Z  finished in 0.177
2019-10-13T04:59:13.8565252Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T04:59:14.0404921Z 
---
2019-10-13T04:59:32.9040946Z  finished in 19.047
2019-10-13T04:59:32.9239388Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T04:59:33.0843004Z 
2019-10-13T04:59:33.0844242Z running 123 tests
2019-10-13T04:59:58.5664452Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-13T05:00:03.5487463Z i.i.i......iii.i.....ii
2019-10-13T05:00:03.5489360Z 
2019-10-13T05:00:03.5495413Z  finished in 30.625
2019-10-13T05:00:03.5507238Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T05:00:03.5509244Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-13T05:00:03.5509244Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-13T05:00:03.5766808Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T05:00:03.7471904Z 
2019-10-13T05:00:03.7472747Z running 69 tests
2019-10-13T05:00:29.4528562Z F............F....................................F...FFFFFFF..F.....
2019-10-13T05:00:29.4563599Z 
2019-10-13T05:00:29.4564180Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-13T05:00:29.4564279Z 
2019-10-13T05:00:29.4564550Z error: test compilation failed although it shouldn't!
2019-10-13T05:00:29.4564550Z error: test compilation failed although it shouldn't!
2019-10-13T05:00:29.4564629Z status: exit code: 1
2019-10-13T05:00:29.4565669Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-13T05:00:29.4566139Z ------------------------------------------
2019-10-13T05:00:29.4566181Z 
2019-10-13T05:00:29.4566445Z ------------------------------------------
2019-10-13T05:00:29.4566497Z stderr:
---
2019-10-13T05:00:29.4567300Z 
2019-10-13T05:00:29.4567349Z error[E0432]: unresolved import `syntax::parse`
2019-10-13T05:00:29.4567644Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:15:13
2019-10-13T05:00:29.4567700Z    |
2019-10-13T05:00:29.4567748Z LL | use syntax::parse::{ParseSess, PResult};
2019-10-13T05:00:29.4567812Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T05:00:29.4567925Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-13T05:00:29.4568204Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:17:13
2019-10-13T05:00:29.4568276Z    |
2019-10-13T05:00:29.4568325Z LL | use syntax::parse::parser::Parser;
---
2019-10-13T05:00:29.4570694Z 
2019-10-13T05:00:29.4570743Z error[E0412]: cannot find type `Parser` in this scope
2019-10-13T05:00:29.4571023Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:26:71
2019-10-13T05:00:29.4571095Z    |
2019-10-13T05:00:29.4571387Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-13T05:00:29.4571530Z 
2019-10-13T05:00:29.4571611Z error[E0412]: cannot find type `Parser` in this scope
2019-10-13T05:00:29.4571953Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:31:20
2019-10-13T05:00:29.4572008Z    |
2019-10-13T05:00:29.4572008Z    |
2019-10-13T05:00:29.4572284Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-13T05:00:29.4572378Z 
2019-10-13T05:00:29.4572425Z error: aborting due to 8 previous errors
2019-10-13T05:00:29.4572475Z 
2019-10-13T05:00:29.4572525Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-13T05:00:29.4572525Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-13T05:00:29.4573402Z For more information about an error, try `rustc --explain E0412`.
2019-10-13T05:00:29.4573450Z 
2019-10-13T05:00:29.4573734Z ------------------------------------------
2019-10-13T05:00:29.4573771Z 
2019-10-13T05:00:29.4573800Z 
2019-10-13T05:00:29.4574211Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-13T05:00:29.4574293Z 
2019-10-13T05:00:29.4574661Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T05:00:29.4574721Z status: exit code: 1
2019-10-13T05:00:29.4575613Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-13T05:00:29.4576071Z ------------------------------------------
2019-10-13T05:00:29.4576121Z 
2019-10-13T05:00:29.4576371Z ------------------------------------------
2019-10-13T05:00:29.4576431Z stderr:
2019-10-13T05:00:29.4576431Z stderr:
2019-10-13T05:00:29.4576692Z ------------------------------------------
2019-10-13T05:00:29.4576751Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T05:00:29.4577034Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T05:00:29.4577108Z    |
2019-10-13T05:00:29.4577158Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T05:00:29.4577216Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4577268Z 
2019-10-13T05:00:29.4577778Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4578135Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T05:00:29.4578265Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4578330Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4578542Z    |
2019-10-13T05:00:29.4578596Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4579765Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-13T05:00:29.4582125Z 
2019-10-13T05:00:29.4582539Z error: test compilation failed although it shouldn't!
2019-10-13T05:00:29.4582595Z status: exit code: 1
2019-10-13T05:00:29.4588010Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-13T05:00:29.4588559Z ------------------------------------------
2019-10-13T05:00:29.4588621Z 
2019-10-13T05:00:29.4588878Z ------------------------------------------
2019-10-13T05:00:29.4588928Z stderr:
2019-10-13T05:00:29.4588928Z stderr:
2019-10-13T05:00:29.4589183Z ------------------------------------------
2019-10-13T05:00:29.4589243Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-13T05:00:29.4590577Z    |
2019-10-13T05:00:29.4590577Z    |
2019-10-13T05:00:29.4590627Z LL | use syntax::parse::{self, ParseSess};
2019-10-13T05:00:29.4590877Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-13T05:00:29.4591024Z    |             could not find `parse` in `syntax`
2019-10-13T05:00:29.4591061Z 
2019-10-13T05:00:29.4591109Z error: aborting due to previous error
2019-10-13T05:00:29.4591141Z 
2019-10-13T05:00:29.4591141Z 
2019-10-13T05:00:29.4591815Z For more information about this error, try `rustc --explain E0432`.
2019-10-13T05:00:29.4591873Z 
2019-10-13T05:00:29.4592123Z ------------------------------------------
2019-10-13T05:00:29.4643326Z 
2019-10-13T05:00:29.4643372Z 
2019-10-13T05:00:29.4643841Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-13T05:00:29.4643877Z 
2019-10-13T05:00:29.4644258Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T05:00:29.4644319Z status: exit code: 1
2019-10-13T05:00:29.4645059Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-13T05:00:29.4645390Z ------------------------------------------
2019-10-13T05:00:29.4645435Z 
2019-10-13T05:00:29.4645646Z ------------------------------------------
2019-10-13T05:00:29.4645809Z stderr:
2019-10-13T05:00:29.4645809Z stderr:
2019-10-13T05:00:29.4646036Z ------------------------------------------
2019-10-13T05:00:29.4646095Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T05:00:29.4646348Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T05:00:29.4646423Z    |
2019-10-13T05:00:29.4646691Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T05:00:29.4646747Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T05:00:29.4646810Z    |                         |
2019-10-13T05:00:29.4646859Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4646890Z 
2019-10-13T05:00:29.4646946Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T05:00:29.4647379Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T05:00:29.4647430Z    |
2019-10-13T05:00:29.4647611Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T05:00:29.4647664Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T05:00:29.4647791Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T05:00:29.4647841Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4647841Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4647887Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T05:00:29.4647916Z 
2019-10-13T05:00:29.4648292Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4648544Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T05:00:29.4648643Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4648685Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4648839Z    |
2019-10-13T05:00:29.4648878Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4649588Z 
2019-10-13T05:00:29.4649627Z 
2019-10-13T05:00:29.4650277Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-13T05:00:29.4650317Z 
2019-10-13T05:00:29.4650650Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T05:00:29.4650720Z status: exit code: 1
2019-10-13T05:00:29.4651462Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-13T05:00:29.4651805Z ------------------------------------------
2019-10-13T05:00:29.4651838Z 
2019-10-13T05:00:29.4652069Z ------------------------------------------
2019-10-13T05:00:29.4652112Z stderr:
2019-10-13T05:00:29.4652112Z stderr:
2019-10-13T05:00:29.4652324Z ------------------------------------------
2019-10-13T05:00:29.4652391Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T05:00:29.4652641Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T05:00:29.4652690Z    |
2019-10-13T05:00:29.4652747Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T05:00:29.4652801Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T05:00:29.4652858Z    |                         |
2019-10-13T05:00:29.4653053Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4653084Z 
2019-10-13T05:00:29.4653140Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T05:00:29.4653570Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T05:00:29.4653617Z    |
2019-10-13T05:00:29.4653662Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T05:00:29.4653729Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T05:00:29.4653827Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T05:00:29.4653892Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4653892Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4653948Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T05:00:29.4653983Z 
2019-10-13T05:00:29.4654336Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4654616Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T05:00:29.4654717Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4654761Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4654804Z    |
2019-10-13T05:00:29.4654860Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4656084Z 
2019-10-13T05:00:29.4656116Z 
2019-10-13T05:00:29.4656598Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-13T05:00:29.4656651Z 
2019-10-13T05:00:29.4657331Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T05:00:29.4657425Z status: exit code: 1
2019-10-13T05:00:29.4658223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-13T05:00:29.4658534Z ------------------------------------------
2019-10-13T05:00:29.4658580Z 
2019-10-13T05:00:29.4658799Z ------------------------------------------
2019-10-13T05:00:29.4658855Z stderr:
2019-10-13T05:00:29.4658855Z stderr:
2019-10-13T05:00:29.4659060Z ------------------------------------------
2019-10-13T05:00:29.4659115Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T05:00:29.4659367Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T05:00:29.4659414Z    |
2019-10-13T05:00:29.4659459Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T05:00:29.4659527Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T05:00:29.4659697Z    |                         |
2019-10-13T05:00:29.4659742Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4659787Z 
2019-10-13T05:00:29.4660373Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T05:00:29.4660898Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T05:00:29.4660970Z    |
2019-10-13T05:00:29.4661018Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T05:00:29.4661074Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T05:00:29.4661189Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T05:00:29.4661241Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4661241Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4661302Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T05:00:29.4661333Z 
2019-10-13T05:00:29.4661703Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4661992Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T05:00:29.4662089Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4662149Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4662192Z    |
2019-10-13T05:00:29.4662235Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4662928Z 
2019-10-13T05:00:29.4662954Z 
2019-10-13T05:00:29.4663181Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-13T05:00:29.4663227Z 
2019-10-13T05:00:29.4663501Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T05:00:29.4663655Z status: exit code: 1
2019-10-13T05:00:29.4664697Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-13T05:00:29.4665028Z ------------------------------------------
2019-10-13T05:00:29.4665488Z 
2019-10-13T05:00:29.4665834Z ------------------------------------------
2019-10-13T05:00:29.4665896Z stderr:
2019-10-13T05:00:29.4665896Z stderr:
2019-10-13T05:00:29.4666113Z ------------------------------------------
2019-10-13T05:00:29.4666177Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T05:00:29.4666993Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T05:00:29.4667055Z    |
2019-10-13T05:00:29.4667097Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T05:00:29.4667157Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4667189Z 
2019-10-13T05:00:29.4667545Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4667819Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T05:00:29.4667906Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4667958Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4668001Z    |
2019-10-13T05:00:29.4668043Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4669122Z 
2019-10-13T05:00:29.4669146Z 
2019-10-13T05:00:29.4669367Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-13T05:00:29.4669413Z 
2019-10-13T05:00:29.4669676Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T05:00:29.4669727Z status: exit code: 1
2019-10-13T05:00:29.4671095Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-13T05:00:29.4671452Z ------------------------------------------
2019-10-13T05:00:29.4671485Z 
2019-10-13T05:00:29.4671700Z ------------------------------------------
2019-10-13T05:00:29.4671745Z stderr:
2019-10-13T05:00:29.4671745Z stderr:
2019-10-13T05:00:29.4671967Z ------------------------------------------
2019-10-13T05:00:29.4672018Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T05:00:29.4672266Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T05:00:29.4672330Z    |
2019-10-13T05:00:29.4672372Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T05:00:29.4672422Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4672463Z 
2019-10-13T05:00:29.4672944Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4673289Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T05:00:29.4673381Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4673426Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4673489Z    |
2019-10-13T05:00:29.4673532Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4674179Z 
2019-10-13T05:00:29.4674204Z 
2019-10-13T05:00:29.4674425Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-13T05:00:29.4674456Z 
2019-10-13T05:00:29.4674750Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T05:00:29.4674809Z status: exit code: 1
2019-10-13T05:00:29.4675541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-13T05:00:29.4675870Z ------------------------------------------
2019-10-13T05:00:29.4675903Z 
2019-10-13T05:00:29.4676116Z ------------------------------------------
2019-10-13T05:00:29.4676276Z stderr:
2019-10-13T05:00:29.4676276Z stderr:
2019-10-13T05:00:29.4676494Z ------------------------------------------
2019-10-13T05:00:29.4676636Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T05:00:29.4677033Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T05:00:29.4677097Z    |
2019-10-13T05:00:29.4677138Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T05:00:29.4677184Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T05:00:29.4677214Z 
2019-10-13T05:00:29.4677652Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4677913Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T05:00:29.4678012Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4678054Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4678105Z    |
2019-10-13T05:00:29.4678156Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4679564Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-13T05:00:29.4679595Z 
2019-10-13T05:00:29.4680255Z error: test compilation failed although it shouldn't!
2019-10-13T05:00:29.4680321Z status: exit code: 1
2019-10-13T05:00:29.4681251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-13T05:00:29.4681645Z ------------------------------------------
2019-10-13T05:00:29.4681679Z 
2019-10-13T05:00:29.4681909Z ------------------------------------------
2019-10-13T05:00:29.4681953Z stderr:
2019-10-13T05:00:29.4681953Z stderr:
2019-10-13T05:00:29.4682161Z ------------------------------------------
2019-10-13T05:00:29.4682225Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-13T05:00:29.4682523Z    |
2019-10-13T05:00:29.4682523Z    |
2019-10-13T05:00:29.4682582Z LL | use syntax::parse::{self, ParseSess};
2019-10-13T05:00:29.4682629Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-13T05:00:29.4682742Z    |             could not find `parse` in `syntax`
2019-10-13T05:00:29.4682779Z 
2019-10-13T05:00:29.4682821Z error: aborting due to previous error
2019-10-13T05:00:29.4682850Z 
2019-10-13T05:00:29.4682850Z 
2019-10-13T05:00:29.4683111Z For more information about this error, try `rustc --explain E0432`.
2019-10-13T05:00:29.4683145Z 
2019-10-13T05:00:29.4683470Z ------------------------------------------
2019-10-13T05:00:29.4683500Z 
2019-10-13T05:00:29.4683523Z 
2019-10-13T05:00:29.4683746Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-13T05:00:29.4683776Z 
2019-10-13T05:00:29.4684027Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-13T05:00:29.4684092Z status: exit code: 1
2019-10-13T05:00:29.4685063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-13T05:00:29.4685546Z ------------------------------------------
2019-10-13T05:00:29.4685580Z 
2019-10-13T05:00:29.4685812Z ------------------------------------------
2019-10-13T05:00:29.4685856Z stderr:
2019-10-13T05:00:29.4685856Z stderr:
2019-10-13T05:00:29.4686065Z ------------------------------------------
2019-10-13T05:00:29.4686131Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-13T05:00:29.4686380Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-13T05:00:29.4686429Z    |
2019-10-13T05:00:29.4686483Z LL | use syntax::parse::token::{self, Token};
2019-10-13T05:00:29.4686542Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T05:00:29.4686623Z error[E0432]: unresolved import `syntax::parse`
2019-10-13T05:00:29.4686886Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-13T05:00:29.4686933Z    |
2019-10-13T05:00:29.4686933Z    |
2019-10-13T05:00:29.4686976Z LL | use syntax::parse::token::{self, Token};
2019-10-13T05:00:29.4687036Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T05:00:29.4687065Z 
2019-10-13T05:00:29.4687122Z error[E0432]: unresolved imports `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::DummyResult`, `syntax::ext::base::MacEager`
2019-10-13T05:00:29.4687392Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:25
2019-10-13T05:00:29.4687441Z    |
2019-10-13T05:00:29.4687488Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-13T05:00:29.4687558Z    |                         ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T05:00:29.4687884Z    |                         |        |          no `DummyResult` in `ext::base`
2019-10-13T05:00:29.4688052Z    |                         |        no `MacResult` in `ext::base`
2019-10-13T05:00:29.4688098Z    |                         no `ExtCtxt` in `ext::base`
2019-10-13T05:00:29.4688126Z 
2019-10-13T05:00:29.4688126Z 
2019-10-13T05:00:29.4688169Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-13T05:00:29.4688451Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:40:26
2019-10-13T05:00:29.4688494Z    |
2019-10-13T05:00:29.4688540Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-13T05:00:29.4688647Z help: possible candidates are found in other modules, you can import them into scope
2019-10-13T05:00:29.4688689Z    |
2019-10-13T05:00:29.4688747Z LL | use syntax::token::Token;
2019-10-13T05:00:29.4688791Z    |
2019-10-13T05:00:29.4688791Z    |
2019-10-13T05:00:29.4688832Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-13T05:00:29.4688881Z    |
2019-10-13T05:00:29.4688906Z 
2019-10-13T05:00:29.4689250Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T05:00:29.4693074Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-13T05:00:29.4693190Z LL | #[plugin_registrar]
2019-10-13T05:00:29.4693235Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T05:00:29.4693418Z    |
2019-10-13T05:00:29.4693458Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T05:00:29.4700350Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-13T05:00:29.4700412Z 
2019-10-13T05:00:29.4700438Z 
2019-10-13T05:00:29.4700462Z 
2019-10-13T05:00:29.4702141Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-13T05:00:29.4702405Z 
2019-10-13T05:00:29.4702433Z 
2019-10-13T05:00:29.4702827Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-13T05:00:29.4702889Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-13T05:00:29.4702889Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-13T05:00:29.4702943Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-13T05:00:29.4703015Z Build completed unsuccessfully in 1:09:37
2019-10-13T05:00:29.4703067Z == clock drift check ==
2019-10-13T05:00:29.4703112Z   local time: Sun Oct 13 05:00:29 UTC 2019
2019-10-13T05:00:29.5007906Z   network time: Sun, 13 Oct 2019 05:00:29 GMT
2019-10-13T05:00:29.5014294Z == end clock drift check ==
2019-10-13T05:00:30.4048455Z ##[error]Bash exited with code '1'.
2019-10-13T05:00:30.4107595Z ##[section]Starting: Checkout
2019-10-13T05:00:30.4109706Z ==============================================================================
2019-10-13T05:00:30.4109768Z Task         : Get sources
2019-10-13T05:00:30.4109821Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
