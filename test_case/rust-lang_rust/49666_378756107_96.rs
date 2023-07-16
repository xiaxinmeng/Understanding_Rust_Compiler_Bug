\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-46472.rs","byte_start":565,"byte_end":566,"line_start":14,"line_end":14,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    &mut 4","highlight_start":10,"highlight_end":11}],"label":"temporary value does not live long enough","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-46472.rs","byte_start":707,"byte_end":708,"line_start":17,"line_end":17,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the lifetime 'a as defined on the function body at 13:1...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-46472.rs","byte_start":526,"byte_end":553,"line_start":13,"line_end":13,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"fn bar<'a>() -> &'a mut u32 {","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough (Mir)\n  --> /checkout/src/test/ui/issue-46472.rs:14:t":17,"line_end":17,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the lifetime 'a as defined on the function body at 13:1...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-46472.rs","byte_start":526,"byte_end":553,"line_start":13,"line_end":13,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"fn bar<'a>() -> &'a mut u32 {","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough (Ast)\n  --> /checkout/src/test/ui/issue-46472.rs:14:10\n   |\nLL |     &mut 4\n   |          ^ temporary value does not live long enough\n...\nLL | }\n   | - temporary value only lives until here\n   |\nnote: borrowed value must be valid for the lifetime 'a as defined on the function body at 13:1...\n  --> /checkout/src/test/ui/issue-46472.rs:13:1\n   |\nLL | fn bar<'a>() -> &'a mut u32 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:42:43] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:42:43] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:42:43]
[00:42:43] -----first assignment to `x`
[00:42:43] + LL |             x = 3;      //~ ERROR (Ast) [E0384]
[00:42:43] +    |             ^^^^^ cannot assign twice to immutable variable
[00:42:43] +
[00:42:43] + error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] 18   --> $DIR/liveness-assign-imm-local-notes.rs:45:13
[00:42:43] 19    |
[00:42:43] 20 LL |             x = 1;      //~ ERROR (Ast) [E0384]
[00:42:43]
[00:42:43] 21    |             ^^^^^ cannot assign twice to immutable variable
[00:42:43] 22
[00:42:43] - error[E0384]: cannot assign twice to immutable variable `x` (Ast)
[00:42:43] + error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] 24   --> $DIR/liveness-assign-imm-local-notes.rs:48:13
[00:42:43] 25    |
[00:42:43] 26 LL |             x = 1;      //~ ERROR (Ast) [E0384]
[00:42:43]
[00:42:43] 29 LL |             x = 2;      //~ ERROR (Ast) [E0384]
[00:42:43] 30    |             ^^^^^ cannot assign twice to immutable variable
[00:42:43] 31
[00:42:43] - error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] -   --> $DIR/liveness-assign-imm-local-notes.rs:23:9
[00:42:43] -    |
[00:42:43] - LL |         x = 2;
[00:42:43] -    |         ----- first assignment to `x`
[00:42:43] - LL |         x = 3;      //~ ERROR (Ast) [E0384]
[00:42:43] -    |         ^^^^^ cannot assign twice to immutable variable
[00:42:43] -
[00:42:43] - error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] -   --> $DIR/liveness-assign-imm-local-notes.rs:35:13
[00:42:43] -    |
[00:42:43] - LL |             x = 2;
[00:42:43] -    |             ----- first assignment to `x`
[00:42:43] - LL |             x = 3;      //~ ERROR (Ast) [E0384]
[00:42:43] -    |             ^^^^^ cannot assign twice to immutable variable
[00:42:43] -
[00:42:43] - error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] + error[E0384]: cannot assign twice to immutable variable `x` (Ast)
[00:42:43] 49   --> $DIR/liveness-assign-imm-local-notes.rs:45:13
[00:42:43] 50    |
[00:42:43] 51 LL |             x = 1;      //~ ERROR (Ast) [E0384]
[00:42:43]
[00:42:43] 52    |             ^^^^^ cannot assign twice to immutable variable
[00:42:43] 53
[00:42:43] - error[E0384]: cannot assign twice to immutable variable `x` (Mir)
[00:42:43] + error[E0384]: cannot assign twice to immutable variable `x` (Ast)
[00:42:43] 55   --> $DIR/liveness-assign-imm-local-notes.rs:48:13
[00:42:43] 56    |
[00:42:43] 57 LL |             x = 1;      //~ ERROR (Ast) [E0384]
[00:42:43]
[00:42:43]
[00:42:43] The actual stderr differed from the expected stderr.
[00:42:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/liveness-assign-imm-local-notes.stderr
[00:42:43] To update references, run this command from build directory:
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lifetime-errors/liveness-assign-imm-local-notes.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/liveness-assign-ight_start":9,"highlight_end":14}],"label":"first assignment to `x`","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/lifetime-errors/liveness-assign-imm-local-notes.rs","byte_start":774,"byte_end":779,"line_start":23,"line_end":23,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        x = 3;      //~ ERROR (Ast) [E0384]","highlight_start":9,"highlight_end":14}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x` (Mir)\n  --> /checkout/src/test/ui/lifetime-errors/liveness-assign-imm-local-notes.rs:23:9\n   |\nLL |         x = 2;\n   |         ----- first assignment to `x`\nLL |         x = 3;      //~ ERROR (Ast) [E0384]\n   |         ^^^^^ cannot assign twice to immutable variable\n\n"}
[00:42:43] {"message":"cannot assign twice to immutable variable `x` (Ast)","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n