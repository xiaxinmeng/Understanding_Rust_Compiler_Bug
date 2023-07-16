\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-38293.rs","byte_start":724,"byte_end":727,"line_start":25,"line_end":25,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    baz(); //~ ERROR expected function, found module `baz`","highlight_start":5,"highlight_end":8}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0423]: expected function, found module `baz`\n  --> /checkout/src/test/ui/issue-38293.rs:25:5\n   |\nLL |     baz(); //~ ERROR expected function, found module `baz`\n   |     ^^^ not a function\n\n"}
[00:49:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:33] {"message":"Some errors occurred: E0423, E0432.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0423, E0432.\n"}
[00:49:33] {"message":"For more information about an error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0423`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/issue-38293.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/issue-38412.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 2   --> $DIR/issue-38412.rs:12:9
[00:49:33] 3    |
[00:49:33] 4 LL |     let Box(a) = loop { };
[00:49:33] -    |         ^^^ constructor is not visible here due to private fields
[00:49:33] +    |      linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-4366-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-4366-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-4366-2/auxiliary" "-A" "unused"
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] ------------------------------------------
[00:49:33] stderr:
[00:49:33] stderr:
[00:49:33] ------------------------------------------
[00:49:33] {"message":"cannot find type `bar` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n