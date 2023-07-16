plain
2019-10-13T16:57:41.3599599Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T16:57:41.3698426Z ##[command]git config gc.auto 0
2019-10-13T16:57:41.3780681Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T16:57:42.0550861Z ##[command]git config --get-all http.proxy
2019-10-13T16:57:42.0559672Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-13T17:07:17.4428218Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-13T17:07:26.6006826Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-13T17:07:28.1917739Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-13T17:07:46.2940412Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-13T17:08:35.0254911Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-13T17:09:22.5681342Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-13T17:14:33.0136415Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-13T17:15:21.7756531Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-13T17:16:36.9824420Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-13T17:17:19.7520162Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
---
2019-10-13T17:31:34.1107764Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-13T17:31:45.3886170Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-13T17:31:47.0121433Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-13T17:32:09.3061642Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-13T17:33:05.7574505Z    Compiling syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-13T17:34:02.1752530Z    Compiling syntax_macros v0.0.0 (/checkout/src/libsyntax_macros)
2019-10-13T17:40:21.0021833Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-13T17:45:39.8738626Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-13T17:47:09.6773966Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-13T17:48:00.1794936Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
---
2019-10-13T18:00:19.4514183Z .................................................................................................... 1600/9173
2019-10-13T18:00:25.2890175Z .................................................................................................... 1700/9173
2019-10-13T18:00:39.1554475Z .......................i...............i............................................................ 1800/9173
2019-10-13T18:00:46.6737094Z .................................................................................................... 1900/9173
2019-10-13T18:01:01.8116705Z ..............iiiii................................................................................. 2000/9173
2019-10-13T18:01:12.7853332Z .................................................................................................... 2200/9173
2019-10-13T18:01:15.4455661Z .................................................................................................... 2300/9173
2019-10-13T18:01:21.2933413Z .................................................................................................... 2400/9173
2019-10-13T18:01:44.2771407Z .................................................................................................... 2500/9173
---
2019-10-13T18:04:50.7804456Z .....................i...............i.............................................................. 4800/9173
2019-10-13T18:05:03.1290079Z .................................................................................................... 4900/9173
2019-10-13T18:05:10.2050171Z .................................................................................................... 5000/9173
2019-10-13T18:05:21.0245360Z .................................................................................................... 5100/9173
2019-10-13T18:05:27.6963705Z .....................ii.ii.......................................................................... 5200/9173
2019-10-13T18:05:38.8896485Z .................................................................................................... 5400/9173
2019-10-13T18:05:49.3892609Z .......................................................................................i............ 5500/9173
2019-10-13T18:05:57.9934013Z .................................................................................................... 5600/9173
2019-10-13T18:06:03.4334333Z .................................................................................................... 5700/9173
2019-10-13T18:06:03.4334333Z .................................................................................................... 5700/9173
2019-10-13T18:06:14.8365635Z ....................................................................................ii...i..ii...... 5800/9173
2019-10-13T18:06:41.5594571Z .................................................................................................... 6000/9173
2019-10-13T18:06:51.9641263Z .................................................................................................... 6100/9173
2019-10-13T18:06:51.9641263Z .................................................................................................... 6100/9173
2019-10-13T18:07:00.5841724Z ...........................................................................................i..ii.... 6200/9173
2019-10-13T18:07:30.4901000Z .................................................................................................... 6400/9173
2019-10-13T18:07:35.1148384Z ...................................................i................................................ 6500/9173
2019-10-13T18:07:37.4286137Z .................................................................................................... 6600/9173
2019-10-13T18:07:39.9584562Z ........................i........................................................................... 6700/9173
---
2019-10-13T18:12:31.6925858Z  finished in 5.872
2019-10-13T18:12:31.7119051Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:12:31.9356162Z 
2019-10-13T18:12:31.9356921Z running 153 tests
2019-10-13T18:12:35.2948793Z i....iii......iii..iiii....i.............................i..i..................i....i............ii. 100/153
2019-10-13T18:12:37.3981170Z i.i..iiii..............i.........iii.i.......ii......
2019-10-13T18:12:37.3981801Z 
2019-10-13T18:12:37.3981884Z  finished in 5.685
2019-10-13T18:12:37.4182285Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:12:37.5953107Z 
---
2019-10-13T18:12:39.7761026Z  finished in 2.358
2019-10-13T18:12:39.7949513Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:12:39.9613319Z 
2019-10-13T18:12:39.9614216Z running 9 tests
2019-10-13T18:12:39.9615037Z iiiiiiiii
2019-10-13T18:12:39.9615787Z 
2019-10-13T18:12:39.9621107Z  finished in 0.167
2019-10-13T18:12:39.9806487Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:12:40.1809921Z 
---
2019-10-13T18:12:59.2829019Z  finished in 19.302
2019-10-13T18:12:59.3059034Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:12:59.5199520Z 
2019-10-13T18:12:59.5202152Z running 123 tests
2019-10-13T18:13:25.1083685Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-13T18:13:30.0745956Z i.i.i......iii.i.....ii
2019-10-13T18:13:30.0747467Z 
2019-10-13T18:13:30.0750714Z  finished in 30.769
2019-10-13T18:13:30.0760104Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:13:30.0761153Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-13T18:13:30.0761153Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-13T18:13:30.0981423Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-13T18:13:30.8319558Z 
2019-10-13T18:13:30.8331621Z running 69 tests
2019-10-13T18:13:56.2919853Z F............F....................................F...FFFFFFF..F.....
2019-10-13T18:13:56.2938098Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-13T18:13:56.2938557Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-13T18:13:56.2944287Z 
2019-10-13T18:13:56.2944880Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-13T18:13:56.2944880Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-13T18:13:56.2945505Z 
2019-10-13T18:13:56.2945795Z error: test compilation failed although it shouldn't!
2019-10-13T18:13:56.2945986Z status: exit code: 1
2019-10-13T18:13:56.2947101Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-13T18:13:56.2948134Z ------------------------------------------
2019-10-13T18:13:56.2948442Z 
2019-10-13T18:13:56.2948916Z ------------------------------------------
2019-10-13T18:13:56.2949112Z stderr:
---
2019-10-13T18:13:56.2951282Z 
2019-10-13T18:13:56.2951437Z error[E0432]: unresolved import `syntax::parse`
2019-10-13T18:13:56.2951854Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:15:13
2019-10-13T18:13:56.2952041Z    |
2019-10-13T18:13:56.2952187Z LL | use syntax::parse::{ParseSess, PResult};
2019-10-13T18:13:56.2952360Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T18:13:56.2952634Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-13T18:13:56.2953332Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:17:13
2019-10-13T18:13:56.2953522Z    |
2019-10-13T18:13:56.2953824Z LL | use syntax::parse::parser::Parser;
---
2019-10-13T18:13:56.2957724Z 
2019-10-13T18:13:56.2957856Z error[E0412]: cannot find type `Parser` in this scope
2019-10-13T18:13:56.2958209Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:26:71
2019-10-13T18:13:56.2958367Z    |
2019-10-13T18:13:56.2958718Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-13T18:13:56.2959027Z 
2019-10-13T18:13:56.2959170Z error[E0412]: cannot find type `Parser` in this scope
2019-10-13T18:13:56.2959517Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:31:20
2019-10-13T18:13:56.2959673Z    |
2019-10-13T18:13:56.2959673Z    |
2019-10-13T18:13:56.2960001Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-13T18:13:56.2960266Z 
2019-10-13T18:13:56.2960798Z error: aborting due to 8 previous errors
2019-10-13T18:13:56.2960942Z 
2019-10-13T18:13:56.2961556Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-13T18:13:56.2961556Z Some errors have detailed explanations: E0412, E0432, E0433.
2019-10-13T18:13:56.2962079Z For more information about an error, try `rustc --explain E0412`.
2019-10-13T18:13:56.2962257Z 
2019-10-13T18:13:56.2962609Z ------------------------------------------
2019-10-13T18:13:56.2968599Z 
2019-10-13T18:13:56.2968703Z 
2019-10-13T18:13:56.2969167Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-13T18:13:56.2969199Z 
2019-10-13T18:13:56.2969474Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T18:13:56.2969546Z status: exit code: 1
2019-10-13T18:13:56.2970184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-13T18:13:56.2970798Z ------------------------------------------
2019-10-13T18:13:56.2970856Z 
2019-10-13T18:13:56.2971084Z ------------------------------------------
2019-10-13T18:13:56.2971132Z stderr:
2019-10-13T18:13:56.2971132Z stderr:
2019-10-13T18:13:56.2971343Z ------------------------------------------
2019-10-13T18:13:56.2971636Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T18:13:56.2971944Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T18:13:56.2972000Z    |
2019-10-13T18:13:56.2972066Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T18:13:56.2972480Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.2972600Z 
2019-10-13T18:13:56.2973192Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.2973481Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T18:13:56.2973846Z LL | #[plugin_registrar]
2019-10-13T18:13:56.2973896Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.2974149Z    |
2019-10-13T18:13:56.2974188Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.2978567Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-13T18:13:56.2978616Z 
2019-10-13T18:13:56.2978844Z error: test compilation failed although it shouldn't!
2019-10-13T18:13:56.2978892Z status: exit code: 1
2019-10-13T18:13:56.2979820Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-13T18:13:56.2981352Z ------------------------------------------
2019-10-13T18:13:56.2981398Z 
2019-10-13T18:13:56.2981871Z ------------------------------------------
2019-10-13T18:13:56.2982067Z stderr:
2019-10-13T18:13:56.2982067Z stderr:
2019-10-13T18:13:56.2982358Z ------------------------------------------
2019-10-13T18:13:56.2982681Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-13T18:13:56.3011580Z    |
2019-10-13T18:13:56.3011580Z    |
2019-10-13T18:13:56.3011689Z LL | use syntax::parse::{self, ParseSess};
2019-10-13T18:13:56.3011772Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-13T18:13:56.3011870Z    |             could not find `parse` in `syntax`
2019-10-13T18:13:56.3012046Z 
2019-10-13T18:13:56.3012119Z error: aborting due to previous error
2019-10-13T18:13:56.3012178Z 
2019-10-13T18:13:56.3012178Z 
2019-10-13T18:13:56.3012610Z For more information about this error, try `rustc --explain E0432`.
2019-10-13T18:13:56.3012800Z 
2019-10-13T18:13:56.3013148Z ------------------------------------------
2019-10-13T18:13:56.3013309Z 
2019-10-13T18:13:56.3013388Z 
2019-10-13T18:13:56.3013678Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-13T18:13:56.3013860Z 
2019-10-13T18:13:56.3014192Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T18:13:56.3014409Z status: exit code: 1
2019-10-13T18:13:56.3015888Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-13T18:13:56.3016661Z ------------------------------------------
2019-10-13T18:13:56.3016862Z 
2019-10-13T18:13:56.3017127Z ------------------------------------------
2019-10-13T18:13:56.3017292Z stderr:
2019-10-13T18:13:56.3017292Z stderr:
2019-10-13T18:13:56.3017569Z ------------------------------------------
2019-10-13T18:13:56.3017625Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T18:13:56.3018013Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T18:13:56.3018207Z    |
2019-10-13T18:13:56.3018311Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T18:13:56.3018362Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T18:13:56.3018549Z    |                         |
2019-10-13T18:13:56.3018618Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3018648Z 
2019-10-13T18:13:56.3018741Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T18:13:56.3019052Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T18:13:56.3019246Z    |
2019-10-13T18:13:56.3019317Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T18:13:56.3019368Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T18:13:56.3019653Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T18:13:56.3019729Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3019729Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3019816Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T18:13:56.3019844Z 
2019-10-13T18:13:56.3020271Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3021179Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T18:13:56.3021623Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3021817Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3021889Z    |
2019-10-13T18:13:56.3021981Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3023093Z 
2019-10-13T18:13:56.3023188Z 
2019-10-13T18:13:56.3023466Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-13T18:13:56.3023630Z 
2019-10-13T18:13:56.3024028Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T18:13:56.3024218Z status: exit code: 1
2019-10-13T18:13:56.3025666Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-13T18:13:56.3026298Z ------------------------------------------
2019-10-13T18:13:56.3026329Z 
2019-10-13T18:13:56.3026516Z ------------------------------------------
2019-10-13T18:13:56.3026572Z stderr:
2019-10-13T18:13:56.3026572Z stderr:
2019-10-13T18:13:56.3026919Z ------------------------------------------
2019-10-13T18:13:56.3027086Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T18:13:56.3027391Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T18:13:56.3027556Z    |
2019-10-13T18:13:56.3027669Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T18:13:56.3027741Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T18:13:56.3027803Z    |                         |
2019-10-13T18:13:56.3027880Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3027907Z 
2019-10-13T18:13:56.3027988Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T18:13:56.3028297Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T18:13:56.3028343Z    |
2019-10-13T18:13:56.3028383Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T18:13:56.3028584Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T18:13:56.3028735Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T18:13:56.3028781Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3028781Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3029129Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T18:13:56.3029161Z 
2019-10-13T18:13:56.3029588Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3029999Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T18:13:56.3030245Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3030286Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3030709Z    |
2019-10-13T18:13:56.3030882Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3032371Z 
2019-10-13T18:13:56.3032474Z 
2019-10-13T18:13:56.3032764Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-13T18:13:56.3032930Z 
2019-10-13T18:13:56.3033299Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-13T18:13:56.3033389Z status: exit code: 1
2019-10-13T18:13:56.3034481Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-13T18:13:56.3035023Z ------------------------------------------
2019-10-13T18:13:56.3035332Z 
2019-10-13T18:13:56.3035769Z ------------------------------------------
2019-10-13T18:13:56.3035924Z stderr:
2019-10-13T18:13:56.3035924Z stderr:
2019-10-13T18:13:56.3036355Z ------------------------------------------
2019-10-13T18:13:56.3036678Z error[E0432]: unresolved imports `syntax::ext::base::SyntaxExtension`, `syntax::ext::base::SyntaxExtensionKind`
2019-10-13T18:13:56.3036991Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:13:25
2019-10-13T18:13:56.3037062Z    |
2019-10-13T18:13:56.3037102Z LL | use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind};
2019-10-13T18:13:56.3037279Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `SyntaxExtensionKind` in `ext::base`
2019-10-13T18:13:56.3037374Z    |                         |
2019-10-13T18:13:56.3037443Z    |                         no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3037470Z 
2019-10-13T18:13:56.3037552Z error[E0432]: unresolved imports `syntax::ext::base::TTMacroExpander`, `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::MacEager`
2019-10-13T18:13:56.3037867Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:14:25
2019-10-13T18:13:56.3037938Z    |
2019-10-13T18:13:56.3037980Z LL | use syntax::ext::base::{TTMacroExpander, ExtCtxt, MacResult, MacEager};
2019-10-13T18:13:56.3038028Z    |                         ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T18:13:56.3038134Z    |                         |                |        no `MacResult` in `ext::base`
2019-10-13T18:13:56.3038185Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3038185Z    |                         |                no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3038244Z    |                         no `TTMacroExpander` in `ext::base`
2019-10-13T18:13:56.3038271Z 
2019-10-13T18:13:56.3038587Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3038832Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:36:1
2019-10-13T18:13:56.3039051Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3039175Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3039213Z    |
2019-10-13T18:13:56.3039252Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3040082Z 
2019-10-13T18:13:56.3040209Z 
2019-10-13T18:13:56.3040856Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-13T18:13:56.3040897Z 
2019-10-13T18:13:56.3041424Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T18:13:56.3041640Z status: exit code: 1
2019-10-13T18:13:56.3042484Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-13T18:13:56.3043194Z ------------------------------------------
2019-10-13T18:13:56.3043398Z 
2019-10-13T18:13:56.3043731Z ------------------------------------------
2019-10-13T18:13:56.3043782Z stderr:
2019-10-13T18:13:56.3043782Z stderr:
2019-10-13T18:13:56.3044574Z ------------------------------------------
2019-10-13T18:13:56.3044623Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T18:13:56.3044986Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T18:13:56.3045681Z    |
2019-10-13T18:13:56.3045882Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T18:13:56.3045965Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3046012Z 
2019-10-13T18:13:56.3046558Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3046951Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T18:13:56.3047532Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3047693Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3047828Z    |
2019-10-13T18:13:56.3047974Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3049918Z 
2019-10-13T18:13:56.3050024Z 
2019-10-13T18:13:56.3050823Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-13T18:13:56.3051040Z 
2019-10-13T18:13:56.3051524Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T18:13:56.3051720Z status: exit code: 1
2019-10-13T18:13:56.3053065Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-13T18:13:56.3053768Z ------------------------------------------
2019-10-13T18:13:56.3053934Z 
2019-10-13T18:13:56.3054827Z ------------------------------------------
2019-10-13T18:13:56.3057643Z stderr:
2019-10-13T18:13:56.3057643Z stderr:
2019-10-13T18:13:56.3058142Z ------------------------------------------
2019-10-13T18:13:56.3058475Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T18:13:56.3059030Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T18:13:56.3059247Z    |
2019-10-13T18:13:56.3059571Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T18:13:56.3059674Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3059707Z 
2019-10-13T18:13:56.3060243Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3060968Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T18:13:56.3061096Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3061160Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3061207Z    |
2019-10-13T18:13:56.3061253Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3062094Z 
2019-10-13T18:13:56.3062136Z 
2019-10-13T18:13:56.3062362Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-13T18:13:56.3062395Z 
2019-10-13T18:13:56.3062676Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-13T18:13:56.3062757Z status: exit code: 1
2019-10-13T18:13:56.3063802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-13T18:13:56.3064500Z ------------------------------------------
2019-10-13T18:13:56.3064537Z 
2019-10-13T18:13:56.3066190Z ------------------------------------------
2019-10-13T18:13:56.3066428Z stderr:
2019-10-13T18:13:56.3066428Z stderr:
2019-10-13T18:13:56.3066869Z ------------------------------------------
2019-10-13T18:13:56.3066936Z error[E0432]: unresolved import `syntax::ext::base::SyntaxExtension`
2019-10-13T18:13:56.3067384Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:10:5
2019-10-13T18:13:56.3067597Z    |
2019-10-13T18:13:56.3067654Z LL | use syntax::ext::base::SyntaxExtension;
2019-10-13T18:13:56.3067703Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SyntaxExtension` in `ext::base`
2019-10-13T18:13:56.3067735Z 
2019-10-13T18:13:56.3068271Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3068537Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:14:1
2019-10-13T18:13:56.3068646Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3068690Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3068732Z    |
2019-10-13T18:13:56.3068792Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3074579Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-13T18:13:56.3074620Z 
2019-10-13T18:13:56.3074820Z error: test compilation failed although it shouldn't!
2019-10-13T18:13:56.3074864Z status: exit code: 1
2019-10-13T18:13:56.3076135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-13T18:13:56.3077205Z ------------------------------------------
2019-10-13T18:13:56.3077253Z 
2019-10-13T18:13:56.3077455Z ------------------------------------------
2019-10-13T18:13:56.3077518Z stderr:
2019-10-13T18:13:56.3077518Z stderr:
2019-10-13T18:13:56.3077928Z ------------------------------------------
2019-10-13T18:13:56.3077987Z error[E0432]: unresolved imports `syntax::parse`, `syntax::parse`
2019-10-13T18:13:56.3078295Z    |
2019-10-13T18:13:56.3078295Z    |
2019-10-13T18:13:56.3078332Z LL | use syntax::parse::{self, ParseSess};
2019-10-13T18:13:56.3078390Z    |             ^^^^^   ^^^^ no `parse` in the root
2019-10-13T18:13:56.3078645Z    |             could not find `parse` in `syntax`
2019-10-13T18:13:56.3078674Z 
2019-10-13T18:13:56.3078731Z error: aborting due to previous error
2019-10-13T18:13:56.3078756Z 
2019-10-13T18:13:56.3078756Z 
2019-10-13T18:13:56.3081559Z For more information about this error, try `rustc --explain E0432`.
2019-10-13T18:13:56.3081610Z 
2019-10-13T18:13:56.3082070Z ------------------------------------------
2019-10-13T18:13:56.3082114Z 
2019-10-13T18:13:56.3082140Z 
2019-10-13T18:13:56.3082421Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-13T18:13:56.3082477Z 
2019-10-13T18:13:56.3082983Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-13T18:13:56.3083071Z status: exit code: 1
2019-10-13T18:13:56.3084132Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-13T18:13:56.3084830Z ------------------------------------------
2019-10-13T18:13:56.3084862Z 
2019-10-13T18:13:56.3085048Z ------------------------------------------
2019-10-13T18:13:56.3085103Z stderr:
2019-10-13T18:13:56.3085103Z stderr:
2019-10-13T18:13:56.3085504Z ------------------------------------------
2019-10-13T18:13:56.3085569Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-13T18:13:56.3085815Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-13T18:13:56.3085862Z    |
2019-10-13T18:13:56.3085901Z LL | use syntax::parse::token::{self, Token};
2019-10-13T18:13:56.3086115Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T18:13:56.3086202Z error[E0432]: unresolved import `syntax::parse`
2019-10-13T18:13:56.3086483Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:17:13
2019-10-13T18:13:56.3086558Z    |
2019-10-13T18:13:56.3086558Z    |
2019-10-13T18:13:56.3086596Z LL | use syntax::parse::token::{self, Token};
2019-10-13T18:13:56.3086637Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-13T18:13:56.3086663Z 
2019-10-13T18:13:56.3086728Z error[E0432]: unresolved imports `syntax::ext::base::ExtCtxt`, `syntax::ext::base::MacResult`, `syntax::ext::base::DummyResult`, `syntax::ext::base::MacEager`
2019-10-13T18:13:56.3087087Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:19:25
2019-10-13T18:13:56.3087167Z    |
2019-10-13T18:13:56.3087210Z LL | use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
2019-10-13T18:13:56.3087257Z    |                         ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^ no `MacEager` in `ext::base`
2019-10-13T18:13:56.3087363Z    |                         |        |          no `DummyResult` in `ext::base`
2019-10-13T18:13:56.3087414Z    |                         |        no `MacResult` in `ext::base`
2019-10-13T18:13:56.3087456Z    |                         no `ExtCtxt` in `ext::base`
2019-10-13T18:13:56.3087498Z 
2019-10-13T18:13:56.3087498Z 
2019-10-13T18:13:56.3087537Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-13T18:13:56.3087794Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:40:26
2019-10-13T18:13:56.3087854Z    |
2019-10-13T18:13:56.3087903Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-13T18:13:56.3088010Z help: possible candidates are found in other modules, you can import them into scope
2019-10-13T18:13:56.3088050Z    |
2019-10-13T18:13:56.3088086Z LL | use syntax::token::Token;
2019-10-13T18:13:56.3088138Z    |
2019-10-13T18:13:56.3088138Z    |
2019-10-13T18:13:56.3088385Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-13T18:13:56.3088425Z    |
2019-10-13T18:13:56.3088554Z 
2019-10-13T18:13:56.3088956Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-13T18:13:56.3089187Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:65:1
2019-10-13T18:13:56.3089280Z LL | #[plugin_registrar]
2019-10-13T18:13:56.3089320Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-13T18:13:56.3089357Z    |
2019-10-13T18:13:56.3089413Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-13T18:13:56.3094092Z test result: FAILED. 58 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-13T18:13:56.3094136Z 
2019-10-13T18:13:56.3094179Z 
2019-10-13T18:13:56.3094205Z 
2019-10-13T18:13:56.3096775Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-13T18:13:56.3097267Z 
2019-10-13T18:13:56.3097295Z 
2019-10-13T18:13:56.3097344Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-13T18:13:56.3097408Z Build completed unsuccessfully in 1:09:51
2019-10-13T18:13:56.3097408Z Build completed unsuccessfully in 1:09:51
2019-10-13T18:13:56.3097451Z == clock drift check ==
2019-10-13T18:13:56.3098398Z   local time: Sun Oct 13 18:13:56 UTC 2019
2019-10-13T18:13:56.4073112Z   network time: Sun, 13 Oct 2019 18:13:56 GMT
2019-10-13T18:13:56.4073234Z == end clock drift check ==
2019-10-13T18:13:57.3376371Z ##[error]Bash exited with code '1'.
2019-10-13T18:13:57.3428222Z ##[section]Starting: Checkout
2019-10-13T18:13:57.3431334Z ==============================================================================
2019-10-13T18:13:57.3431675Z Task         : Get sources
2019-10-13T18:13:57.3431750Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
