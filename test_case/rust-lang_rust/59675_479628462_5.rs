\n\nSee the Declaration Statements section of the reference for more information\nabout what constitutes an Item declaration and what does not:\n\nhttps://doc.rust-lang.org/reference.html#statements\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0260.rs","byte_start":54,"byte_end":63,"line_start":5,"line_end":5,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"mod alloc {","highlight_start":1,"highlight_end":10}],"label":"`alloc` redefined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0260.rs","byte_start":33,"byte_end":52,"line_start":3,"line_end":3,"column_start":1,"column_end":20,"is_primary":false,"text":[{"text":"extern crate alloc;","highlight_start":1,"highlight_end":20}],"label":"previous import of the extern crate `alloc` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`alloc` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use `as` to change the binding name of the import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0260.rs","byte_start":33,"byte_end":52,"line_start":3,"line_end":3,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate alloc;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"extern crate alloc as other_alloc;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0260]: the name `alloc` is defined multiple times\n  --> /checkout/src/test/ui/error-codes/E0260.rs:5:1\n   |\nLL | extern crate alloc;\n   | ------------------- previous import of the extern crate `alloc` here\nLL | \nLL | mod alloc {\n   | ^^^^^^^^^ `alloc` redefined here\n   |\n   = note: `alloc` must be defined only once in the type namespace of this module\nhelp: you can use `as` to change the binding name of the import\n   |\nLL | extern crate alloc as other_alloc;\n   |\n\n"}
[01:15:23] {"message":"For more information about this error, try `rustc --explain E0260`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0260`.\n"}
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] 
---
[01:15:23] 15    |
[01:15:23] 16 LL |             use alloc::HashMap;
[01:15:23] 17    |                 ^^^^^
[01:15:23] 
[01:15:23] 20    |                 help: a similar path exists: `a::alloc`
[01:15:23] 22 error[E0432]: unresolved import `alloc`
[01:15:23] -   --> $DIR/resolve_self_super_hint.rs:21:21
[01:15:23] +   --> $DIR/resolve_self_super_hint.rs:20:21
[01:15:23] 24    |
[01:15:23] 24    |
[01:15:23] 25 LL |                 use alloc::HashMap;
[01:15:23] 26    |                     ^^^^^
[01:15:23] 
[01:15:23] 
[01:15:23] The actual stderr differed from the expected stderr.
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve_self_super_hint/resolve_self_super_hint.stderr
[01:15:23] To update references, rerun the tests and pass the `--bless` flag
[01:15:23] To only update this specific test, also pass `--test-args resolve_self_super_hint.rs`
[01:15:23] error: 1 errors occurred comparing output.
[01:15:23] status: exit code: 1
[01:15:23] status: exit code: 1
[01:15:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve_self_super_hint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve_self_super_hint/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve_self_super_hint/auxiliary" "-A" "unused"
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] stderr:
[01:15:23] stderr:
[01:15:23] ------------------------------------------
[01:15:23] {"message":"unresolved import `alloc`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n