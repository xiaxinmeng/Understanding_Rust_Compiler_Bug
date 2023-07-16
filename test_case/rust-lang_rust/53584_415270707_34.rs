\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-14092.rs","byte_start":477,"byte_end":480,"line_start":11,"line_end":11,"column_start":11,"column_end":14,"is_primary":true,"text":[{"text":"fn fn1(0: Box) {}","highlight_start":11,"highlight_end":14}],"label":"expected 1 type argument","suggested_replacement":null,"suggestion_apoccurred: E0107, E0191, E0658.
[00:46:53] 25 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/issue-23024.stderr
[00:46:53] To update references, rerun the tests and pass the `--bless` flag
[00:46:53] To only update this specific test, also pass `--test-args issues/issue-23024.rs`
[00:46:53] error: 1 errors occurred comparing output.
[00:46:53] status: exit code: 1
[00:46:53] status: exit code: 1
[00:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23024.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/auxiliary" "-A" "unused"
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] ------------------------------------------
[00:46:53] stderr:
[00:46:53] stderr:
[00:46:53] ------------------------------------------
[00:46:53] {"message":"the precise format of `Fn`-family traits' type parameters is subject to change. Use parenthetical notation (Fn(Foo, Bar) -> Baz) instead (see issue #29625)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n