

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0161]: cannot move a value of type str: the size of str cannot be statically determined
  --> /checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs:10:18
   |
LL |     let x = vec![*"1"];
   |                  ^^^^

error: aborting due to previous error
