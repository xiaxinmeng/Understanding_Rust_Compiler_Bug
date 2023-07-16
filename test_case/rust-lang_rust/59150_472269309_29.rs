\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-in-while.rs","byte_start":220,"byte_end":222,"line_start":14,"line_end":14,"column_start":7,"column_end":9,"is_primary":true,"text":[{"text":"    }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`","highlight_start":7,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `hi` found for type `()` in the current scope\n  --> /checkout/src/test/ui/parser/struct-literal-in-while.rs:14:7\n   |\nLL |     }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`\n   |       ^^\n\n"}
[01:17:14] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:17:14] {"message":"Some errors occurred: E0423, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0423, E0599.\n"}
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] thread '[ui] ui/parser/struct-literal-in-while.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:14] thread '[ui] ui/parser/struct-literal-in-while.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:14] 
[01:17:14] ---- [ui] ui/parser/struct-literal-restrictions-in-lamda.rs stdout ----
[01:17:14] diff of stderr:
[01:17:14] 
[01:17:14] 10    |
[01:17:14] 11 LL |         x: 3
[01:17:14] 12    |         ^
[01:17:14] - note: ...due to this, which is why a type is expected
[01:17:14] + note: ...due to this, which is why a type is expected after
[01:17:14] 15    |
[01:17:14] 16 LL |         x: 3
[01:17:14] 
[01:17:14] 
[01:17:14] 
[01:17:14] The actual stderr differed from the expected stderr.
[01:17:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-restrictions-in-lamda/struct-literal-restrictions-in-lamda.stderr
[01:17:14] To update references, rerun the tests and pass the `--bless` flag
[01:17:14] To only update this specific test, also pass `--test-args parser/struct-literal-restrictions-in-lamda.rs`
[01:17:14] error: 1 errors occurred comparing output.
[01:17:14] status: exit code: 1
[01:17:14] status: exit code: 1
[01:17:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-restrictions-in-lamda/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-restrictions-in-lamda/auxiliary" "-A" "unused"
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] stderr:
[01:17:14] stderr:
[01:17:14] ------------------------------------------
[01:17:14] {"message":"expected type, found `3`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs","byte_start":177,"byte_end":178,"line_start":13,"line_end":13,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"        x: 3    //~ ERROR expected type, found `3`","highlight_start":12,"highlight_end":13}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this expression is annotated with type ascription...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs","byte_start":174,"byte_end":175,"line_start":13,"line_end":13,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        x: 3    //~ ERROR expected type, found `3`","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...due to this, which is why a type is expected after","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs","byte_start":175,"byte_end":176,"line_start":13,"line_end":13,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"        x: 3    //~ ERROR expected type, found `3`","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this might be indicative of a syntax error elsewhere","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: expected type, found `3`\n  --> /checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs:13:12\n   |\nLL |         x: 3    //~ ERROR expected type, found `3`\n   |            ^ expecting a type here because of type ascription\n   |\n   = note: type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`\nnote: this expression is annotated with type ascription...\n  --> /checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs:13:9\n   |\nLL |         x: 3    //~ ERROR expected type, found `3`\n   |         ^\nnote: ...due to this, which is why a type is expected after\n  --> /checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs:13:10\n   |\nLL |         x: 3    //~ ERROR expected type, found `3`\n   |          ^\n   = help: this might be indicative of a syntax error elsewhere\n\n"}
[01:17:14] {"message":"expected one of `.`, `;`, `?`, `}`, or an operator, found `{`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs","byte_start":228,"byte_end":229,"line_start":14,"line_end":14,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`","highlight_start":12,"highlight_end":13}],"label":"expected one of `.`, `;`, `?`, `}`, or an operator here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `.`, `;`, `?`, `}`, or an operator, found `{`\n  --> /checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs:14:12\n   |\nLL |     }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`\n   |            ^ expected one of `.`, `;`, `?`, `}`, or an operator here\n\n"}
[01:17:14] {"message":"expected value, found struct `Foo`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n