plain
2019-09-21T00:58:47.4638165Z test [ui] ui/coherence/coherence_local_err_tuple.rs#re ... ok
2019-09-21T00:58:47.5781216Z test [ui] ui/coherence/coherence_local_ref.rs#old ... ok
2019-09-21T00:58:47.6040528Z test [ui] ui/coherence/coherence_local_ref.rs#re ... ok
2019-09-21T00:58:47.6357933Z test [ui] ui/coherence/conflicting-impl-with-err.rs ... ok
2019-09-21T00:58:47.7368223Z test [ui] ui/coherence/impl-foreign[foreign]-for-foreign.rs ... ok
2019-09-21T00:58:47.7842846Z test [ui] ui/coherence/impl-foreign[foreign]-for-local.rs ... ok
2019-09-21T00:58:47.8987410Z test [ui] ui/coherence/impl[t]-foreign[foreign[t],local]-for-foreign.rs ... ok
2019-09-21T00:58:47.9416765Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs ... ok
2019-09-21T00:58:48.0490475Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-t.rs ... ok
2019-09-21T00:58:48.0906364Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t],local]-for-foreign.rs ... ok
2019-09-21T00:58:48.1945259Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs ... ok
2019-09-21T00:58:48.2403786Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-09-21T00:58:48.3417281Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-local.rs ... ok
2019-09-21T00:58:48.3823725Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-t.rs ... ok
2019-09-21T00:58:48.4923275Z test [ui] ui/coherence/impl[t]-foreign[local, fundamental[t]]-for-foreign.rs ... ok
2019-09-21T00:58:48.5385868Z test [ui] ui/coherence/impl[t]-foreign[local]-for-foreign.rs ... ok
2019-09-21T00:58:48.6248770Z test [ui] ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs ... ok
2019-09-21T00:58:48.6890772Z test [ui] ui/coherence/impl[t]-foreign[local]-for-local.rs ... ok
2019-09-21T00:58:48.7578750Z test [ui] ui/coherence/impl[t]-foreign[local]-for-t.rs ... ok
2019-09-21T00:58:48.8343414Z test [ui] ui/coherence/impl[t]-foreign[t]-for-foreign.rs ... ok
2019-09-21T00:58:48.9014596Z test [ui] ui/coherence/impl[t]-foreign[t]-for-fundamental.rs ... ok
2019-09-21T00:58:48.9861616Z test [ui] ui/coherence/impl[t]-foreign[t]-for-local.rs ... ok
2019-09-21T00:58:49.0387885Z test [ui] ui/coherence/impl[t]-foreign[t]-for-t.rs ... ok
2019-09-21T00:58:49.2730209Z test [ui] ui/coherence/re-rebalance-coherence.rs ... ok
2019-09-21T00:58:49.3055668Z test [ui] ui/collections-const-new.rs ... ok
2019-09-21T00:58:49.3510704Z test [ui] ui/command-line-diagnostics.rs ... ok
2019-09-21T00:58:50.7272190Z test [ui] ui/command-exec.rs ... ok
---
2019-09-21T01:04:29.0350038Z test [ui] ui/lint/reasons.rs ... ok
2019-09-21T01:04:29.0947631Z test [ui] ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs ... ok
2019-09-21T01:04:29.1862049Z test [ui] ui/lint/suggestions.rs ... ok
2019-09-21T01:04:29.2633991Z test [ui] ui/lint/test-inner-fn.rs ... ok
2019-09-21T01:04:29.3011031Z test [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs ... FAILED
2019-09-21T01:04:29.3447722Z test [ui] ui/lint/trivial-casts.rs ... ok
2019-09-21T01:04:29.4366606Z test [ui] ui/lint/uninitialized-zeroed.rs ... ok
2019-09-21T01:04:29.4438082Z test [ui] ui/lint/type-overflow.rs ... ok
2019-09-21T01:04:29.6086504Z test [ui] ui/lint/unreachable_pub-pub_crate.rs ... ok
---
2019-09-21T01:11:01.5183456Z test [ui] ui/wrapping-int-combinations.rs ... ok
2019-09-21T01:11:01.5186741Z 
2019-09-21T01:11:01.5186916Z failures:
2019-09-21T01:11:01.5237569Z 
2019-09-21T01:11:01.5238153Z ---- [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs stdout ----
2019-09-21T01:11:01.5238315Z 
2019-09-21T01:11:01.5238315Z 
2019-09-21T01:11:01.5240689Z - TokenStream [Ident { ident: "fn", span: #0 bytes(197..199) }, Ident { ident: "span_preservation", span: #0 bytes(200..217) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(217..219) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "let", span: #0 bytes(227..230) }, Ident { ident: "tst", span: #0 bytes(231..234) }, Punct { ch: '=', spacing: Alone, span: #0 bytes(235..236) }, Literal { lit: Lit { kind: Integer, symbol: 123, suffix: None }, span: Span { lo: BytePos(237), hi: BytePos(240), ctxt: #0 } }, Punct { ch: ';', spacing: Joint, span: #0 bytes(240..241) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(241..242) }, Ident { ident: "match", span: #0 bytes(288..293) }, Ident { ident: "tst", span: #0 bytes(294..297) }, Group { delimiter: Brace, stream: TokenStream [Literal { lit: Lit { kind: Integer, symbol: 123, suffix: None }, span: Span { lo: BytePos(482), hi: BytePos(485), ctxt: #0 } }, Punct { ch: '=', spacing: Joint, span: #0 bytes(486..488) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(486..488) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(489..491) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(491..492) }, Ident { ident: "_", span: #0 bytes(501..502) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(503..505) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(503..505) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(506..508) }], span: #0 bytes(298..514) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(514..515) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(515..516) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(516..517) }], span: #0 bytes(221..561) }]
2019-09-21T01:11:01.5242169Z -   --> $DIR/redundant-semi-proc-macro.rs:9:19
2019-09-21T01:11:01.5242169Z -   --> $DIR/redundant-semi-proc-macro.rs:9:19
2019-09-21T01:11:01.5242447Z + error[E0463]: can't find crate for `redundant_semi_proc_macro`
2019-09-21T01:11:01.5243403Z +   --> $DIR/redundant-semi-proc-macro.rs:4:1
2019-09-21T01:11:01.5243491Z 4    |
2019-09-21T01:11:01.5243740Z - LL |     let tst = 123;;
2019-09-21T01:11:01.5243999Z -    |                   ^ help: remove this semicolon
2019-09-21T01:11:01.5244481Z - note: lint level defined here
2019-09-21T01:11:01.5244734Z -   --> $DIR/redundant-semi-proc-macro.rs:3:9
2019-09-21T01:11:01.5244960Z -    |
2019-09-21T01:11:01.5244960Z -    |
2019-09-21T01:11:01.5245188Z - LL | #![deny(redundant_semicolon)]
2019-09-21T01:11:01.5245443Z -    |         ^^^^^^^^^^^^^^^^^^^
2019-09-21T01:11:01.5245665Z + LL | extern crate redundant_semi_proc_macro;
2019-09-21T01:11:01.5246087Z 13 
2019-09-21T01:11:01.5246541Z - error: unnecessary trailing semicolons
2019-09-21T01:11:01.5246808Z -   --> $DIR/redundant-semi-proc-macro.rs:16:7
2019-09-21T01:11:01.5247008Z -    |
---
2019-09-21T01:11:01.5248376Z 22 
2019-09-21T01:11:01.5248433Z 
2019-09-21T01:11:01.5248467Z 
2019-09-21T01:11:01.5248551Z The actual stderr differed from the expected stderr.
2019-09-21T01:11:01.5248937Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/redundant-semi-proc-macro.stderr
2019-09-21T01:11:01.5249255Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T01:11:01.5249593Z To only update this specific test, also pass `--test-args lint/redundant-semicolon/redundant-semi-proc-macro.rs`
2019-09-21T01:11:01.5249732Z error: 1 errors occurred comparing output.
2019-09-21T01:11:01.5249820Z status: exit code: 1
2019-09-21T01:11:01.5249820Z status: exit code: 1
2019-09-21T01:11:01.5250779Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/auxiliary" "-A" "unused"
2019-09-21T01:11:01.5251303Z ------------------------------------------
2019-09-21T01:11:01.5251380Z 
2019-09-21T01:11:01.5251602Z ------------------------------------------
2019-09-21T01:11:01.5251691Z stderr:
2019-09-21T01:11:01.5251691Z stderr:
2019-09-21T01:11:01.5251910Z ------------------------------------------
2019-09-21T01:11:01.5252182Z error[E0463]: can't find crate for `redundant_semi_proc_macro`
2019-09-21T01:11:01.5252468Z   --> /checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs:4:1
2019-09-21T01:11:01.5253172Z    |
2019-09-21T01:11:01.5253270Z LL | extern crate redundant_semi_proc_macro;
2019-09-21T01:11:01.5253639Z 
2019-09-21T01:11:01.5253724Z error: aborting due to previous error
2019-09-21T01:11:01.5253771Z 
2019-09-21T01:11:01.5254068Z For more information about this error, try `rustc --explain E0463`.
2019-09-21T01:11:01.5254068Z For more information about this error, try `rustc --explain E0463`.
2019-09-21T01:11:01.5254129Z 
2019-09-21T01:11:01.5254385Z ------------------------------------------
2019-09-21T01:11:01.5254433Z 
2019-09-21T01:11:01.5254602Z 
2019-09-21T01:11:01.5254660Z 
2019-09-21T01:11:01.5254724Z failures:
2019-09-21T01:11:01.5255039Z     [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs
2019-09-21T01:11:01.5255418Z test result: FAILED. 9001 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out
2019-09-21T01:11:01.5255486Z 
2019-09-21T01:11:01.5276481Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T01:11:01.5276611Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T01:11:01.5276611Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T01:11:01.5291438Z 
2019-09-21T01:11:01.5291536Z 
2019-09-21T01:11:01.5294466Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T01:11:01.5295231Z 
2019-09-21T01:11:01.5295275Z 
2019-09-21T01:11:01.5307980Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-21T01:11:01.5308109Z Build completed unsuccessfully in 1:24:44
2019-09-21T01:11:01.5308109Z Build completed unsuccessfully in 1:24:44
2019-09-21T01:11:01.5367962Z == clock drift check ==
2019-09-21T01:11:01.5405623Z   local time: Sat Sep 21 01:11:01 UTC 2019
2019-09-21T01:11:01.6991729Z   network time: Sat, 21 Sep 2019 01:11:01 GMT
2019-09-21T01:11:01.6991919Z == end clock drift check ==
2019-09-21T01:11:02.4585211Z ##[error]Bash exited with code '1'.
2019-09-21T01:11:02.4622146Z ##[section]Starting: Upload CPU usage statistics
2019-09-21T01:11:02.4629340Z ==============================================================================
2019-09-21T01:11:02.4629433Z Task         : Bash
2019-09-21T01:11:02.4629524Z Description  : Run a Bash script on macOS, Linux, or Windows
