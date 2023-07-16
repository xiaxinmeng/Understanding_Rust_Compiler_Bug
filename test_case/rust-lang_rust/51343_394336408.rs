plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:27] 
[00:49:27] running 1486 tests
[00:49:31] ...........F...............................................................................i........
[00:49:42] ....................................................................................................
[00:49:45] ....................................................................................................
[00:49:49] ....................................................................................................
[00:49:52] ....................................................................................................
---
[00:50:20] ...........................................................................i........................
[00:50:25] ....................................................................................................
[00:50:32] ....................................................................................................
[00:50:38] ....................................................................................................
[00:50:43] ........iF................iiiiiiiii...................................................
[00:50:43] 
[00:50:43] ---- [ui] ui/asm-out-assign-imm.rs stdout ----
[00:50:43] diff of stderr:
[00:50:43] 
[00:50:43] 
[00:50:43] 1 error[E0384]: cannot assign twice to immutable variable `x`
[00:50:43] -   --> $DIR/asm-out-assign-imm.rs:30:9
[00:50:43] +   --> $DIR/asm-out-assign-imm.rs:31:9
[00:50:43] 3    |
[00:50:43] 4 LL |     x = 1;
[00:50:43] 5    |     ----- first assignment to `x`
[00:50:43] 
[00:50:43] The actual stderr differed from the expected stderr.
[00:50:43] The actual stderr differed from the expected stderr.
[00:50:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/asm-out-assign-imm.stderr
[00:50:43] To update references, rerun the tests and pass the `--bless` flag
[00:50:43] To only update this specific test, also pass `--test-args asm-out-assign-imm.rs`
[00:50:43] error: 1 errors occurred comparing output.
[00:50:43] status: exit code: 101
[00:50:43] status: exit code: 101
[00:50:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm-out-assign-imm.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/auxiliary" "-A" "unused"
[00:50:43] ------------------------------------------
[00:50:43] 
[00:50:43] ------------------------------------------
[00:50:43] stderr:
[00:50:43] stderr:
[00:50:43] ------------------------------------------
[00:50:43] {"message":"cannot assign twice to immutable variable `x`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n