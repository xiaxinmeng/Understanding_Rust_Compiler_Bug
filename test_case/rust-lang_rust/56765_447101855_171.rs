\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-47706.rs","byte_start":848,"byte_end":851,"line_start":37,"line_end":37,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo(Qux::Bar);","highlight_start":5,"highlight_end":8}],"label":"expected function that takes 0 arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-47706.rs","byte_start":779,"byte_end":787,"line_start":27,"line_end":27,"column_start":5,"column_end":13,"is_primary":false,"text":[{"text":"    Bar(i32),","highlight_start":5,"highlight_end":13}],"label":"takes 1 argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-47706.rs","byte_start":792,"byte_end":830,"line_start":30,"line_end":34,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo<F>(f: F)","highlight_start":1,"highlight_end":16},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    F: Fn(),","highlight_start":1,"highlight_end":13},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_repla:00:57] 9    |
[01:00:57] 10 LL |     [(); return || {}];
[01:00:57] 21    |
[01:00:57] 21    |
[01:00:57] 22 LL |     [(); return while let Some(n) = Some(0) {}];
[01:00:57] + 
[01:00:57] + 
[01:00:57] + error[E0572]: return statement outside of function body
[01:00:57] +   --> $DIR/issue-51714.rs:12:14
[01:00:57] +    |
[01:00:57] + LL |     |_:  [_; return || {}] | {};
[01:00:57] 24 
[01:00:57] 25 error[E0165]: irrefutable while-let pattern
[01:00:57] 26   --> $DIR/issue-51714.rs:21:27
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/issue-51714.stderr
[01:00:57] To update references, rerun the tests and pass the `--bless` flag
[01:00:57] To only update this specific test, also pass `--test-args issues/issue-51714.rs`
[01:00:57] error: 1 errors occurred comparing output.
[01:00:57] status: exit code: 1
[01:00:57] status: exit code: 1
[01:00:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51714.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/auxiliary" "-A" "unused"
[01:00:57] stdout:
[01:00:57] ------e return keyword or move the expression into a\nfunction. Example:\n\n