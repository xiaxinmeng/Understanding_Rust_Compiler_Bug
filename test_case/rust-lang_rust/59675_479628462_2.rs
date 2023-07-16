\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0254.rs","byte_start":148,"byte_end":158,"line_start":11,"line_end":11,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"use foo::alloc;","highlight_start":5,"highlight_end":15}],"label":"`alloc` reimported here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0254.rs","byte_start":55,"byte_end":74,"line_start":3,"line_end":3,"column_start":1,"column_end":20,"is_primary":false,"text":[{"text":"extern crate alloc;","highlight_start":1,"highlight_end":20}],"label":"previous import of the extern crate `alloc` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`alloc` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use `as` to change the binding name of the import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0254.rs","byte_start":148,"byte_end":158,"line_start":11,"line_end":11,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"use foo::alloc;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"foo::alloc as other_alloc","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0254]: the name `alloc` is defined multiple times\n  --> /checkout/src/test/ui/error-codes/E0254.rs:11:5\n   |\nLL | extern crate alloc;\n   | ------------------- previous import of the extern crate `alloc` here\n...\nLL | use foo::alloc;\n   |     ^^^^^^^^^^ `alloc` reimported here\n   |\n   = note: `alloc` must be defined only once in the type namespace of this module\nhelp: you can use `as` to change the binding name of the import\n   |\nLL | use foo::alloc as other_alloc;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:15:23] {"message":"For more information about this error, try `rustc --explain E0254`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0254`.\n"}
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] 
---
[01:15:23] 1 error[E0260]: the name `alloc` is defined multiple times
[01:15:23] -   --> $DIR/E0260.rs:6:1
[01:15:23] +   --> $DIR/E0260.rs:5:1
[01:15:23] 3    |
[01:15:23] 4 LL | extern crate alloc;
[01:15:23] 5    | ------------------- previous import of the extern crate `alloc` here
[01:15:23] 
[01:15:23] The actual stderr differed from the expected stderr.
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0260/E0260.stderr
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0260/E0260.stderr
[01:15:23] To update references, rerun the tests and pass the `--bless` flag
[01:15:23] To only update this specific test, also pass `--test-args error-codes/E0260.rs`
[01:15:23] error: 1 errors occurred comparing output.
[01:15:23] status: exit code: 1
[01:15:23] status: exit code: 1
[01:15:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0260.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0260/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0260/auxiliary" "-A" "unused"
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] stderr:
[01:15:23] stderr:
[01:15:23] ------------------------------------------
[01:15:23] {"message":"the name `alloc` is defined multiple times","code":{"code":"E0260","explanation":"\nThe name for an item declaration conflicts with an external crate's name.\n\nErroneous code example:\n\n