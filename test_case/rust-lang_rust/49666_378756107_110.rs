\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/liveness-assign-imm-local-notes.rs","byte_start":1288,"byte_end":1293,"line_start":48,"line_end":48,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"            x = 2;      //~ ERROR (Ast) [E0384]","highlight_start":13,"highlight_end":18}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/lifetime-errors/liveness-assign-imm-local-notes.rs","byte_start":1174,"byte_end":1179,"line_start":45,"line_end":45,"column_start":13,"column_end":18,"is_primary":false,"text":[{"text":"            x = 1;      //~ ERROR (Ast) [E0384]","highlight_start":13,"highlight_end":18}],"label":"first assignment to `x`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x` (Ast)\n  --> /checkout/src/test/ui/lifetime-errors/liveness-assign-imm-local-notes.rs:48:13\n   |\nLL |             x = 1;      //~ ERROR (Ast) [E0384]\n   |             ----- first assignment to `x`\n...\nLL |             x = 2;      //~ ERROR (Ast) [E0384]\n   |             ^^^^^ cannot assign twice to immutable variable\n\n"}
[00:42:43] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:42:43] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
---
[00:42:43] - error[E0382]: use of moved value: `x` (Ast)
[00:42:43] + error[E0382]: use of moved value: `x` (Mir)
[00:42:43] 2   --> $DIR/moves-based-on-type-tuple.rs:16:13
[00:42:43] 3    |
[00:42:43] 4 LL |     box (x, x)
[00:42:43]
[00:42:43] 8    |
[00:42:43] 9    = note: move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
[00:42:43] 10
[00:42:43] - error[E0382]: use of moved value: `x` (Mir)
[00:42:43] + error[E0382]: use of moved value: `x` (Ast)
[00:42:43] 12   --> $DIR/moves-based-on-type-tuple.rs:16:13
[00:42:43] 13    |
[00:42:43] 14 LL |     box (x, x)
[00:42:43]
[00:42:43]
[00:42:43] The actual stderr differed from the expected stderr.
[00:42:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves-based-on-type-tuple.stderr
[00:42:43] To update references, run this command from build directory:
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'moves-based-on-type-tuple.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves-based-on-type-tuple.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves-based-on-type-tuple.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "emit-end-regions" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves-based-on-type-tuple.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:43] {"message":"use of moved value: `x` (Mir)","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n