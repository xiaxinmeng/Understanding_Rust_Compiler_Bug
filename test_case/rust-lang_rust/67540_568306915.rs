plain
2019-12-22T21:51:18.6614234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T21:51:18.6631147Z ##[command]git config gc.auto 0
2019-12-22T21:51:18.6635824Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T21:51:18.6640561Z ##[command]git config --get-all http.proxy
2019-12-22T21:51:18.6645658Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67540/merge:refs/remotes/pull/67540/merge
---
2019-12-22T21:57:03.5353565Z    Compiling serde_json v1.0.40
2019-12-22T21:57:05.1628810Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-22T21:57:14.6837053Z     Finished release [optimized] target(s) in 1m 16s
2019-12-22T21:57:14.6941942Z tidy check
2019-12-22T21:57:14.7959637Z tidy error: /checkout/src/tools/build-manifest/src/main.rs:637: line longer than 100 chars
2019-12-22T21:57:14.8208245Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:62: line longer than 100 chars
2019-12-22T21:57:14.8208742Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:257: line longer than 100 chars
2019-12-22T21:57:14.8208938Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:286: line longer than 100 chars
2019-12-22T21:57:14.8214489Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1023: line longer than 100 chars
2019-12-22T21:57:14.8214770Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1098: line longer than 100 chars
2019-12-22T21:57:14.8214941Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1099: line longer than 100 chars
2019-12-22T21:57:14.8256285Z tidy error: /checkout/src/librustc_resolve/lib.rs: too many lines (3089) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-12-22T21:57:14.9033227Z tidy error: /checkout/src/libcore/iter/adapters/mod.rs:1138: line longer than 100 chars
2019-12-22T21:57:14.9060104Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs:46: line longer than 100 chars
2019-12-22T21:57:14.9060180Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs:59: line longer than 100 chars
2019-12-22T21:57:14.9060257Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs:65: line longer than 100 chars
2019-12-22T21:57:14.9064755Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs:83: line longer than 100 chars
2019-12-22T21:57:14.9076998Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: too many lines (3106) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-12-22T21:57:14.9307283Z tidy error: /checkout/src/librustc_passes/ast_validation.rs:375: line longer than 100 chars
2019-12-22T21:57:15.3196823Z tidy error: /checkout/src/libsyntax_ext/test.rs:165: line longer than 100 chars
2019-12-22T21:57:15.3513144Z tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:1458: line longer than 100 chars
2019-12-22T21:57:15.3545293Z tidy error: /checkout/src/librustc_codegen_llvm/back/write.rs:620: line longer than 100 chars
2019-12-22T21:57:15.3581484Z tidy error: /checkout/src/librustc_codegen_llvm/consts.rs:281: line longer than 100 chars
2019-12-22T21:57:15.3683287Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:184: line longer than 100 chars
2019-12-22T21:57:15.3725073Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:338: line longer than 100 chars
2019-12-22T21:57:15.3863668Z tidy error: /checkout/src/librustc/hir/lowering.rs:1478: line longer than 100 chars
2019-12-22T21:57:15.3896540Z tidy error: /checkout/src/librustc/traits/mod.rs:524: line longer than 100 chars
2019-12-22T21:57:15.4120588Z tidy error: /checkout/src/librustc/middle/resolve_lifetime.rs: ignoring file length unnecessarily
2019-12-22T21:57:15.4242631Z tidy error: /checkout/src/librustc/ty/wf.rs:208: line longer than 100 chars
2019-12-22T21:57:15.4242739Z tidy error: /checkout/src/librustc/ty/wf.rs:250: line longer than 100 chars
2019-12-22T21:57:15.4242829Z tidy error: /checkout/src/librustc/ty/wf.rs:263: line longer than 100 chars
2019-12-22T21:57:15.4242888Z tidy error: /checkout/src/librustc/ty/wf.rs:265: line longer than 100 chars
2019-12-22T21:57:15.4242947Z tidy error: /checkout/src/librustc/ty/wf.rs:271: line longer than 100 chars
2019-12-22T21:57:15.4243022Z tidy error: /checkout/src/librustc/ty/wf.rs:278: line longer than 100 chars
2019-12-22T21:57:15.4338485Z tidy error: /checkout/src/librustc/ty/context.rs: ignoring file length unnecessarily
2019-12-22T21:57:15.4376501Z tidy error: /checkout/src/librustc/ty/constness.rs:101: line longer than 100 chars
2019-12-22T21:57:15.4376624Z tidy error: /checkout/src/librustc/ty/constness.rs:114: line longer than 100 chars
2019-12-22T21:57:15.4382035Z tidy error: /checkout/src/librustc/lint/mod.rs:538: line longer than 100 chars
2019-12-22T21:57:15.4610669Z tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:76: line longer than 100 chars
2019-12-22T21:57:15.4610771Z tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:93: line longer than 100 chars
2019-12-22T21:57:15.4610878Z tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:97: line longer than 100 chars
2019-12-22T21:57:15.4698016Z tidy error: /checkout/src/libsyntax_expand/parse/tests.rs:68: line longer than 100 chars
2019-12-22T21:57:15.4698106Z tidy error: /checkout/src/libsyntax_expand/parse/tests.rs:73: line longer than 100 chars
2019-12-22T21:57:15.4698197Z tidy error: /checkout/src/libsyntax_expand/parse/tests.rs:78: line longer than 100 chars
2019-12-22T21:57:15.4698257Z tidy error: /checkout/src/libsyntax_expand/parse/tests.rs:84: line longer than 100 chars
2019-12-22T21:57:15.4698355Z tidy error: /checkout/src/libsyntax_expand/parse/lexer/tests.rs:133: line longer than 100 chars
2019-12-22T21:57:15.4698428Z tidy error: /checkout/src/libsyntax_expand/parse/lexer/tests.rs:142: line longer than 100 chars
2019-12-22T21:57:15.4830799Z tidy error: /checkout/src/librustc_mir/borrow_check/type_check/mod.rs:1923: line longer than 100 chars
2019-12-22T21:57:15.4830922Z tidy error: /checkout/src/librustc_mir/borrow_check/type_check/mod.rs:1928: line longer than 100 chars
2019-12-22T21:57:15.4830989Z tidy error: /checkout/src/librustc_mir/borrow_check/type_check/mod.rs:2201: line longer than 100 chars
2019-12-22T21:57:15.4913672Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/mutability_errors.rs:193: line longer than 100 chars
2019-12-22T21:57:15.4913798Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/mutability_errors.rs:230: line longer than 100 chars
2019-12-22T21:57:15.4977126Z tidy error: /checkout/src/librustc_mir/monomorphize/collector.rs:886: line longer than 100 chars
2019-12-22T21:57:15.5044475Z tidy error: /checkout/src/librustc_mir/interpret/place.rs:1090: line longer than 100 chars
2019-12-22T21:57:15.5142646Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:413: line longer than 100 chars
2019-12-22T21:57:15.5152631Z tidy error: /checkout/src/librustc_mir/build/mod.rs:43: line longer than 100 chars
2019-12-22T21:57:15.5183112Z tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:333: line longer than 100 chars
2019-12-22T21:57:15.5311919Z tidy error: /checkout/src/librustc_mir/const_eval.rs:716: line longer than 100 chars
2019-12-22T21:57:15.5312007Z tidy error: /checkout/src/librustc_mir/const_eval.rs:719: line longer than 100 chars
2019-12-22T21:57:15.5312094Z tidy error: /checkout/src/librustc_mir/const_eval.rs:730: line longer than 100 chars
2019-12-22T21:57:15.5312155Z tidy error: /checkout/src/librustc_mir/const_eval.rs:731: line longer than 100 chars
2019-12-22T21:57:15.5312231Z tidy error: /checkout/src/librustc_mir/const_eval.rs:748: line longer than 100 chars
2019-12-22T21:57:15.5489285Z tidy error: /checkout/src/librustdoc/html/markdown/tests.rs: ignoring line length unnecessarily
2019-12-22T21:57:15.5614169Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:906: line longer than 100 chars
2019-12-22T21:57:15.5614289Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:907: line longer than 100 chars
2019-12-22T21:57:15.5614353Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:908: line longer than 100 chars
2019-12-22T21:57:15.5641747Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:5257: line longer than 100 chars
2019-12-22T21:57:15.5656578Z tidy error: /checkout/src/librustc_typeck/check/_match.rs:184: line longer than 100 chars
2019-12-22T21:57:15.5685079Z tidy error: /checkout/src/librustc_typeck/check/wfcheck.rs:771: line longer than 100 chars
2019-12-22T21:57:15.5695861Z tidy error: /checkout/src/librustc_typeck/check/coercion.rs:927: line longer than 100 chars
2019-12-22T21:57:15.5742125Z tidy error: /checkout/src/librustc_typeck/check/method/confirm.rs:549: line longer than 100 chars
2019-12-22T21:57:15.5845356Z tidy error: /checkout/src/librustc_typeck/astconv.rs:2601: line longer than 100 chars
2019-12-22T21:57:15.5845480Z tidy error: /checkout/src/librustc_typeck/astconv.rs:2602: line longer than 100 chars
2019-12-22T21:57:15.5845542Z tidy error: /checkout/src/librustc_typeck/astconv.rs:2603: line longer than 100 chars
2019-12-22T21:57:15.5845606Z tidy error: /checkout/src/librustc_codegen_utils/symbol_names/v0.rs:199: line longer than 100 chars
2019-12-22T21:57:15.6098453Z tidy error: /checkout/src/libstd/sys/unix/os.rs:73: line longer than 100 chars
2019-12-22T21:57:15.6248374Z tidy error: /checkout/src/libstd/sys/sgx/abi/mem.rs:36: line longer than 100 chars
2019-12-22T21:57:15.6248493Z tidy error: /checkout/src/libstd/sys/sgx/abi/mem.rs:46: line longer than 100 chars
2019-12-22T21:57:15.6903344Z tidy error: /checkout/src/librustc_errors/emitter.rs:128: line longer than 100 chars
2019-12-22T21:57:15.6903452Z tidy error: /checkout/src/librustc_errors/emitter.rs:586: line longer than 100 chars
2019-12-22T21:57:16.0042697Z thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/eb3f7c2d3aec576f47eba854cfbd3c1187b8a2a0/src/libcore/macros/mod.rs:15:40
2019-12-22T21:57:16.0042946Z 
2019-12-22T21:57:16.0042992Z 
2019-12-22T21:57:16.0042992Z 
2019-12-22T21:57:16.0043442Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-22T21:57:16.0043555Z 
2019-12-22T21:57:16.0043579Z 
2019-12-22T21:57:16.0052285Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-22T21:57:16.0052605Z Build completed unsuccessfully in 0:01:25
2019-12-22T21:57:16.0052605Z Build completed unsuccessfully in 0:01:25
2019-12-22T21:57:16.0102147Z == clock drift check ==
2019-12-22T21:57:16.0112474Z   local time: Sun Dec 22 21:57:16 UTC 2019
2019-12-22T21:57:16.2780366Z   network time: Sun, 22 Dec 2019 21:57:16 GMT
2019-12-22T21:57:16.2783567Z == end clock drift check ==
2019-12-22T21:57:17.7800292Z 
2019-12-22T21:57:17.7957623Z ##[error]Bash exited with code '1'.
2019-12-22T21:57:17.7994933Z ##[section]Starting: Checkout
2019-12-22T21:57:17.7997156Z ==============================================================================
2019-12-22T21:57:17.7997222Z Task         : Get sources
2019-12-22T21:57:17.7997307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
