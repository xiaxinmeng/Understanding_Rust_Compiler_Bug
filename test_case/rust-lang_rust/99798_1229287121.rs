plain
failures:

---- [incremental] src/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-85031-2.rs stdout ----

error in revision `cfail`: incremental test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-85031-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-85031-2/issue-85031-2.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-85031-2" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-85031-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function cannot return without recursing
   |
   |
LL |     pub fn foo<const A: usize>() -> [(); A - 0] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
LL |         Self::foo()
   |         ----------- recursive call site
   = note: `#[warn(unconditional_recursion)]` on by default
   = note: `#[warn(unconditional_recursion)]` on by default
   = help: a `loop` may express intention better if this is on purpose
warning: 1 warning emitted
------------------------------------------


