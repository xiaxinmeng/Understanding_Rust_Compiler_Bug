\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":891,"byte_end":906,"line_start":31,"line_end":31,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    for _ in std::ops::Range { start: 0, end: 10 } {}","highlight_start":14,"highlight_end":29}],"label":"did you mean `(std::ops::Range { /* fields */ })`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0423]: expected value, found struct `std::ops::Range`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:31:14\n   |\nLL |     for _ in std::ops::Range { start: 0, end: 10 } {}\n   |              ^^^^^^^^^^^^^^^ did you mean `(std::ops::Range { /* fields */ })`?\n\n"}
[00:44:06] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:44:06] {"message":"For more information about this error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0423`.\n"}
[00:44:06] ------------------------------------------
[00:44:06] 
[00:44:06] thread '[ui] ui/error-codes/E0423.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:44:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:06] 
[00:44:06] ---- [ui] ui/resolve/privacy-struct-ctor.rs stdout ----
[00:44:06] diff of stderr:
[00:44:06] 
[00:44:06] 16    |
[00:44:06] 17 LL |     S;
[00:44:06] 18    |     ^ constructor is not visible here due to private fields
[00:44:06] - help: possible better candidate is found in another module, you can import it into scope
[00:44:06] + help: possible better candidates are found in other modules, you can import them into scope
[00:44:06] 20    |
[00:44:06] 21 LL | use m::S;
[00:44:06] 
[00:44:06] 
[00:44:06] + LL | use std::time::S;
[00:44:06] 23 
[00:44:06] 24 error[E0423]: expected value, found struct `S2`
[00:44:06] 25   --> $DIR/privacy-struct-ctor.rs:48:5
[00:44:06] 
[00:44:06] 
[00:44:06] 32    |
[00:44:06] 33 LL |     xcrate::S;
[00:44:06] 34    |     ^^^^^^^^^ constructor is not visible here due to private fields
[00:44:06] - help: possible better candidate is found in another module, you can import it into scope
[00:44:06] + help: possible better candidates are found in other modules, you can import them into scope
[00:44:06] 36    |
[00:44:06] 37 LL | use m::S;
[00:44:06] +    |
[00:44:06] + LL | use std::time::S;
[00:44:06] 39 
[00:44:06] 39 
[00:44:06] 40 error[E0603]: tuple struct `Z` is private
[00:44:06] 
[00:44:06] The actual stderr differed from the expected stderr.
[00:44:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/privacy-struct-ctor.stderr
[00:44:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/privacy-struct-ctor.stderr
[00:44:06] To update references, rerun the tests and pass the `--bless` flag
[00:44:06] To only update this specific test, also pass `--test-args resolve/privacy-struct-ctor.rs`
[00:44:06] error: 1 errors occurred comparing output.
[00:44:06] status: exit code: 1
[00:44:06] status: exit code: 1
[00:44:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-struct-ctor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-struct-ctor/auxiliary" "-A" "unused"
[00:44:06] ------------------------------------------
[00:44:06] 
[00:44:06] ------------------------------------------
[00:44:06] stderr:
[00:44:06] stderr:
[00:44:06] ------------------------------------------
[00:44:06] {"message":"expected value, found struct `Z`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n