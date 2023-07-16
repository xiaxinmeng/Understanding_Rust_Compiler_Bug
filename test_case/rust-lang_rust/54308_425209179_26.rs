\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/empty/empty-struct-braces-expr.rs","byte_start":1227,"byte_end":1237,"line_start":33,"line_end":33,"column_start":15,"column_end":25,"is_primary":true,"text":[{"text":"    let xe3 = XE::Empty3(); //~ ERROR no variant named `Empty3` found for type","highlight_start":15,"highlight_end":25}],"label":"variant not found in `empty_struct::XE`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `empty_struct::XE::XEmpty3`?","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no variant named `Empty3` found for type `empty_struct::XE` in the current scope\n  --> /checkout/src/test/ui/empty/empty-struct-braces-expr.rs:33:15\n   |\nLL |     let xe3 = XE::Empty3(); //~ ERROR no variant named `Empty3` found for type\n   |               ^^^^^^^^^^ variant not found in `empty_struct::XE`\n   |\n   = note: did you mean `empty_struct::XE::XEmpty3`?\n\n"}
[00:52:49] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:52:49] {"message":"Some errors occurred: E0423, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0423, E0599.\n"}
[00:52:49] {"message":"For more information about an error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0423`.\n"}
[00:52:49] ------------------------------------------
[00:52:49] 
[00:52:49] thread '[ui] ui/empty/empty-struct-braces-expr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:52:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:49] 
[00:52:49] ---- [ui] ui/error-codes/E0423.rs stdout ----
[00:52:49] diff of stderr:
[00:52:49] 
[00:52:49] 20   --> $DIR/E0423.rs:14:13
[00:52:49] 21    |
[00:52:49] 22 LL |     let f = Foo(); //~ ERROR E0423
[00:52:49] -    |             |
[00:52:49] -    |             did you mean `foo`?
[00:52:49] -    |             did you mean `foo`?
[00:52:49] -    |             did you mean `Foo { /* fields */ }`?
[00:52:49] +    |             ^^^ did you mean `foo`?
[00:52:49] 28 error[E0423]: expected value, found struct `S`
[00:52:49] 29   --> $DIR/E0423.rs:22:32
[00:52:49] 
[00:52:49] 
[00:52:49] 
[00:52:49] The actual stderr differed from the expected stderr.
[00:52:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/E0423.stderr
[00:52:49] To update references, rerun the tests and pass the `--bless` flag
[00:52:49] To only update this specific test, also pass `--test-args error-codes/E0423.rs`
[00:52:49] error: 1 errors occurred comparing output.
[00:52:49] status: exit code: 1
[00:52:49] status: exit code: 1
[00:52:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0423.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/auxiliary" "-A" "unused"
[00:52:49] ------------------------------------------
[00:52:49] 
[00:52:49] ------------------------------------------
[00:52:49] stderr:
[00:52:49] stderr:
[00:52:49] ------------------------------------------
[00:52:49] {"message":"expected type, found `1`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":670,"byte_end":671,"line_start":22,"line_end":22,"column_start":39,"column_end":40,"is_primary":true,"text":[{"text":"    if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }","highlight_start":39,"highlight_end":40}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected type, found `1`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:22:39\n   |\nLL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }\n   |                                       ^ expecting a type here because of type ascription\n\n"}
[00:52:49] {"message":"expected expression, found `==`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":768,"byte_end":770,"line_start":25,"line_end":25,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    if T {} == T {} { println!(\"Ok\"); }","highlight_start":13,"highlight_end":15}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `==`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:25:13\n   |\nLL |     if T {} == T {} { println!(\"Ok\"); }\n   |             ^^ expected expression\n\n"}
[00:52:49] {"message":"expected type, found `0`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":916,"byte_end":917,"line_start":31,"line_end":31,"column_start":39,"column_end":40,"is_primary":true,"text":[{"text":"    for _ in std::ops::Range { start: 0, end: 10 } {}","highlight_start":39,"highlight_end":40}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected type, found `0`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:31:39\n   |\nLL |     for _ in std::ops::Range { start: 0, end: 10 } {}\n   |                                       ^ expecting a type here because of type ascription\n\n"}
[00:52:49] {"message":"expected function, found struct `Foo`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n