plain
2019-10-19T08:51:48.5346354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T08:51:48.5541226Z ##[command]git config gc.auto 0
2019-10-19T08:51:48.5627694Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T08:51:48.5704378Z ##[command]git config --get-all http.proxy
2019-10-19T08:51:48.5842305Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-19T09:56:30.5280306Z .................................................................................................... 1600/9198
2019-10-19T09:56:35.8675035Z .................................................................................................... 1700/9198
2019-10-19T09:56:49.2017302Z .............................i...............i...................................................... 1800/9198
2019-10-19T09:56:56.8178223Z .................................................................................................... 1900/9198
2019-10-19T09:57:11.6226763Z ...................iiiii............................................................................ 2000/9198
2019-10-19T09:57:22.3568154Z .................................................................................................... 2200/9198
2019-10-19T09:57:24.9829754Z .................................................................................................... 2300/9198
2019-10-19T09:57:30.5723223Z .................................................................................................... 2400/9198
2019-10-19T09:57:53.3399013Z .................................................................................................... 2500/9198
---
2019-10-19T10:00:52.9265530Z ......................i...............i............................................................. 4800/9198
2019-10-19T10:01:04.8590240Z .................................................................................................... 4900/9198
2019-10-19T10:01:11.2012116Z .................................................................................................... 5000/9198
2019-10-19T10:01:21.8729000Z .................................................................................................... 5100/9198
2019-10-19T10:01:28.2646856Z ......................ii.ii......................................................................... 5200/9198
2019-10-19T10:01:38.9109097Z .................................................................................................... 5400/9198
2019-10-19T10:01:49.3269135Z ........................................................................................i........... 5500/9198
2019-10-19T10:01:57.6228531Z .................................................................................................... 5600/9198
2019-10-19T10:02:02.8457585Z .................................................................................................... 5700/9198
2019-10-19T10:02:02.8457585Z .................................................................................................... 5700/9198
2019-10-19T10:02:13.8679027Z .....................................................................................ii...i..ii..... 5800/9198
2019-10-19T10:02:40.7321706Z .................................................................................................... 6000/9198
2019-10-19T10:02:50.4494287Z .................................................................................................... 6100/9198
2019-10-19T10:02:56.6451244Z .................................................................................................... 6200/9198
2019-10-19T10:02:56.6451244Z .................................................................................................... 6200/9198
2019-10-19T10:03:10.8947768Z .......i..ii........................................................................................ 6300/9198
2019-10-19T10:03:31.1613771Z ...................................................................i................................ 6500/9198
2019-10-19T10:03:33.4335711Z .................................................................................................... 6600/9198
2019-10-19T10:03:35.9874158Z ..........................................i......................................................... 6700/9198
2019-10-19T10:03:39.9398407Z .................................................................................................... 6800/9198
---
2019-10-19T10:08:18.3790084Z  finished in 5.489
2019-10-19T10:08:18.3978357Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:08:18.5672971Z 
2019-10-19T10:08:18.5673264Z running 153 tests
2019-10-19T10:08:21.9121933Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-19T10:08:23.9900577Z .i.iiii..............i.........iii.i.........ii......
2019-10-19T10:08:23.9901599Z 
2019-10-19T10:08:23.9907582Z  finished in 5.593
2019-10-19T10:08:24.0107316Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:08:24.5445138Z 
---
2019-10-19T10:08:26.3278011Z  finished in 2.317
2019-10-19T10:08:26.3486966Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:08:26.5208353Z 
2019-10-19T10:08:26.5208691Z running 9 tests
2019-10-19T10:08:26.5209633Z iiiiiiiii
2019-10-19T10:08:26.5210037Z 
2019-10-19T10:08:26.5210103Z  finished in 0.163
2019-10-19T10:08:26.5323374Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:08:27.0391716Z 
---
2019-10-19T10:08:45.1542653Z  finished in 18.622
2019-10-19T10:08:45.1772673Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:08:45.3514225Z 
2019-10-19T10:08:45.3515862Z running 123 tests
2019-10-19T10:09:10.1537931Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-19T10:09:14.9404760Z i.i.i......iii.i.....ii
2019-10-19T10:09:14.9406181Z 
2019-10-19T10:09:14.9406383Z  finished in 29.763
2019-10-19T10:09:14.9416143Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T10:09:14.9416837Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-19T10:09:59.0538659Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-19T10:09:59.0538696Z 
2019-10-19T10:09:59.0538959Z error: test compilation failed although it shouldn't!
2019-10-19T10:09:59.0539011Z status: exit code: 1
2019-10-19T10:09:59.0539715Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-19T10:09:59.0540048Z ------------------------------------------
2019-10-19T10:09:59.0540100Z 
2019-10-19T10:09:59.0540517Z ------------------------------------------
2019-10-19T10:09:59.0540729Z stderr:
---
2019-10-19T10:09:59.0545573Z 
2019-10-19T10:09:59.0545616Z error[E0412]: cannot find type `Parser` in this scope
2019-10-19T10:09:59.0545889Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:27:71
2019-10-19T10:09:59.0545938Z    |
2019-10-19T10:09:59.0546208Z LL | pub fn string_to_parser<'a>(ps: &'a ParseSess, source_str: String) -> Parser<'a> {
2019-10-19T10:09:59.0546331Z 
2019-10-19T10:09:59.0546375Z error[E0412]: cannot find type `Parser` in this scope
2019-10-19T10:09:59.0546726Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:32:20
2019-10-19T10:09:59.0546804Z    |
2019-10-19T10:09:59.0546804Z    |
2019-10-19T10:09:59.0547067Z LL |     F: FnOnce(&mut Parser<'a>) -> PResult<'a, T>,
2019-10-19T10:09:59.0547170Z 
2019-10-19T10:09:59.0547215Z error[E0282]: type annotations needed
2019-10-19T10:09:59.0547469Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:48:51
2019-10-19T10:09:59.0547539Z    |
2019-10-19T10:09:59.0547539Z    |
2019-10-19T10:09:59.0547586Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-19T10:09:59.0547641Z    |                                                   ^ consider giving this closure parameter a type
2019-10-19T10:09:59.0547912Z    = note: type must be known at this point
2019-10-19T10:09:59.0547941Z 
2019-10-19T10:09:59.0548140Z error[E0282]: type annotations needed
2019-10-19T10:09:59.0548583Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:54:51
2019-10-19T10:09:59.0548583Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:54:51
2019-10-19T10:09:59.0548797Z    |
2019-10-19T10:09:59.0548839Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-19T10:09:59.0548903Z    |                                                   ^ consider giving this closure parameter a type
2019-10-19T10:09:59.0548983Z    = note: type must be known at this point
2019-10-19T10:09:59.0549008Z 
2019-10-19T10:09:59.0549061Z error[E0282]: type annotations needed
2019-10-19T10:09:59.0549286Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:60:51
2019-10-19T10:09:59.0549286Z   --> /checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs:60:51
2019-10-19T10:09:59.0549329Z    |
2019-10-19T10:09:59.0549385Z LL |     with_error_checking_parse(s.to_string(), ps, |p| {
2019-10-19T10:09:59.0549432Z    |                                                   ^ consider giving this closure parameter a type
2019-10-19T10:09:59.0549514Z    = note: type must be known at this point
2019-10-19T10:09:59.0549556Z 
2019-10-19T10:09:59.0549601Z error: aborting due to 11 previous errors
2019-10-19T10:09:59.0549705Z 
---
2019-10-19T10:09:59.0554145Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-19T10:09:59.0554179Z 
2019-10-19T10:09:59.0554412Z error: test compilation failed although it shouldn't!
2019-10-19T10:09:59.0554480Z status: exit code: 1
2019-10-19T10:09:59.0555248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-19T10:09:59.0555913Z ------------------------------------------
2019-10-19T10:09:59.0555956Z 
2019-10-19T10:09:59.0556220Z ------------------------------------------
2019-10-19T10:09:59.0556267Z stderr:
---
2019-10-19T10:09:59.0558169Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-19T10:09:59.0558203Z 
2019-10-19T10:09:59.0558448Z error: test compilation failed although it shouldn't!
2019-10-19T10:09:59.0558500Z status: exit code: 1
2019-10-19T10:09:59.0559876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-19T10:09:59.0560271Z ------------------------------------------
2019-10-19T10:09:59.0560307Z 
2019-10-19T10:09:59.0560559Z ------------------------------------------
2019-10-19T10:09:59.0560606Z stderr:
---
2019-10-19T10:09:59.0563645Z 
2019-10-19T10:09:59.0563672Z 
2019-10-19T10:09:59.0563915Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-19T10:09:59.0563948Z 
2019-10-19T10:09:59.0564250Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-19T10:09:59.0564309Z status: exit code: 1
2019-10-19T10:09:59.0565093Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-19T10:09:59.0565460Z ------------------------------------------
2019-10-19T10:09:59.0565496Z 
2019-10-19T10:09:59.0565724Z ------------------------------------------
2019-10-19T10:09:59.0565771Z stderr:
2019-10-19T10:09:59.0565771Z stderr:
2019-10-19T10:09:59.0566010Z ------------------------------------------
2019-10-19T10:09:59.0566064Z error[E0433]: failed to resolve: could not find `parse` in `syntax`
2019-10-19T10:09:59.0566327Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:18:13
2019-10-19T10:09:59.0566398Z    |
2019-10-19T10:09:59.0566443Z LL | use syntax::parse::token::{self, Token};
2019-10-19T10:09:59.0566491Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-19T10:09:59.0566585Z error[E0432]: unresolved import `syntax::parse`
2019-10-19T10:09:59.0566845Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:18:13
2019-10-19T10:09:59.0566894Z    |
2019-10-19T10:09:59.0566894Z    |
2019-10-19T10:09:59.0567045Z LL | use syntax::parse::token::{self, Token};
2019-10-19T10:09:59.0567113Z    |             ^^^^^ could not find `parse` in `syntax`
2019-10-19T10:09:59.0567190Z error[E0422]: cannot find struct, variant or union type `Token` in this scope
2019-10-19T10:09:59.0567491Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:41:26
2019-10-19T10:09:59.0567540Z    |
2019-10-19T10:09:59.0567540Z    |
2019-10-19T10:09:59.0567589Z LL |         TokenTree::Token(Token { kind: token::Ident(s, _), .. }) => s.to_string(),
2019-10-19T10:09:59.0567726Z help: possible candidates are found in other modules, you can import them into scope
2019-10-19T10:09:59.0567791Z    |
2019-10-19T10:09:59.0567833Z LL | use syntax::token::Token;
2019-10-19T10:09:59.0567875Z    |
2019-10-19T10:09:59.0567875Z    |
2019-10-19T10:09:59.0567919Z LL | use syntax::tokenstream::TokenTree::Token;
2019-10-19T10:09:59.0567980Z    |
2019-10-19T10:09:59.0568005Z 
2019-10-19T10:09:59.0568480Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-10-19T10:09:59.0568835Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:66:1
2019-10-19T10:09:59.0568935Z LL | #[plugin_registrar]
2019-10-19T10:09:59.0569008Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-10-19T10:09:59.0569055Z    |
2019-10-19T10:09:59.0569102Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-19T10:09:59.0571978Z test result: FAILED. 65 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-19T10:09:59.0572035Z 
2019-10-19T10:09:59.0575246Z 
2019-10-19T10:09:59.0575317Z 
2019-10-19T10:09:59.0577140Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T10:09:59.0577431Z 
2019-10-19T10:09:59.0577461Z 
2019-10-19T10:09:59.0582281Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T10:09:59.0582355Z Build completed unsuccessfully in 1:11:13
2019-10-19T10:09:59.0582355Z Build completed unsuccessfully in 1:11:13
2019-10-19T10:09:59.0622684Z == clock drift check ==
2019-10-19T10:09:59.0641601Z   local time: Sat Oct 19 10:09:59 UTC 2019
2019-10-19T10:09:59.3425078Z   network time: Sat, 19 Oct 2019 10:09:59 GMT
2019-10-19T10:09:59.3430286Z == end clock drift check ==
2019-10-19T10:10:00.8842370Z 
2019-10-19T10:10:00.8967487Z ##[error]Bash exited with code '1'.
2019-10-19T10:10:00.9006139Z ##[section]Starting: Checkout
2019-10-19T10:10:00.9008960Z ==============================================================================
2019-10-19T10:10:00.9009032Z Task         : Get sources
2019-10-19T10:10:00.9009265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
