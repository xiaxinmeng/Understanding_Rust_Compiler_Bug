\n\nRemember this solution is unsafe! You will have to ensure that accesses to the\ncell are synchronized.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17718-const-borrow.rs","byte_start":582,"byte_end":584,"line_start":14,"line_end":14,"column_start":39,"column_end":41,"is_primary":true,"text":[{"text":"const B: &'static UnsafeCell<usize> = &A;","highlight_start":39,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0492]: cannot borrow a constant which may contain interior mutability,
[01:00:57] + 
[01:00:57] + error[E0013]: constants cannot refer to statics, use a constant instead
[01:00:57] +   --> $DIR/issue-17718-references.rs:19:28
[01:00:57] +    |
[01:00:57] + LL | const T2: &'static usize = &S; //~ ERROR: constants cannot refer to statics
[01:00:57] 18 
[01:00:57] 19 error: aborting due to 3 previous errors
[01:00:57] 20 
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-references/issue-17718-references.stderr
[01:00:57] To update references, rerun the tests and pass the `--bless` flag
[01:00:57] To only update this specific test, also pass `--test-args issues/issue-17718-references.rs`
[01:00:57] error: 1 errors occurred comparing output.
[01:00:57] status: exit code: 1
[01:00:57] status: exit code: 1
[01:00:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-references.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-references/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-references/auxiliary" "-A" "unused"
[01:00:57] ------------------------------------------
[01:00:57] 
[01:00:57] ------------------------------------------
[01:00:57] ------------------------------------------
[01:00:57] stde   let f = Bar();","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0618]: expected function, found `Bar`\n  --> /checkout/src/test/ui/issues/issue-21701.rs:19:13\n   |\nLL | struct Bar;\n   | ----------- `Bar` defined here\n...\nLL |     let f = Bar();\n   |             ^^^--\n   |             |\n   |             call expression requires function\n\n"}
[01:00:57] {"message":"expected function, found `U`","code":{"code":"E0618","explanation":"\nAttempted to call something which isn't a function nor a method.\n\nErroneous code examples:\n\n