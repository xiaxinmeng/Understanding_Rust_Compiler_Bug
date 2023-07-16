plain
...........................iii.......................................................... 13112/13158
..............................................
failures:

---- [ui] src/test/ui/panics/drop_in_panic_payload_does_not_unwind.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/drop_in_panic_payload_does_not_unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/drop_in_panic_payload_does_not_unwind/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/drop_in_panic_payload_does_not_unwind/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `just_me` in this scope
  --> /checkout/src/test/ui/panics/drop_in_panic_payload_does_not_unwind.rs:29:31
   |
LL |         process::Command::new(just_me)

error[E0599]: no method named `not` found for type `bool` in the current scope
  --> /checkout/src/test/ui/panics/drop_in_panic_payload_does_not_unwind.rs:34:30
   |
   |
LL |     assert!(status.success().not(), "the `exit(0)` ought to be unreachable");
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
  ::: /checkout/library/core/src/ops/bit.rs:51:8
   |
