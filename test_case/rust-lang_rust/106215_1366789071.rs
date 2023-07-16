plain
test [ui] src/test/ui/typeck/issue-98982.rs ... ok
test [ui] src/test/ui/typeck/point-at-type-param-in-path-expr.rs ... ok
test [ui] src/test/ui/typeck/issue-92481.rs ... ok
test [ui] src/test/ui/typeck/issue-91328.rs ... ok
test [ui] src/test/ui/typeck/quiet-type-err-let-binding.rs ... ok
test [ui] src/test/ui/typeck/type-placeholder-fn-in-const.rs ... ok
test [ui] src/test/ui/typeck/typeck-cast-pointer-to-float.rs ... ok
test [ui] src/test/ui/typeck/struct-enum-wrong-args.rs ... ok
test [ui] src/test/ui/typeck/return_type_containing_closure.rs ... ok
---
---- [ui] src/test/ui/expr-copy.rs stdout ----

error: test run failed!
status: exit status: 101
command: RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr-copy/a"
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr-copy/a", waiting for result
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'client.read_exact(&mut header) failed with Connection reset by peer (os error 104)', src/tools/remote-test-client/src/main.rs:310:9
