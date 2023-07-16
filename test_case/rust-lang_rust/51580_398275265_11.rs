\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will defirendered":"For more information about this error, try `rustc --explain E0038`.\n"}
[00:43:54] ------------------------------------------
[00:43:54] 
[00:43:54] thread '[ui] ui/did_you_mean/issue-40006.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:54] 
[00:43:54] ---- [ui] ui/token/issue-41155.rs stdout ----
[00:43:54] diff of stderr:
[00:43:54] 
[00:43:54] - error: expected one of `(`, `const`, `default`, `extern`, `fn`, `type`, or `unsafe`, found `}`
[00:43:54] + error: expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `type`, or `unsafe`, found `}`
[00:43:54] 2   --> $DIR/issue-41155.rs:13:1
[00:43:54] 3    |
[00:43:54] 4 LL |     pub
[00:43:54] 
[00:43:54] -    |        - expected one of 7 possible tokens here
[00:43:54] +    |        - expected one of 8 possible tokens here
[00:43:54] 6 LL | } //~ ERROR expected one of
[00:43:54] 7    | ^ unexpected token
[00:43:54] 
[00:43:54] 
[00:43:54] The actual stderr differed from the expected stderr.
[00:43:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-41155/issue-41155.stderr
[00:43:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-41155/issue-41155.stderr
[00:43:54] To update references, rerun the tests and pass the `--bless` flag
[00:43:54] To only update this specific test, also pass `--test-args token/issue-41155.rs`
[00:43:54] error: 1 errors occurred comparing output.
[00:43:54] status: exit code: 101
[00:43:54] status: exit code: 101
[00:43:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/token/issue-41155.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-41155/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-41155/auxiliary" "-A" "unused"
[00:43:54] ------------------------------------------
[00:43:54] 
[00:43:54] ------------------------------------------
[00:43:54] stderr:
[00:43:54] stderr:
[00:43:54] ------------------------------------------
[00:43:54] {"message":"expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `type`, or `unsafe`, found `}`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-41155.rs","byte_start":510,"byte_end":510,"line_start":12,"line_end":12,"column_start":8,"column_end":8,"is_primary":false,"text":[{"text":"    pub","highlight_start":8,"highlight_end":8}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/token/issue-41155.rs","byte_start":511,"byte_end":512,"line_start":13,"line_end":13,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"} //~ ERROR expected one of","highlight_start":1,"highlight_end":2}],"label":"unexpected token","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `type`, or `unsafe`, found `}`\n  --> /checkout/src/test/ui/token/issue-41155.rs:13:1\n   |\nLL |     pub\n   |        - expected one of 8 possible tokens here\nLL | } //~ ERROR expected one of\n   | ^ unexpected token\n\n"}
[00:43:54] {"message":"cannot find type `S` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n