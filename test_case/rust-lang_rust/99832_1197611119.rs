plain
---- [ui] src/test/ui/nll/issue-52057.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52057.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52057/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52057/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0311]: the parameter type `I` may not live long enough
   |
note: the parameter type `I` must be valid for the anonymous lifetime defined here...
   |
LL |     fn parse_first(_: &mut Self::Input) {}
   |                       ^^^^^^^^^^^^^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: ...so that the type `I` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
LL | impl<'a, I: 'b, P: ?Sized> Parser for &'a mut P

error: aborting due to previous error
------------------------------------------

