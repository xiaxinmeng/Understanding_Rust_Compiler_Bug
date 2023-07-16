plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:22] 
[00:46:22] running 2197 tests
[00:46:25] .............F......................................................................................
[00:46:33] ....................................................................................................
[00:46:35] ....................................................................................................
[00:46:37] ....................................................................................................
[00:46:39] ....................................................................................................
---
[00:47:13] ...................................................................................i................
[00:47:17] ....................................................................................................
[00:47:20] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:27] .....iF.....................................................................................i....
[00:47:27] 
[00:47:27] ---- [ui] ui/asm-out-assign-imm.rs stdout ----
[00:47:27] diff of stderr:
[00:47:27] 
[00:47:27] 
[00:47:27] 1 error[E0384]: cannot assign twice to immutable variable `x`
[00:47:27] +   --> $DIR/asm-out-assign-imm.rs:34:9
[00:47:27] 3    |
[00:47:27] 3    |
[00:47:27] 4 LL |     x = 1;
[00:47:27] 5    |     ----- first assignment to `x`
[00:47:27] 
[00:47:27] The actual stderr differed from the expected stderr.
[00:47:27] The actual stderr differed from the expected stderr.
[00:47:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/asm-out-assign-imm.stderr
[00:47:27] To update references, rerun the tests and pass the `--bless` flag
[00:47:27] To only update this specific test, also pass `--test-args asm-out-assign-imm.rs`
[00:47:27] error: 1 errors occurred comparing output.
[00:47:27] status: exit code: 1
[00:47:27] status: exit code: 1
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm-out-assign-imm.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/auxiliary" "-A" "unused"
[00:47:27] ------------------------------------------
[00:47:27] 
[00:47:27] ------------------------------------------
[00:47:27] stderr:
[00:47:27] stderr:
[00:47:27] ------------------------------------------
[00:47:27] {"message":"cannot assign twice to immutable variable `x`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n