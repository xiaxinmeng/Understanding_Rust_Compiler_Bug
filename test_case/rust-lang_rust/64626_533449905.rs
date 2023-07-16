plain
2019-09-20T07:44:16.8489997Z test [ui] ui/lint/reasons-erroneous.rs ... ok
2019-09-20T07:44:16.9914108Z test [ui] ui/lint/reasons.rs ... ok
2019-09-20T07:44:17.0485976Z test [ui] ui/lint/rfc-2457-non-ascii-idents/lint-non-ascii-idents.rs ... ok
2019-09-20T07:44:17.1426012Z test [ui] ui/lint/suggestions.rs ... ok
2019-09-20T07:44:17.2124704Z test [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs ... FAILED
2019-09-20T07:44:17.2547957Z test [ui] ui/lint/trivial-casts.rs ... ok
2019-09-20T07:44:17.2548392Z test [ui] ui/lint/trivial-casts-featuring-type-ascription.rs ... ok
2019-09-20T07:44:17.3450613Z test [ui] ui/lint/uninitialized-zeroed.rs ... ok
2019-09-20T07:44:17.4106524Z test [ui] ui/lint/type-overflow.rs ... ok
---
2019-09-20T07:50:37.4636140Z test [ui] ui/wrapping-int-combinations.rs ... ok
2019-09-20T07:50:37.4636350Z 
2019-09-20T07:50:37.4636424Z failures:
2019-09-20T07:50:37.4680593Z 
2019-09-20T07:50:37.4681213Z ---- [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs stdout ----
2019-09-20T07:50:37.4681380Z 
2019-09-20T07:50:37.4681380Z 
2019-09-20T07:50:37.4684775Z - TokenStream [Ident { ident: "fn", span: #0 bytes(197..199) }, Ident { ident: "span_preservation", span: #0 bytes(200..217) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(217..219) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "let", span: #0 bytes(227..230) }, Ident { ident: "tst", span: #0 bytes(231..234) }, Punct { ch: '=', spacing: Alone, span: #0 bytes(235..236) }, Literal { lit: Lit { kind: Integer, symbol: 123, suffix: None }, span: Span { lo: BytePos(237), hi: BytePos(240), ctxt: #0 } }, Punct { ch: ';', spacing: Joint, span: #0 bytes(240..241) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(241..242) }, Ident { ident: "match", span: #0 bytes(288..293) }, Ident { ident: "tst", span: #0 bytes(294..297) }, Group { delimiter: Brace, stream: TokenStream [Literal { lit: Lit { kind: Integer, symbol: 123, suffix: None }, span: Span { lo: BytePos(482), hi: BytePos(485), ctxt: #0 } }, Punct { ch: '=', spacing: Joint, span: #0 bytes(486..488) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(486..488) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(489..491) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(491..492) }, Ident { ident: "_", span: #0 bytes(501..502) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(503..505) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(503..505) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(506..508) }], span: #0 bytes(298..514) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(514..515) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(515..516) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(516..517) }], span: #0 bytes(221..561) }]
2019-09-20T07:50:37.4686481Z -   --> $DIR/redundant-semi-proc-macro.rs:9:19
2019-09-20T07:50:37.4686481Z -   --> $DIR/redundant-semi-proc-macro.rs:9:19
2019-09-20T07:50:37.4686788Z + error[E0463]: can't find crate for `redundant_semi_proc_macro`
2019-09-20T07:50:37.4687089Z +   --> $DIR/redundant-semi-proc-macro.rs:4:1
2019-09-20T07:50:37.4687171Z 4    |
2019-09-20T07:50:37.4687433Z - LL |     let tst = 123;;
2019-09-20T07:50:37.4687706Z -    |                   ^ help: remove this semicolon
2019-09-20T07:50:37.4688216Z - note: lint level defined here
2019-09-20T07:50:37.4688481Z -   --> $DIR/redundant-semi-proc-macro.rs:3:9
2019-09-20T07:50:37.4688717Z -    |
2019-09-20T07:50:37.4688717Z -    |
2019-09-20T07:50:37.4688964Z - LL | #![deny(redundant_semicolon)]
2019-09-20T07:50:37.4689245Z -    |         ^^^^^^^^^^^^^^^^^^^
2019-09-20T07:50:37.4689330Z + LL | extern crate redundant_semi_proc_macro;
2019-09-20T07:50:37.4689911Z 13 
2019-09-20T07:50:37.4690175Z - error: unnecessary trailing semicolons
2019-09-20T07:50:37.4690453Z -   --> $DIR/redundant-semi-proc-macro.rs:16:7
2019-09-20T07:50:37.4690676Z -    |
---
2019-09-20T07:50:37.4692198Z 22 
2019-09-20T07:50:37.4692238Z 
2019-09-20T07:50:37.4692274Z 
2019-09-20T07:50:37.4692364Z The actual stderr differed from the expected stderr.
2019-09-20T07:50:37.4692783Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/redundant-semi-proc-macro.stderr
2019-09-20T07:50:37.4694180Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T07:50:37.4694991Z To only update this specific test, also pass `--test-args lint/redundant-semicolon/redundant-semi-proc-macro.rs`
2019-09-20T07:50:37.4695143Z error: 1 errors occurred comparing output.
2019-09-20T07:50:37.4695238Z status: exit code: 1
2019-09-20T07:50:37.4695238Z status: exit code: 1
2019-09-20T07:50:37.4696449Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/auxiliary" "-A" "unused"
2019-09-20T07:50:37.4697294Z ------------------------------------------
2019-09-20T07:50:37.4697364Z 
2019-09-20T07:50:37.4697605Z ------------------------------------------
2019-09-20T07:50:37.4697698Z stderr:
2019-09-20T07:50:37.4697698Z stderr:
2019-09-20T07:50:37.4697935Z ------------------------------------------
2019-09-20T07:50:37.4698227Z error[E0463]: can't find crate for `redundant_semi_proc_macro`
2019-09-20T07:50:37.4698532Z   --> /checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs:4:1
2019-09-20T07:50:37.4698654Z    |
2019-09-20T07:50:37.4698740Z LL | extern crate redundant_semi_proc_macro;
2019-09-20T07:50:37.4699087Z 
2019-09-20T07:50:37.4699159Z error: aborting due to previous error
2019-09-20T07:50:37.4699204Z 
2019-09-20T07:50:37.4700142Z For more information about this error, try `rustc --explain E0463`.
2019-09-20T07:50:37.4700142Z For more information about this error, try `rustc --explain E0463`.
2019-09-20T07:50:37.4700225Z 
2019-09-20T07:50:37.4700768Z ------------------------------------------
2019-09-20T07:50:37.4700837Z 
2019-09-20T07:50:37.4700872Z 
2019-09-20T07:50:37.4700925Z 
2019-09-20T07:50:37.4700988Z failures:
2019-09-20T07:50:37.4701322Z     [ui] ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs
2019-09-20T07:50:37.4701787Z test result: FAILED. 8980 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out
2019-09-20T07:50:37.4701853Z 
2019-09-20T07:50:37.4725708Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T07:50:37.4725850Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T07:50:37.4725850Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T07:50:37.4742824Z 
2019-09-20T07:50:37.4742946Z 
2019-09-20T07:50:37.4745673Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T07:50:37.4746635Z 
2019-09-20T07:50:37.4746678Z 
2019-09-20T07:50:37.4766365Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-20T07:50:37.4766878Z Build completed unsuccessfully in 1:20:01
2019-09-20T07:50:37.4766878Z Build completed unsuccessfully in 1:20:01
2019-09-20T07:50:37.4808133Z == clock drift check ==
2019-09-20T07:50:37.4823518Z   local time: Fri Sep 20 07:50:37 UTC 2019
2019-09-20T07:50:37.7793595Z   network time: Fri, 20 Sep 2019 07:50:37 GMT
2019-09-20T07:50:37.7801673Z == end clock drift check ==
2019-09-20T07:50:38.6505360Z ##[error]Bash exited with code '1'.
2019-09-20T07:50:38.6543034Z ##[section]Starting: Upload CPU usage statistics
2019-09-20T07:50:38.6551838Z ==============================================================================
2019-09-20T07:50:38.6551958Z Task         : Bash
2019-09-20T07:50:38.6552021Z Description  : Run a Bash script on macOS, Linux, or Windows
