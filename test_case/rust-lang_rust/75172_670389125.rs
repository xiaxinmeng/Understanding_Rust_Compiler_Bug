console
--- [ui] ui/panic-while-printing.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-while-printing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-while-printing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-while-printing/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0277]: the trait bound `std::vec::Vec<_>: std::io::LocalOutput` is not satisfied
  --> /checkout/src/test/ui/panic-while-printing.rs:19:20
   |
LL |     set_panic(Some(Box::new(Vec::new())));
   |                    ^^^^^^^^^^^^^^^^^^^^ the trait `std::io::LocalOutput` is not implemented for `std::vec::Vec<_>`
   |
   = note: required for the cast to the object type `dyn std::io::LocalOutput`



--- [ui] ui/threads-sendsync/task-stderr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/task-stderr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/task-stderr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/task-stderr/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0277]: the trait bound `Sink: std::io::LocalOutput` is not satisfied
  --> /checkout/src/test/ui/threads-sendsync/task-stderr.rs:24:28
   |
LL |         io::set_panic(Some(Box::new(sink)));
   |                            ^^^^^^^^^^^^^^ the trait `std::io::LocalOutput` is not implemented for `Sink`
   |
   = note: required for the cast to the object type `dyn std::io::LocalOutput`
