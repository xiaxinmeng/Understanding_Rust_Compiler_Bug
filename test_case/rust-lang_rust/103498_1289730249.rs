plain
---- [assembly] src/test/assembly/x86_64-naked-fn-no-cet-prolog.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/x86_64-naked-fn-no-cet-prolog.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-naked-fn-no-cet-prolog/x86_64-naked-fn-no-cet-prolog.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-Zcf-protection=full" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-naked-fn-no-cet-prolog/auxiliary" "--emit=asm"
stdout: none
--- stderr -------------------------------
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


