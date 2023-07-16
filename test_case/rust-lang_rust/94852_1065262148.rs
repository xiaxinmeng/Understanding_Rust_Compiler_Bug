plain
---- [ui] ui/svh-add-nothing.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-add-nothing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-add-nothing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-add-nothing/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0460]: found possibly newer version of crate `a` which `b` depends on
   |
LL | extern crate b;
   | ^^^^^^^^^^^^^^^
   |
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-add-nothing/auxiliary/liba.so
           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-add-nothing/auxiliary/libb.so
error: aborting due to previous error
------------------------------------------


