\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types-ICE-when-projecting-out-of-err.rs","byte_start":850,"byte_end":851,"line_start":33,"line_end":33,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    r = r + a;","highlight_start":11,"highlight_end":12}],"label":"the trait `Add<A>` is not implemented for `()`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0277]: the trait bound `(): Add<A>` is not satisfied\n  --> /checkout/src/test/ui/associated-types-ICE-when-projecting-out-of-err.rs:33:11: in fn ice\n   |\nLL |     r = r + a;\n   |           ^ the trait `Add<A>` is not implemented for `()`\n\n"}
[00:48:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/associated-types-ICE-when-projecting-out-of-err.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/asm-out-assign-imm.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0384]: cannot assign twice to immutable variable `x`
[00:48:30] -   --> $DIR/asm-out-assign-imm.rs:30:9
[00:48:30] +   --> $DIR/asm-out-assign-imm.rs:30:9: in fn main
[00:48:30] 3    |
[00:48:30] 4 LL |     x = 1;
[00:48:30] 5    |     ----- first assignment to `x`
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'asm-out-assign-imm.rs'
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm-out-assign-imm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"cannot assign twice to immutable variable `x`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n