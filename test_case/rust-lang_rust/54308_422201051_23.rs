\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-39226.rs","byte_start":625,"byte_end":631,"line_start":20,"line_end":20,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"        handle: Handle","highlight_start":17,"highlight_end":23}],"label":"did you mean `handle`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0423]: expected value, found struct `Handle`\n  --> /checkout/src/test/ui/resolve/issue-39226.rs:20:17\n   |\nLL |         handle: Handle\n   |                 ^^^^^^ did you mean `handle`?\n\n"}
[01:05:24] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:24] {"message":"For more information about this error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0423`.\n"}
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] thread '[ui] ui/resolve/issue-39226.rs' panicked at 'explicit panic', tools/r module, you can import it into scope
[01:05:24] 36    |
[01:05:24] 36    |
[01:05:24] 37 LL | use m::S;
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/privacy-struct-ctor.stderr
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/privacy-struct-ctor.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args resolve/privacy-struct-ctor.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-struct-ctor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"expected value, found struct `Z`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n