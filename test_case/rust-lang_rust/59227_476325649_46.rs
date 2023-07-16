\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-7607-1.rs","byte_start":34,"byte_end":36,"line_start":5,"line_end":5,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"impl Fo { //~ ERROR cannot find type `Fo` in this scope","highlight_start":6,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a struct with a similar name exists","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-7607-1.rs","byte_start":34,"byte_end":36,"line_start":5,"line_end":5,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"impl Fo { //~ ERROR cannot find type `Fo` in this scope","highlight_start":6,"highlight_end":8}],"label":null,"suggested_replacement":"Foo","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0412]: cannot find type `Fo` in this scope\n  --> /checkout/src/test/ui/issues/issue-7607-1.rs:5:6\n   |\nLL | impl Fo { //~ ERROR cannot find type `Fo` in this scope\n   |      ^^ help: a struct with a similar name exists: `Foo`\n\n"}
[01:15:56] {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0412`.\n"}
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] 
[01:15:56] thread '[ui] ui/issues/issue-7607-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:15:56] 
[01:15:56] ---- [ui] ui/resolve/levenshtein.rs stdout ----
[01:15:56] diff of stderr:
[01:15:56] 
[01:15:56] 2   --> $DIR/levenshtein.rs:5:11
[01:15:56] 3    |
[01:15:56] 4 LL | fn foo(c: esize) {} // Misspelled primitive type name.
[01:15:56] -    |           ^^^^^ help: a primitive type with a similar name exists: `isize`
[01:15:56] +    |           ^^^^^ help: a primitive type with a similar name exists: `usize`
[01:15:56] 7 error[E0412]: cannot find type `Baz` in this scope
[01:15:56] 8   --> $DIR/levenshtein.rs:10:10
[01:15:56] 
[01:15:56] 
[01:15:56] 
[01:15:56] The actual stderr differed from the expected stderr.
[01:15:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/levenshtein.stderr
[01:15:56] To update references, rerun the tests and pass the `--bless` flag
[01:15:56] To only update this specific test, also pass `--test-args resolve/levenshtein.rs`
[01:15:56] error: 1 errors occurred comparing output.
[01:15:56] status: exit code: 1
[01:15:56] status: exit code: 1
[01:15:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/levenshtein.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/auxiliary" "-A" "unused"
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] stderr:
[01:15:56] stderr:
[01:15:56] ------------------------------------------
[01:15:56] {"message":"cannot find type `esize` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n