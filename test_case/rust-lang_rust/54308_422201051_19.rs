\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":521,"byte_end":524,"line_start":14,"line_end":14,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let f = Foo(); //~ ERROR E0423","highlight_start":13,"highlight_end":16}],"label":"did you mean `foo`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":521,"byte_end":524,"line_start":14,"line_end":14,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let f = Foo(); //~ ERROR E0423","highlight_start":13,"highlight_end":16}],"label":"did you mean `Foo { /* fielt panic', tools/compiletest/src/runtest.rs:3205:9
[01:05:24] ---- [ui] ui/issues/issue-38412.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 2   --> $DIR/issue-38412.rs:12:9
[01:05:24] 2   --> $DIR/issue-38412.rs:12:9
[01:05:24] 3    |
[01:05:24] 4 LL |     let Box(a) = loop { };
[01:05:24] -    |         ^^^ constructor is not visible here due to private fields
[01:05:24] 6 
[01:05:24] 7 error: aborting due to previous error
[01:05:24] 8 
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38412/issue-38412.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args issues/issue-38412.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38412.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38412/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38412/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
[01:05:24] -------------------------------------e this specific test, also pass `--test-args resolve/issue-22692.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-22692.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-22692/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-22692/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"expected value, found struct `String`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n